use crate::{
    structures::{
        base_config::TMC2209_BaseConfig, saved_config::TMC2209_SavedConfig,
    },
    TMC2209UART,
};
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_io::{Read, Write};

impl<'a, Uart> TMC2209UART<'a, Uart>
where
    Uart: Read + Write,
{
    /// There is a `uart: &'a Mutex<RefCell<Option<Uart>>>` parameter here.
    /// You are supposed to use a special pettern to share Uart instance
    /// between different parts of the code (usually tasks, interrupts)
    /// (see example: https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/serial_interrupts.rs).
    /// Basically the TMC2208UART structure just stores an immutable reference to
    /// `Mutex<RefCell<Option<Uart>>>` which is used to get a mutable reference to
    /// Uart inside critical_section::with() when needed
    ///
    /// This decision was made because of the need to create
    /// several TMC2209UART instances that must have mutable access to one uart
    pub fn new(
        shared_uart: &'a Mutex<RefCell<Option<Uart>>>,
        base_config: TMC2209_BaseConfig,
    ) -> Self {
        Self {
            shared_uart,
            base_config,
            saved_config: TMC2209_SavedConfig::new(),
        }
    }
}
