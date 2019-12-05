use embedded_hal as hal;

use hal::blocking::delay::DelayUs;
use hal::digital::v2::{InputPin, OutputPin};
//use hal::prelude::*;

#[derive(Debug)]
pub enum Error {
    Ack,
}

pub struct TM1637<'a, D, CLK, DI, DO> {
    delay: &'a mut D,
    clk: &'a mut CLK,
    dio: DO,
    dio_to_input: fn(DO) -> DI,
    dio_to_output: fn(DI) -> DO,
}

impl<'a, D, CLK, DI, DO> TM1637<'a, D, CLK, DI, DO>
where
    D: DelayUs<u16>,
    CLK: OutputPin,
    <CLK as embedded_hal::digital::v2::OutputPin>::Error: core::fmt::Debug,
    DI: InputPin,
    DO: OutputPin,
    <DO as embedded_hal::digital::v2::OutputPin>::Error: core::fmt::Debug,
{
    pub fn new(
        delay: &'a mut D,
        clk: &'a mut CLK,
        dio: DO,
        dio_to_input: fn(DO) -> DI,
        dio_to_output: fn(DI) -> DO,
    ) -> Self {
        Self {
            delay,
            clk,
            dio,
            dio_to_input,
            dio_to_output,
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.start();
        self.send(ADDRESS_AUTO_INCREMENT_1_MODE)?;
        self.stop();
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        self.print_raw_iter(0, core::iter::repeat(0).take(4))
    }

    pub fn print_raw(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error> {
        self.print_raw_iter(address, bytes.iter().map(|b| *b))
    }

    pub fn print_hex(&mut self, address: u8, digits: &[u8]) -> Result<(), Error> {
        self.print_raw_iter(
            address,
            digits.iter().map(|digit| DIGITS[(digit & 0xf) as usize]),
        )
    }

    pub fn print_raw_iter<Iter: Iterator<Item = u8>>(
        &mut self,
        address: u8,
        bytes: Iter,
    ) -> Result<(), Error> {
        self.start();
        self.send(ADDRESS_COMMAND_BITS | (address & ADDRESS_COMMAND_MASK))?;
        for byte in bytes {
            self.send(byte)?;
        }
        self.stop();
        Ok(())
    }

    pub fn set_brightness(&mut self, level: u8) -> Result<(), Error> {
        self.start();
        self.send(DISPLAY_CONTROL_BRIGHTNESS_BITS | (level & DISPLAY_CONTROL_BRIGHTNESS_MASK))?;
        self.stop();
        Ok(())
    }

    fn send(&mut self, byte: u8) -> Result<(), Error> {
        let mut rest = byte;
        //let mut doo = (self.dio_to_output)(self.dio);
        for _ in 0..8 {
            self.clk.set_low().unwrap();
            if rest & 1 != 0 {
                self.dio.set_high().unwrap();
            } else {
                self.dio.set_low().unwrap();
            }
            rest = rest >> 1;
            self.clk.set_high().unwrap();
            self.delay();
        }

        // Wait for the ACK
        self.clk.set_low().unwrap();
        self.dio.set_high().unwrap();
        self.clk.set_high().unwrap();
        {
            let di = (self.dio_to_input)(self.dio);
            for _ in 0..255 {
                if let Ok(true) = di.is_low() {
                    return Ok(());
                }
                self.delay();
            }
        }

        Err(Error::Ack)
    }

    fn start(&mut self) {
        //let mut doo = (self.dio_to_output)(self.dio);
        self.clk.set_low().unwrap();
        self.dio.set_high().unwrap();
        self.clk.set_high().unwrap();
        self.delay();
        self.dio.set_low().unwrap();
    }

    fn stop(&mut self) {
        //let mut doo = (self.dio_to_output)(self.dio);
        self.clk.set_low().unwrap();
        self.dio.set_low().unwrap();
        self.clk.set_high().unwrap();
        self.delay();
        self.dio.set_high().unwrap();
        self.delay();
    }

    fn delay(&mut self) {
        self.delay.delay_us(DELAY_USECS);
    }
}

const MAX_FREQ_KHZ: u16 = 250;
const USECS_IN_MSEC: u16 = 1_000;
const DELAY_USECS: u16 = USECS_IN_MSEC / MAX_FREQ_KHZ;

const ADDRESS_AUTO_INCREMENT_1_MODE: u8 = 0x40;

const ADDRESS_COMMAND_BITS: u8 = 0xc0;
const ADDRESS_COMMAND_MASK: u8 = 0x0f;

const DISPLAY_CONTROL_BRIGHTNESS_BITS: u8 = 0x88;
const DISPLAY_CONTROL_BRIGHTNESS_MASK: u8 = 0x07;

const DIGITS: [u8; 16] = [
    0x3f, 0x06, 0x5b, 0x4f, //
    0x66, 0x6d, 0x7d, 0x07, //
    0x7f, 0x6f, 0x77, 0x7c, //
    0x39, 0x5e, 0x79, 0x71, //
];
