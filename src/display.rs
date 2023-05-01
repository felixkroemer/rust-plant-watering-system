use core::fmt::Write as _;

use embedded_hal::blocking::i2c::Write;
use ssd1306::{
    mode::{TerminalMode},
    prelude::I2CInterface,
    rotation::DisplayRotation,
    size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306,
};

pub struct Display<I2C> {
    display: Ssd1306<I2CInterface<I2C>, DisplaySize128x64, TerminalMode>,
}

impl<I2C: Write> Display<I2C> {
    pub fn new(i2c: I2C) -> Self {
        let interface: I2CInterface<I2C> = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_terminal_mode();
        display.init_with_addr_mode(ssd1306::command::AddrMode::Page).unwrap();
        display.clear().unwrap();
        Self { display }
    }

    pub fn display(&mut self, reading: u16) {
        self.display.write_fmt(format_args!("{}", reading)).unwrap();
        self.display.write_char('\n').unwrap();
        // set timeout for clear;
    }
}
