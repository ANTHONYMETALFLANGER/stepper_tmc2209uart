use super::{
    base_config::TMC2209_BaseConfig, saved_config::TMC2209_SavedConfig,
};

/// The TMC2209UART driver API
///
/// Users are not expected to use this API directly, except to create an
/// instance using [`TMC2209UART::new`].
pub struct TMC2209UART<Enable, Fault, Sleep, Reset, UartRef, Step, Dir> {
    pub enable: Enable,
    pub fault: Fault,
    pub shared_uart: UartRef,
    pub base_config: TMC2209_BaseConfig,
    pub saved_config: TMC2209_SavedConfig,
    pub sleep: Sleep,
    pub reset: Reset,
    pub step: Step,
    pub dir: Dir,
}
