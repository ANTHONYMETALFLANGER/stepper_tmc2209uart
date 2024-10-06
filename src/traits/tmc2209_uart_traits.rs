///! kals
use crate::structures::{
    config::TMC2209_Config, debug_readed_config::TMC2209_DebugConfig,
};
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_io::{Read, Write};

/// Enable TMC2209 controll trough uart
pub trait EnableTMC2209UartControl<'a, Uart: Read + Write> {
    // Uart type
    type UartRef;

    /// The type of the driver after microstepping mode control has been enabled
    type WithUartControl: TMC2209UartControl;

    /// Enable microstepping mode control
    fn enable_uart_control(
        self,
        uart: &'a Mutex<RefCell<Option<Uart>>>,
    ) -> Self::WithUartControl;
}

/// TMC2209 uart controll methods
pub trait TMC2209UartControl {
    // Configuration controll

    /// Apply configuration to TMC2209 driver
    /// Only changed registers will be written
    fn apply_config(&mut self, config: &TMC2209_Config) -> Result<(), ()>;

    /// Read configuration directly from TMC2209 driver.
    /// Used for debug purposes
    fn debug_read_config_from_driver(
        &mut self,
    ) -> Result<TMC2209_DebugConfig, ()>;

    // Controll stepper motor trough uart

    /// Set vactual value
    /// Run Motor using driver internal pulse generator (see tmc2209 datasheet, page 64)
    fn vactual(&mut self, v_actual: i32) -> Result<(), ()>;

    /// Set motor direction (useful if you controll motor trough vactual)
    fn set_shaft(&mut self, shaft: bool) -> Result<(), ()>;

    /// Inverse motor direction (useful if you controll motor trough vactual)
    /// Direction will invert automaticly based on previous value
    fn shaft(&mut self) -> Result<(), ()>;
}
