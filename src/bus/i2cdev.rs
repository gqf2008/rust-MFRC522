/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
//! `MFRC522Bus` implementation for `I2CDevice`.
//! This is not tested at all. Would appricciate testing & bug feedback.
//!
//! WARNING: It conflicts with Spidev implementation so only one of
//! `spidev` `i2cdev` features may be selected.

use bus;
use bus::MFRC522Bus;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use pcd::reg::Reg;
const ADDR: u8 = 0x28;

impl<I2C: WriteRead + Write> MFRC522Bus for I2C {
    #[inline]
    fn register_read(&mut self, reg: Reg) -> bus::Result<u8> {
        let mut buffer = [0];
        self.write_read(ADDR, &[reg as u8], &mut buffer)
            .map_err(|_x| ())?;

        Ok(buffer[0])
    }

    #[inline]
    fn register_write(&mut self, reg: Reg, value: u8) -> bus::Result<()> {
        self.write(ADDR, &[reg as u8, value]).map_err(|_x| ())?;
        Ok(())
    }
}
