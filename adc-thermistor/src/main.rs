#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;

use embedded_hal::adc::OneShot;

use microbit::hal::adc::Adc;
use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;

use cast::f32;

use micromath::F32Ext;

const SERIES_R: f32 = 5420.0;
const B_COEFFICIENT: f32 = 3950.0;
const R_25: f32 = 5000.0;
const K_OFFSET: f32 = 273.15;
const T_25: f32 = 25.0 + K_OFFSET;

#[entry]
fn main() -> ! {
    let dp = microbit::Peripherals::take().unwrap();
    let mut pin3 = dp.GPIO.split().pin3;

    let mut delay = Delay::new(dp.TIMER0);

    let mut adc = Adc::default(dp.ADC);

    loop {
        let mut avg: u16 = 0;
        for _ in 0..5 {
            let sample: u16 = adc.read(&mut pin3).unwrap();
            avg += sample;
            delay.delay_ms(100_u16);
        }

        let sample = avg / 5;
        let r = SERIES_R * (1023.0 / f32(sample) - 1.0);
        let temp_k = 1.0 / (1.0 / T_25 + 1.0 / B_COEFFICIENT * (r / R_25).ln());
        let temp_c = temp_k - K_OFFSET;
        hprintln!("ADC: {}, R: {}, T: {}", sample, r, temp_c).unwrap();

        delay.delay_ms(3_000_u16);
    }
}
