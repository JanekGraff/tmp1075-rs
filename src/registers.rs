#![allow(dead_code)]

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Register {
    TEMP = 0x00,
    CFGR = 0x01,
    LLIM = 0x02,
    HLIM = 0x03,
    DIEID = 0x0F,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

// ------------------- Register Masks ------------------- //
// ---------- CFGR ---------- //
pub const ONE_SHOT_MASK: u16 = 0b1000_0000_0000_0000;
pub const CONVERSION_RATE_MASK: u16 = 0b0110_0000_0000_0000;
pub const CONVERSION_RATE_SHIFT: u16 = 13;
pub const CONSECUTIVE_FAULT_MASK: u16 = 0b0001_1000_0000_0000;
pub const CONSECUTIVE_FAULT_SHIFT: u16 = 11;
pub const ALERT_POLARITY_MASK: u16 = 0b0000_0100_0000_0000;
pub const ALERT_FUNCTION_MASK: u16 = 0b0000_0010_0000_0000;
pub const SHUTDOWN_MODE_MASK: u16 = 0b0000_0001_0000_0000;
// ---------- LLIM ---------- //
pub const LOW_LIMIT_MASK: u16 = 0b1111_1111_1111_0000;
pub const LOW_LIMIT_SHIFT: u16 = 4;
// ---------- HLIM ---------- //
pub const HIGH_LIMIT_MASK: u16 = 0b1111_1111_1111_0000;
pub const HIGH_LIMIT_SHIFT: u16 = 4;
