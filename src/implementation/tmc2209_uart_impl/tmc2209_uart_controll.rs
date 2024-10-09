use core::cell::RefCell;

use super::config_read_write_methods::{
    debug_read_config_from_driver, get_registers_changed_in_config,
    read_sg_result, set_vactual, test_connection,
    write_registers_changed_in_config,
};
use super::reg_processor::process_reg_config;

use crate::structures::base_config::TMC2209_BaseConfig;
use crate::{
    structures::{
        config::TMC2209_Config, debug_readed_config::TMC2209_DebugConfig,
        saved_config::TMC2209_SavedConfig,
    },
    TMC2209UART,
};
use critical_section::Mutex;
use embedded_io::{Read, Write};

impl<'a, Uart: Read + Write> TMC2209UART<'a, Uart> {
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

    /// Load readable registers from driver and save it in saved_config
    pub fn init_saved_config(&mut self) -> Result<(), ()> {
        let debug_config = self.debug_read_config_from_driver()?;
        self.saved_config =
            TMC2209_SavedConfig::new_from_debug_config(&debug_config);
        Ok(())
    }

    /// Send TMC2209_Config to driver
    pub fn apply_config(&mut self, config: &TMC2209_Config) -> Result<(), ()> {
        // Read registers changed by config
        let mut ready_registers = critical_section::with(|cs| {
            let mut uart_cell = self.shared_uart.borrow(cs).borrow_mut();
            if let Some(uart) = uart_cell.as_mut() {
                return get_registers_changed_in_config(
                    uart,
                    self.base_config.uart_address,
                    config,
                );
            } else {
                return Err(());
            }
        });

        match ready_registers.as_mut() {
            Ok(ready_registers) => {
                // Write changes in registers
                let mut config_for_save = self.saved_config.clone();
                process_reg_config(
                    ready_registers,
                    config,
                    &mut self.base_config,
                    &mut config_for_save,
                );

                // Write registers to driver
                critical_section::with(|cs| {
                    let mut uart_cell =
                        self.shared_uart.borrow(cs).borrow_mut();
                    if let Some(uart) = uart_cell.as_mut() {
                        return write_registers_changed_in_config(
                            uart,
                            self.base_config.uart_address,
                            ready_registers,
                        );
                    }

                    // Config writed succesful, save it
                    self.saved_config = config_for_save;
                    Ok(())
                })
            }
            Err(_) => return Err(()),
        }
    }

    /// Read config directly from driver (Not all registers can be readed)
    pub fn debug_read_config_from_driver(
        &mut self,
    ) -> Result<TMC2209_DebugConfig, ()> {
        critical_section::with(|cs| {
            let mut uart_cell = self.shared_uart.borrow(cs).borrow_mut();
            if let Some(uart) = uart_cell.as_mut() {
                return debug_read_config_from_driver(
                    uart,
                    self.base_config.uart_address,
                );
            } else {
                return Err(());
            }
        })
    }

    /// Move motor to v_actual steps
    pub fn vactual(&mut self, v_actual: i32) -> Result<(), ()> {
        critical_section::with(|cs| {
            let mut uart_cell = self.shared_uart.borrow(cs).borrow_mut();
            if let Some(uart) = uart_cell.as_mut() {
                set_vactual(uart, self.base_config.uart_address, v_actual)
            } else {
                return Err(());
            }
        })
    }

    /// Set motor direction
    pub fn set_shaft(&mut self, shaft: bool) -> Result<(), ()> {
        let config = TMC2209_Config {
            shaft: Some(shaft),
            ..Default::default()
        };
        self.apply_config(&config)
    }

    /// Invert motor direction
    pub fn shaft(&mut self) -> Result<(), ()> {
        let config = TMC2209_Config {
            shaft: Some(!self.saved_config.shaft),
            ..Default::default()
        };
        self.apply_config(&config)
    }

    /// Read SG_RESULT
    pub fn read_sg_result(&mut self) -> Result<u16, ()> {
        critical_section::with(|cs| {
            let mut uart_cell = self.shared_uart.borrow(cs).borrow_mut();
            if let Some(uart) = uart_cell.as_mut() {
                read_sg_result(uart, self.base_config.uart_address)
            } else {
                return Err(());
            }
        })
    }

    /// Get saved config
    pub fn get_saved_config(&self) -> &TMC2209_SavedConfig {
        &self.saved_config
    }

    /// Test connect to TMC2209. Returns true if connection was succesful
    pub fn test_connection(&self) -> bool {
        critical_section::with(|cs| {
            let mut uart_cell = self.shared_uart.borrow(cs).borrow_mut();
            if let Some(uart) = uart_cell.as_mut() {
                test_connection(uart, self.base_config.uart_address)
            } else {
                return false;
            }
        })
    }
}
