#![no_main]
#![no_std]

use core::fmt::Write;

use panic_halt as _;

use embedded_hal::adc::OneShot;

use microbit::hal::delay::Delay;
use microbit::hal::gpio::{gpio::PIN3, Floating, Input};
use microbit::hal::prelude::*;
use microbit::hal::serial::BAUD9600;
use microbit::hal::{adc, serial};
use microbit::UART0;

use cast::f32;

use rtfm::app;

#[app(device = microbit::hal::nrf51, peripherals = true)]
const APP: () = {
    struct Resources {
        delay: Delay,
        adc: adc::Adc,
        analog_pin: PIN3<Input<Floating>>,
        tx: serial::Tx<UART0>,
        rx: serial::Rx<UART0>,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        let dp = ctx.device;
        let gpio = dp.GPIO.split();

        /* Set up pin3 to read analog input */
        let analog_pin = gpio.pin3;
        let mut adc = adc::Adc::default(dp.ADC);
        adc.set_input_selection(adc::AdcInputSelection::AnalogInputNoPrescaling);
        adc.set_reference_selection(adc::AdcReferenceSelection::VBG);

        /* Set up pin 1 and 2 for bluetooth serial */
        let tx = gpio.pin2.into_push_pull_output().into();
        let rx = gpio.pin1.into_floating_input().into();
        let uart0 = dp.UART0;
        uart0.intenset.write(|w| w.rxdrdy().set_bit());
        uart0.events_rxdrdy.reset();
        let (mut tx, rx) = serial::Serial::uart0(uart0, tx, rx, BAUD9600).split();

        write!(tx, "Temperature Sensor started.\n").unwrap();

        let delay = Delay::new(dp.TIMER0);

        init::LateResources {
            delay,
            adc,
            analog_pin,
            tx,
            rx,
        }
    }

    #[task(binds = UART0, resources = [adc, analog_pin, tx, rx])]
    fn update_request(ctx: update_request::Context) {
        match ctx.resources.rx.read() {
            // T
            Ok(0x54_u8) => {
                let sample: u32 = ctx.resources.adc.read(ctx.resources.analog_pin).unwrap();
                // Equivalent of (sample/1023 * 1.2 - 0.5) / 0.01
                let temp = f32(sample * 120 - 51150) / 1023.0;
                write!(ctx.resources.tx, "T{}\r\n", temp).unwrap();
            }
            _ => {}
        }
    }
};
