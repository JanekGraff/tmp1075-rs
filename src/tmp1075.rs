const DEFAULT_I2C_ADDRESS: u8 = 0b1001000;

use crate::register_settings::{
    AlertFunction, AlertPolarity, ConsecutiveFaults, ConversionRate, PowerMode,
};
use crate::registers::{
    Register, ALERT_FUNCTION_MASK, ALERT_POLARITY_MASK, CONSECUTIVE_FAULT_MASK,
    CONSECUTIVE_FAULT_SHIFT, CONVERSION_RATE_MASK, CONVERSION_RATE_SHIFT, POWER_MODE_MASK,
};

#[cfg(feature = "blocking")]
use embedded_hal::i2c::I2c;

#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

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

    /// Get the temperature as raw value
    pub async fn get_temperature_raw(&mut self) -> Result<u16, I2C::Error> {
        self.read_reg(Register::TEMP).await
    }

    /// Get the temperature converted to f32
    pub async fn get_temperature_float(&mut self) -> Result<f32, I2C::Error> {
        let raw = self.get_temperature_raw().await?;
        Ok(convert_temperature_float(raw))
    }

    /// Get the MSB of the temperature as i8
    pub async fn get_temperature_msb(&mut self) -> Result<i8, I2C::Error> {
        let raw = self.get_temperature_raw().await?;
        Ok(convert_temperature_msb(raw))
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
            PowerMode::Shutdown => self.reg_set_bits(Register::CFGR, POWER_MODE_MASK).await,
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

fn convert_temperature_float(raw: u16) -> f32 {
    // Convert to a signed 16-bit integer by extending the sign bit of the 12-bit value
    let temp_raw = (raw >> 4) as i16;
    let temp_signed = if temp_raw & 0x0800 != 0 {
        // Check if negative (bit 11 set)
        temp_raw | 0xF000u16 as i16 // Sign extend by setting upper 4 bits to 1
    } else {
        temp_raw
    };

    // Apply the Q4 scaling factor (0.0625 = 1/16)
    temp_signed as f32 * 0.0625
}

fn convert_temperature_msb(raw: u16) -> i8 {
    (raw >> 8) as i8
}

#[cfg(test)]
mod tests {
    use super::{convert_temperature_float, convert_temperature_msb};

    #[test]
    fn test_msb_temperature_conversion() {
        for t in i8::MIN as i16..i8::MAX as i16 {
            let raw = (t << 8) as u16;
            assert_eq!(convert_temperature_msb(raw), t as i8);
        }
    }

    #[test]
    fn test_float_temperature_conversion() {
        for t in i8::MIN as i16 + 1..i8::MAX as i16 {
            let raw = (t << 8) as u16;
            assert_eq!(convert_temperature_float(raw), t as f32);
        }
    }
}
