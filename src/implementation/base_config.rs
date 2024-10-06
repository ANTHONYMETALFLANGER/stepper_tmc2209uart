use crate::structures::base_config::TMC2209_BaseConfig;

impl<'a> Default for TMC2209_BaseConfig {
    fn default() -> Self {
        TMC2209_BaseConfig {
            uart_address: 0,
            r_sense: 0.11, // Default for SilentStepStick series drivers
            ihold_multiplier: 0.5, // Decreas hold current with 50%
        }
    }
}
