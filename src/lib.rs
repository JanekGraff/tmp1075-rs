//! # TMP1075
//! A platform agnostic driver to interface with the TMP1075 temperature sensor.
//! The driver supports async and blocking mode, selectable wit hthe `async` and `blocking` features.
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

#[cfg(all(feature = "blocking", feature = "async"))]
compile_error!("Feature \"blocking\" and feature \"async\" cannot be enabled at the same time");
#[cfg(not(any(feature = "blocking", feature = "async")))]
compile_error!("Either feature \"blocking\" or feature \"async\" must be anbled");

mod register_settings;
mod registers;

const DEFAULT_I2C_ADDRESS: u8 = 0b1001000;

#[cfg(feature = "blocking")]
use embedded_hal::i2c::I2c;

#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

use registers::Register;

pub use register_settings::*;

/// TMP1075 driver
#[maybe_async_cfg::maybe(sync(feature = "blocking", keep_self), async(feature = "async"))]
pub struct Tmp1075<I2C> {
    address: u8,
    bus: I2C,
}

#[maybe_async_cfg::maybe(sync(feature = "blocking", keep_self), async(feature = "async"))]
impl<I2C: I2c> Tmp1075<I2C> {
    /// Create a new instance with the default I2C address (`0b1001000` / `0x48`)
    pub fn new(bus: I2C) -> Self {
        Self {
            address: DEFAULT_I2C_ADDRESS,
            bus,
        }
    }

    /// Create a new instance with the given address
    pub fn with_address(address: u8, bus: I2C) -> Self {
        Self { address, bus }
    }

    /// Get the temperature
    pub async fn get_temperature(&mut self) -> Result<u16, I2C::Error> {
        self.read_reg(Register::TEMP).await
    }

    #[inline]
    async fn read_reg(&mut self, reg: Register) -> Result<u16, I2C::Error> {
        let mut data = [0_u8; 2];
        self.bus
            .write_read(self.address, &[reg.addr()], &mut data)
            .await?;

        Ok(0)
    }
}
