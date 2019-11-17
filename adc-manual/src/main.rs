#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;

use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    let dp = microbit::Peripherals::take().unwrap();
    let mut delay = Delay::new(dp.TIMER0);
    let adc = dp.ADC;

    hprintln!("Start").unwrap();
    while adc.busy.read().busy().is_busy() {
        hprintln!("ADC busy").unwrap();
        delay.delay_ms(100_u16);
    }

    hprintln!("Configuring ADC").unwrap();
    adc.config.write(|w| {
        w.res()
            ._10bit()
            .inpsel()
            .analog_input_one_third_prescaling()
            .refsel()
            .supply_one_third_prescaling()
            .psel()
            .analog_input4()
    });
    adc.enable.write(|w| w.enable().enabled());

    hprintln!("ADC configured").unwrap();

    loop {
        if adc.busy.read().busy().is_ready() {
            hprintln!("Starting ADC").unwrap();
            adc.events_end.write(|w| unsafe { w.bits(0) });
            adc.tasks_start.write(|w| unsafe { w.bits(1) });
            hprintln!("ADC started").unwrap();

            while adc.events_end.read().bits() == 0 {}

            adc.events_end.write(|w| unsafe { w.bits(0) });
            let result: u16 = adc.result.read().result().bits();
            hprintln!("ADC got results {}", result).unwrap();
        }

        delay.delay_ms(5_000_u16);
    }
}
