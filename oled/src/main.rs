#![no_main]
#![no_std]

mod font24x32;

use panic_halt as _;
//use panic_semihosting as _;

use cortex_m_rt::entry;

use embedded_hal::adc::OneShot;

use microbit::hal::adc;
use microbit::hal::delay::Delay;
use microbit::hal::i2c;
use microbit::hal::prelude::*;

use ssd1306::{mode::GraphicsMode, Builder};

use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;

use cast::f32;

use core::fmt::Write;
use heapless::consts::U10;
use heapless::String;

use crate::font24x32::Font24x32;

#[entry]
fn main() -> ! {
    let dp = microbit::Peripherals::take().unwrap();
    let mut delay = Delay::new(dp.TIMER0);

    let gpio = dp.GPIO.split();

    let mut pin3 = gpio.pin3;
    let mut adc = adc::Adc::default(dp.ADC);
    adc.set_input_selection(adc::AdcInputSelection::AnalogInputNoPrescaling);
    adc.set_reference_selection(adc::AdcReferenceSelection::VBG);

    /* Configure SCL and SDA pins accordingly */
    let scl = gpio.pin0.into_open_drain_input().into();
    let sda = gpio.pin30.into_open_drain_input().into();

    /* Set up I2C */
    let i2c = i2c::I2c::i2c1(dp.TWI1, sda, scl);

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    disp.init().unwrap();
    disp.flush().unwrap();

    let mut buf = String::<U10>::new();

    loop {
        disp.display_on(true).unwrap();
        disp.clear();
        let sample: u32 = adc.read(&mut pin3).unwrap();
        // Equivalent of (sample/1023 * 1.2 - 0.5) / 0.01
        let temp = f32(sample * 120 - 51150) / 1023.0;

        write!(&mut buf, " {:.1}", temp).unwrap();

        disp.draw(Font24x32::render_str(&buf).into_iter());
        disp.flush().unwrap();
        delay.delay_ms(1_000_u16);

        disp.display_on(false).unwrap();
        delay.delay_ms(3_000_u16);
    }
}
