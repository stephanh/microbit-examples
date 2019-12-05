#![no_main]
#![no_std]

pub mod tm1637;

use panic_halt as _;

use cortex_m_rt::entry;

//use cortex_m_semihosting::hprintln;

use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;

use tm1637::TM1637;

#[entry]
fn main() -> ! {
    let dp = microbit::Peripherals::take().unwrap();
    let gpio = dp.GPIO.split();
    let mut dio = gpio.pin3.into_open_drain_output();
    let mut clk = gpio.pin2.into_open_drain_output();

    let mut delay = Delay::new(dp.TIMER0);
    test(dio, |p| p.into_open_drain_output());

    /*let mut display = TM1637::new(
        &mut delay,
        &mut clk,
        &mut dio,
        |&mut p| p.into_floating_input(),
        |p| p.into_open_drain_output(),
    );*/

    /*display.init().unwrap();
    display.clear().unwrap();
    loop {
        for i in 0..255 {
            display.print_hex(0, &[i, i + 1]).unwrap();

            display.print_raw(3, &[i]).unwrap();

            display.set_brightness(i >> 5).unwrap();
        }*/

    loop {}
}

use embedded_hal::digital::v2::OutputPin;
fn test<A, B>(a: A, f: fn(A) -> B) -> B
where
    B: OutputPin,
{
    f(a)
}

use core::borrow::BorrowMut;
use core::cell::RefCell;
use microbit::hal::gpio::{gpio, Floating, Input, OpenDrain, Output};
fn test2(a: gpio::PIN3<Output<OpenDrain>>) {
    a.into_floating_input();
}
