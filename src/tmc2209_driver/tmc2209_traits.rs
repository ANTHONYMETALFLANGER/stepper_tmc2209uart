///! kals
use crate::structures::*;
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
    fn apply_config(&mut self, config: &TMC2209_Config);
    //     fn read_full_config(&mut self) -> Result<TMC2209_Config, ()>;
    //     fn set_vactual(&mut self, v_actual: u16) -> Result<(), ()>;
    //     fn set_shaft(&mut self, shaft: bool) -> Result<(), ()>;
}
