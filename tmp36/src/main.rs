#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;

use embedded_hal::adc::OneShot;

use microbit::hal::adc;
use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;

use cast::f32;

#[entry]
fn main() -> ! {
    let dp = microbit::Peripherals::take().unwrap();
    let mut pin3 = dp.GPIO.split().pin3;

    let mut delay = Delay::new(dp.TIMER0);

    let mut adc = adc::Adc::default(dp.ADC);
    adc.set_input_selection(adc::AdcInputSelection::AnalogInputNoPrescaling);
    adc.set_reference_selection(adc::AdcReferenceSelection::VBG);

    loop {
        let sample: u32 = adc.read(&mut pin3).unwrap();
        // Equivalent of (sample/1023 * 1.2 - 0.5) / 0.01
        let temp = f32(sample * 120 - 51150) / 1023.0;

        hprintln!("ADC: {}, T: {}", sample, temp).unwrap();
        delay.delay_ms(3_000_u16);
    }
}
