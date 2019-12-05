#![no_main]
#![no_std]

mod font24x32;

use core::fmt::Write;

use panic_halt as _;
//use panic_semihosting as _;

use embedded_hal::adc::OneShot;

use microbit::hal::delay::Delay;
use microbit::hal::gpio::{gpio::PIN3, Floating, Input};
use microbit::hal::prelude::*;
use microbit::hal::{adc, i2c};
use microbit::{GPIOTE, TWI1};

use ssd1306::{prelude::*, Builder};

use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;

use cast::f32;

use heapless::consts::U10;
use heapless::String;

use crate::font24x32::Font24x32;

use rtfm::app;

#[app(device = microbit::hal::nrf51, peripherals = true)]
const APP: () = {
    struct Resources {
        delay: Delay,
        adc: adc::Adc,
        analog_pin: PIN3<Input<Floating>>,
        display: GraphicsMode<I2cInterface<i2c::I2c<TWI1>>>,
        gpiote: GPIOTE,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        let dp = ctx.device;
        let gpio = dp.GPIO.split();

        /* Set up pin3 to read analog input */
        let analog_pin = gpio.pin3;
        let mut adc = adc::Adc::default(dp.ADC);
        adc.set_input_selection(adc::AdcInputSelection::AnalogInputNoPrescaling);
        adc.set_reference_selection(adc::AdcReferenceSelection::VBG);

        /* Configure SCL and SDA pins accordingly */
        let scl = gpio.pin0.into_open_drain_input().into();
        let sda = gpio.pin30.into_open_drain_input().into();

        /* Set up I2C */
        let i2c = i2c::I2c::i2c1(dp.TWI1, sda, scl);

        let mut display: GraphicsMode<I2cInterface<i2c::I2c<TWI1>>> = Builder::new()
            .with_rotation(DisplayRotation::Rotate180)
            .connect_i2c(i2c)
            .into();
        display.init().unwrap();
        display.flush().unwrap();

        /* Set up button a trigger interrupt */
        let _ = gpio.pin17.into_floating_input();
        /* Set up GPIO 17 (button A) to generate an interrupt when pulled down */
        let gpiote = dp.GPIOTE;
        gpiote.config[0]
            .write(|w| unsafe { w.mode().event().psel().bits(17).polarity().hi_to_lo() });
        gpiote.intenset.write(|w| w.in0().set_bit());
        gpiote.events_in[0].write(|w| unsafe { w.bits(0) });

        let delay = Delay::new(dp.TIMER0);

        init::LateResources {
            delay,
            adc,
            analog_pin,
            display,
            gpiote,
        }
    }

    #[task(binds = GPIOTE, resources = [delay, adc, analog_pin, display, gpiote])]
    fn button_a_push(ctx: button_a_push::Context) {
        let mut buf = String::<U10>::new();

        let display = ctx.resources.display;

        display.display_on(true).unwrap();
        display.clear();
        let sample: u32 = ctx.resources.adc.read(ctx.resources.analog_pin).unwrap();
        // Equivalent of (sample/1023 * 1.2 - 0.5) / 0.01
        let temp = f32(sample * 120 - 51150) / 1023.0;

        write!(&mut buf, " {:.1}", temp).unwrap();

        display.draw(
            Font24x32::render_str(&buf)
                .translate(Point::new(0, 15))
                .into_iter(),
        );
        display.flush().unwrap();

        ctx.resources.delay.delay_ms(1_500_u16);
        display.display_on(false).unwrap();
        ctx.resources.gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
    }
};
