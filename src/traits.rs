/// Enable microstepping mode control for a driver
///
/// The `Resources` type parameter defines the hardware resources required for
/// controlling microstepping mode.
pub trait EnableTMC2209UartControl<Resources> {
    /// The type of the driver after microstepping mode control has been enabled
    type WithUartControl: UartTMC2209Control;

    /// Enable microstepping mode control
    fn enable_uart_control(self, res: Resources) -> Self::WithUartControl;
}

pub trait UartTMC2209Control {
    fn apply_tmc2209_config(&mut self) -> Result<(), ()>;
}
