use super::config_read_write_methods::{
    debug_read_config_from_driver, get_registers_changed_in_config,
    read_sg_result, set_vactual, write_registers_changed_in_config,
};
use super::reg_processor::process_reg_config;

use crate::{
    structures::{
        config::TMC2209_Config, debug_readed_config::TMC2209_DebugConfig,
        saved_config::TMC2209_SavedConfig,
    },
    TMC2209UART,
};
use embedded_io::{Read, Write};

impl<'a, Uart: Read + Write> TMC2209UART<'a, Uart> {
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

    pub fn set_shaft(&mut self, shaft: bool) -> Result<(), ()> {
        let config = TMC2209_Config {
            shaft: Some(shaft),
            ..Default::default()
        };
        self.apply_config(&config)
    }

    pub fn shaft(&mut self) -> Result<(), ()> {
        let config = TMC2209_Config {
            shaft: Some(!self.saved_config.shaft),
            ..Default::default()
        };
        self.apply_config(&config)
    }

    pub fn read_sg_result(&mut self) -> Result<u16, ()> {
        read_sg_result(self.shared_uart, self.base_config.uart_address)
    }

    pub fn get_saved_config(&self) -> &TMC2209_SavedConfig {
        &self.saved_config
    }
}
