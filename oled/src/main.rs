#![no_main]
#![no_std]

mod font24x32;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;

use panic_halt as _;
//use panic_semihosting as _;

use cortex_m::interrupt::Mutex;

use cortex_m_rt::entry;

use embedded_hal::adc::OneShot;

use microbit::hal::delay::Delay;
use microbit::hal::gpio::{gpio::PIN3, Floating, Input};
use microbit::hal::prelude::*;
use microbit::hal::{adc, i2c};
use microbit::interrupt;
use microbit::{GPIOTE, NVIC, TWI1};

use ssd1306::{prelude::*, Builder};

use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;

use cast::f32;

use heapless::consts::U10;
use heapless::String;

use crate::font24x32::Font24x32;

static GPIOTE: Mutex<RefCell<Option<GPIOTE>>> = Mutex::new(RefCell::new(None));
static DELAY: Mutex<RefCell<Option<Delay>>> = Mutex::new(RefCell::new(None));
static DISPLAY: Mutex<RefCell<Option<GraphicsMode<I2cInterface<i2c::I2c<TWI1>>>>>> =
    Mutex::new(RefCell::new(None));
static ADC: Mutex<RefCell<Option<adc::Adc>>> = Mutex::new(RefCell::new(None));
static PIN: Mutex<RefCell<Option<PIN3<Input<Floating>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = microbit::Peripherals::take().unwrap();

    cortex_m::interrupt::free(move |cs| {
        /* Enable external GPIO interrupts */
        unsafe { NVIC::unmask(microbit::Interrupt::GPIOTE) };
        NVIC::unpend(microbit::Interrupt::GPIOTE);

        let gpio = dp.GPIO.split();

        let pin3 = gpio.pin3;
        let mut adc = adc::Adc::default(dp.ADC);
        adc.set_input_selection(adc::AdcInputSelection::AnalogInputNoPrescaling);
        adc.set_reference_selection(adc::AdcReferenceSelection::VBG);

        /* Configure SCL and SDA pins accordingly */
        let scl = gpio.pin0.into_open_drain_input().into();
        let sda = gpio.pin30.into_open_drain_input().into();

        /* Set up I2C */
        let i2c = i2c::I2c::i2c1(dp.TWI1, sda, scl);

        let mut disp: GraphicsMode<I2cInterface<i2c::I2c<TWI1>>> = Builder::new()
            .with_rotation(DisplayRotation::Rotate180)
            .connect_i2c(i2c)
            .into();
        disp.init().unwrap();
        disp.flush().unwrap();

        let _ = gpio.pin17.into_floating_input();
        /* Set up GPIO 17 (button A) to generate an interrupt when pulled down */
        dp.GPIOTE.config[0]
            .write(|w| unsafe { w.mode().event().psel().bits(17).polarity().hi_to_lo() });
        dp.GPIOTE.intenset.write(|w| w.in0().set_bit());
        dp.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });

        *GPIOTE.borrow(cs).borrow_mut() = Some(dp.GPIOTE);
        *DELAY.borrow(cs).borrow_mut() = Some(Delay::new(dp.TIMER0));
        *DISPLAY.borrow(cs).borrow_mut() = Some(disp);
        *ADC.borrow(cs).borrow_mut() = Some(adc);
        *PIN.borrow(cs).borrow_mut() = Some(pin3);
    });

    loop {
        continue;
    }
}

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        if let (
            Some(gpiote),
            Some(ref mut disp),
            Some(ref mut delay),
            Some(ref mut adc),
            Some(ref mut pin3),
        ) = (
            GPIOTE.borrow(cs).borrow().as_ref(),
            DISPLAY.borrow(cs).borrow_mut().deref_mut(),
            DELAY.borrow(cs).borrow_mut().deref_mut(),
            ADC.borrow(cs).borrow_mut().deref_mut(),
            PIN.borrow(cs).borrow_mut().deref_mut(),
        ) {
            let mut buf = String::<U10>::new();

            disp.display_on(true).unwrap();
            disp.clear();
            let sample: u32 = adc.read(pin3).unwrap();
            // Equivalent of (sample/1023 * 1.2 - 0.5) / 0.01
            let temp = f32(sample * 120 - 51150) / 1023.0;

            write!(&mut buf, " {:.1}", temp).unwrap();

            disp.draw(
                Font24x32::render_str(&buf)
                    .translate(Point::new(0, 15))
                    .into_iter(),
            );
            disp.flush().unwrap();

            delay.delay_ms(1_500_u16);
            disp.display_on(false).unwrap();

            /* Clear events */
            gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
        }
    });
}
