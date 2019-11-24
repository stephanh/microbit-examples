#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;

use panic_halt as _;

use cortex_m_rt::entry;

use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;

#[entry]
fn main() -> ! {
    let dp = microbit::Peripherals::take().unwrap();

    let gpio = dp.GPIO.split();
    let tx = gpio.pin24.into_push_pull_output().into();
    let rx = gpio.pin25.into_floating_input().into();

    let (mut tx, _) = serial::Serial::uart0(dp.UART0, tx, rx, BAUD115200).split();

    let _ = write!(tx, "Hello World from serial\n\r");

    loop {
        continue;
    }
}
