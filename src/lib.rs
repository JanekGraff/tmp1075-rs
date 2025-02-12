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

use registers::{
    Register, ALERT_FUNCTION_MASK, ALERT_POLARITY_MASK, CONSECUTIVE_FAULT_MASK,
    CONSECUTIVE_FAULT_SHIFT, CONVERSION_RATE_MASK, CONVERSION_RATE_SHIFT, POWER_MODE_MASK,
};

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

    /// Set the conversion rate
    /// See the [datasheet (section 7.5.1.2)](https://www.ti.com/lit/gpn/tmp1075) for more info.
    pub async fn set_conversion_rate(&mut self, rate: ConversionRate) -> Result<(), I2C::Error> {
        self.modify_reg(Register::CFGR, |v| {
            v & !CONVERSION_RATE_MASK | (rate as u16) << CONVERSION_RATE_SHIFT
        })
        .await
    }

    /// Set the number of consecutive fault measurements to trigger the alert function
    /// See the [datasheet (section 7.5.1.2)](https://www.ti.com/lit/gpn/tmp1075) for more info.
    pub async fn set_consecutive_faults(
        &mut self,
        faults: ConsecutiveFaults,
    ) -> Result<(), I2C::Error> {
        self.modify_reg(Register::CFGR, |v| {
            v & !CONSECUTIVE_FAULT_MASK | (faults as u16) << CONSECUTIVE_FAULT_SHIFT
        })
        .await
    }

    /// Set the polarity of the alert pin
    /// See the [datasheet (section 7.5.1.2)](https://www.ti.com/lit/gpn/tmp1075) for more info.
    pub async fn set_alert_polarity(&mut self, polarity: AlertPolarity) -> Result<(), I2C::Error> {
        match polarity {
            AlertPolarity::ActiveLow => {
                self.reg_reset_bits(Register::CFGR, ALERT_POLARITY_MASK)
                    .await
            }
            AlertPolarity::ActiveHigh => {
                self.reg_set_bits(Register::CFGR, ALERT_POLARITY_MASK).await
            }
        }
    }

    /// Set the function of the alert pin
    /// See the [datasheet (section 7.5.1.2)](https://www.ti.com/lit/gpn/tmp1075) for more info.
    pub async fn set_alert_function(&mut self, function: AlertFunction) -> Result<(), I2C::Error> {
        match function {
            AlertFunction::ComparatorMode => {
                self.reg_reset_bits(Register::CFGR, ALERT_FUNCTION_MASK)
                    .await
            }
            AlertFunction::InterruptMode => {
                self.reg_set_bits(Register::CFGR, ALERT_FUNCTION_MASK).await
            }
        }
    }

    /// Set the power mode
    /// See the [datasheet (section 7.5.1.2)](https://www.ti.com/lit/gpn/tmp1075) for more info.
    pub async fn set_power_mode(&mut self, mode: PowerMode) -> Result<(), I2C::Error> {
        match mode {
            PowerMode::ContinuousConversion => {
                self.reg_reset_bits(Register::CFGR, POWER_MODE_MASK).await
            }
            PowerMode::ShutdowMode => self.reg_set_bits(Register::CFGR, POWER_MODE_MASK).await,
        }
    }

    /// Get the device ID
    /// See the [datasheet (section 7.5.1.5)](https://www.ti.com/lit/gpn/tmp1075) for more info.
    pub async fn get_device_id(&mut self) -> Result<u16, I2C::Error> {
        self.read_reg(Register::DIEID).await
    }

    #[inline]
    async fn read_reg(&mut self, reg: Register) -> Result<u16, I2C::Error> {
        let mut data = [0_u8; 2];
        self.bus
            .write_read(self.address, &[reg.addr()], &mut data)
            .await?;
        Ok(u16::from_be_bytes(data))
    }

    #[inline]
    async fn write_reg(&mut self, reg: Register, data: u16) -> Result<(), I2C::Error> {
        let bytes = data.to_be_bytes();
        let buffer = [reg.addr(), bytes[0], bytes[1]];
        self.bus.write(self.address, &buffer).await
    }

    #[inline]
    async fn modify_reg<F: FnOnce(u16) -> u16>(
        &mut self,
        reg: Register,
        f: F,
    ) -> Result<(), I2C::Error> {
        let r = self.read_reg(reg).await?;
        self.write_reg(reg, f(r)).await
    }

    #[inline]
    async fn reg_set_bits(&mut self, reg: Register, mask: u16) -> Result<(), I2C::Error> {
        self.modify_reg(reg, |r| r | mask).await
    }

    #[inline]
    async fn reg_reset_bits(&mut self, reg: Register, mask: u16) -> Result<(), I2C::Error> {
        self.modify_reg(reg, |r| r & !mask).await
    }
}
