//#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::cell::RefCell;
use core::ops::DerefMut;

use panic_halt as _;

use cortex_m::interrupt::Mutex;

use cortex_m_rt::entry;

use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;
use microbit::hal::temp::Temp;
use microbit::interrupt;
use microbit::led::Display;
use microbit::{GPIOTE, NVIC};

use cast::i32;

static GPIOTE: Mutex<RefCell<Option<GPIOTE>>> = Mutex::new(RefCell::new(None));
static DELAY: Mutex<RefCell<Option<Delay>>> = Mutex::new(RefCell::new(None));
static DISPLAY: Mutex<RefCell<Option<Display>>> = Mutex::new(RefCell::new(None));
static TEMP: Mutex<RefCell<Option<Temp>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = microbit::Peripherals::take().unwrap();

    cortex_m::interrupt::free(move |cs| {
        /* Enable external GPIO interrupts */
        unsafe { NVIC::unmask(microbit::Interrupt::GPIOTE) };
        NVIC::unpend(microbit::Interrupt::GPIOTE);

        let temp = Temp::new(dp.TEMP);

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

        let display = Display::new(
            pin4, pin5, pin6, pin7, pin8, pin9, pin10, pin11, pin12, pin13, pin14, pin15,
        );

        let _ = gpio.pin17.into_floating_input();
        /* Set up GPIO 17 (button A) to generate an interrupt when pulled down */
        dp.GPIOTE.config[0]
            .write(|w| unsafe { w.mode().event().psel().bits(17).polarity().hi_to_lo() });
        dp.GPIOTE.intenset.write(|w| w.in0().set_bit());
        dp.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });

        *GPIOTE.borrow(cs).borrow_mut() = Some(dp.GPIOTE);
        *DELAY.borrow(cs).borrow_mut() = Some(Delay::new(dp.TIMER0));
        *DISPLAY.borrow(cs).borrow_mut() = Some(display);
        *TEMP.borrow(cs).borrow_mut() = Some(temp);
    });

    loop {
        continue;
    }
}

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        if let (Some(gpiote), Some(ref mut display), Some(ref mut delay), Some(ref mut temp)) = (
            GPIOTE.borrow(cs).borrow().as_ref(),
            DISPLAY.borrow(cs).borrow_mut().deref_mut(),
            DELAY.borrow(cs).borrow_mut().deref_mut(),
            TEMP.borrow(cs).borrow_mut().deref_mut(),
        ) {
            let m = temp.measure();
            let t = i32(m);

            if t < 0 || t > 100 {
                display.display(delay, display_number(t), 1000);
            }

            let first_digit = t / 10;
            let second_digit = t % 10;

            if first_digit != 0 {
                display.display(delay, display_number(first_digit), 750);
            }

            display.display(delay, display_number(second_digit), 750);

            /* Clear events */
            gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
        }
    });
}

fn display_number(n: i32) -> [[u8; 5]; 5] {
    match n {
        0 => [
            [1, 1, 1, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 1, 1, 1],
        ],
        1 => [
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1],
        ],
        2 => [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ],
        3 => [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 1],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 1],
            [1, 1, 1, 1, 1],
        ],
        4 => [
            [1, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1],
        ],
        5 => [
            [1, 1, 1, 1, 1],
            [1, 1, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 1],
            [1, 1, 1, 1, 1],
        ],
        6 => [
            [1, 1, 0, 0, 0],
            [1, 1, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 1, 1, 1],
        ],
        7 => [
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1],
        ],
        8 => [
            [1, 1, 1, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 1, 1, 1],
        ],
        9 => [
            [1, 1, 1, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1],
        ],
        _ => [
            // E
            [1, 1, 1, 1, 1],
            [1, 1, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [1, 1, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ],
    }
}
