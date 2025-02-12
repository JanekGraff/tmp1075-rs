/// Conversion Rate setting
/// See the [datasheet (section 7.5.1.2)](https://www.ti.com/lit/gpn/tmp1075) for more info.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConversionRate {
    #[default]
    /// 27.5ms conversion rate
    Rate27_5ms = 0b00,
    /// 55ms conversion rate
    Rate55ms = 0b01,
    /// 110ms conversion rate
    Rate110ms = 0b10,
    /// 220ms conversion rate (250ms on TMP1075N)
    ///
    /// # NOTE:
    ///
    /// On TMP1075**N** this is fixed to 0b11 (250ms conversion rate)
    Rate220ms = 0b11,
}

/// Consecutive fault measurements to trigger the alert function
/// See the [datasheet (section 7.5.1.2)](https://www.ti.com/lit/gpn/tmp1075) for more info.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConsecutiveFaults {
    #[default]
    /// 1 Fault
    One = 0b00,
    /// 2 Faults
    Two = 0b01,
    /// 3 Faults (4 on TMP1075N)
    Three = 0b10,
    /// 4 Faults (6 on TMP1075N)
    Four = 0b11,
}
