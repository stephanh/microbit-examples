#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;
use microbit::led::Display;

#[entry]
fn main() -> ! {
    let dp = microbit::Peripherals::take().unwrap();

    let timer0 = dp.TIMER0;
    let mut delay = Delay::new(timer0);

    let gpio = dp.GPIO.split();
    let pin4 = gpio.pin4.into_push_pull_output();
    let pin5 = gpio.pin5.into_push_pull_output();
    let pin6 = gpio.pin6.into_push_pull_output();
    let pin7 = gpio.pin7.into_push_pull_output();
    let pin8 = gpio.pin8.into_push_pull_output();
    let pin9 = gpio.pin9.into_push_pull_output();
    let pin10 = gpio.pin10.into_push_pull_output();
    let pin11 = gpio.pin11.into_push_pull_output();
    let pin12 = gpio.pin12.into_push_pull_output();
    let pin13 = gpio.pin13.into_push_pull_output();
    let pin14 = gpio.pin14.into_push_pull_output();
    let pin15 = gpio.pin15.into_push_pull_output();

    let mut display = Display::new(
        pin4, pin5, pin6, pin7, pin8, pin9, pin10, pin11, pin12, pin13, pin14, pin15,
    );

    let letter_s = [
        [1, 1, 1, 1, 1],
        [1, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 1, 1],
        [1, 1, 1, 1, 1],
    ];

    let letter_t = [
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
        [0, 1, 1, 1, 0],
        [0, 1, 1, 1, 0],
    ];

    let letter_e = [
        [1, 1, 1, 1, 1],
        [1, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
        [1, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ];

    let letter_p = [
        [1, 1, 1, 1, 1],
        [1, 1, 0, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 0, 0, 0],
        [1, 1, 0, 0, 0],
    ];

    let letter_h = [
        [1, 1, 0, 1, 1],
        [1, 1, 0, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 0, 1, 1],
        [1, 1, 0, 1, 1],
    ];

    let letter_a = [
        [1, 1, 1, 1, 1],
        [1, 1, 0, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 0, 1, 1],
        [1, 1, 0, 1, 1],
    ];

    let letter_n = [
        [1, 0, 0, 0, 1],
        [1, 1, 0, 0, 1],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 1, 1],
        [1, 0, 0, 0, 1],
    ];

    let name = [
        letter_s, letter_t, letter_e, letter_p, letter_h, letter_a, letter_n,
    ];

    loop {
        for l in name.iter() {
            display.display(&mut delay, *l, 1000);
        }
    }
}
