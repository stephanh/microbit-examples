#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;

use panic_halt as _;

use cortex_m_rt::entry;

use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD1200;

use nb;

use cortex_m_semihosting::hprint;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let dp = microbit::Peripherals::take().unwrap();

    let gpio = dp.GPIO.split();
    //PIN1
    let tx = gpio.pin2.into_push_pull_output().downgrade();
    //PIN2
    let rx = gpio.pin1.into_floating_input().downgrade();

    let timer0 = dp.TIMER0;
    let mut delay = Delay::new(timer0);

    //delay.delay_ms(1_500_u16);
    let (mut tx, mut rx) = serial::Serial::uart0(dp.UART0, tx, rx, BAUD1200).split();
    //delay.delay_ms(1_500_u16);

    write!(tx, "AT+VERSION\r\n").unwrap();
    /*nb::block!(tx.write(b'A')).ok();
    nb::block!(tx.write(b'T')).ok();
    nb::block!(tx.write(b'\r')).ok();
    nb::block!(tx.write(b'\n')).ok();
    nb::block!(tx.flush()).ok();*/
    hprintln!("Message sent").unwrap();
    //write!(tx, "THis is a new test. I want to see how well this works.").unwrap();

    loop {
        if let Ok(c) = nb::block!(rx.read()) {
            hprint!("{}", c as char).unwrap();
            //nb::block!(tx.write(c)).ok();
        }
        hprintln!("Looping").unwrap();
    }

    //let s = b"Please type characters to echo:\r\n";

    //let _ = s.into_iter().map(|c| nb::block!(tx.write(*c))).last();

    /* Endless loop */
    /*loop {
        /* Read and echo back */
        if let Ok(c) = nb::block!(rx.read()) {
            let _ = nb::block!(tx.write(c));
        }
    }*/
}
