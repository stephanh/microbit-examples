#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;

use panic_halt as _;

use cortex_m_rt::entry;

use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD9600;

use nb;

#[entry]
fn main() -> ! {
    let dp = microbit::Peripherals::take().unwrap();

    let gpio = dp.GPIO.split();
    //PIN1
    let tx = gpio.pin2.into_push_pull_output().into();
    //PIN2
    let rx = gpio.pin1.into_floating_input().into();

    let (mut tx, mut rx) = serial::Serial::uart0(dp.UART0, tx, rx, BAUD9600).split();

    write!(tx, "Echo server started.").unwrap();

    loop {
        if let Ok(c) = nb::block!(rx.read()) {
            write!(tx, "{}", c as char).unwrap();
        }
    }
}
