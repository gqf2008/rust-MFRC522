/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
//! `MFRC522Bus` implementation for `Spidev`.
//!
//! WARNING: It conflicts with I2CDevice implementation so only one of
//! `spidev` `i2cdev` features may be selected.

use bus;
use bus::MFRC522Bus;
use embedded_hal::spi::FullDuplex;

use pcd::reg::Reg;

impl<SPI: FullDuplex<u8>> MFRC522Bus for SPI {
    #[inline]
    fn register_read(&mut self, reg: Reg) -> bus::Result<u8> {
        let reg_addr = bus::spi_reg_addr(reg, bus::Mode::Read);
        self.send(reg_addr).map_err(|_x| ())?;
        let value = self.read().map_err(|_x| ())?;
        Ok(value)
    }

    #[inline]
    fn register_write(&mut self, reg: Reg, value: u8) -> bus::Result<()> {
        let reg_addr = bus::spi_reg_addr(reg, bus::Mode::Write);
        self.send(reg_addr).map_err(|_x| ())?;
        self.send(value).map_err(|_x| ())?;
        Ok(())
    }

    fn register_write_slice(&mut self, reg: Reg, values: &[u8]) -> bus::MultiResult<usize> {
        let reg_addr = bus::spi_reg_addr(reg, bus::Mode::Write);
        let mut nwrite = 0;
        self.send(reg_addr).map_err(|_x| nwrite)?;
        for value in values.iter() {
            self.send(*value).map_err(|_x| nwrite)?;
            nwrite += 1;
        }

        Ok(nwrite)
    }
}
