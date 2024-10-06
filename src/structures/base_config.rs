#[allow(non_camel_case_types)]
//. Some values that are not sent to the driver, but are involved in the calculations
pub struct TMC2209_BaseConfig {
    /// You can connect multiple drivers to one uart (see tmc2209 datasheet, page 17, 18)
    pub uart_address: u8,

    /// Sense resistor (see tmc2209 datasheet, page 49)
    pub r_sense: f32,

    /// You can decrease hold current (in comparison to run current) with this multiplier
    pub ihold_multiplier: f32,
}
