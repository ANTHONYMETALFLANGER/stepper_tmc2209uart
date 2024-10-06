use super::config_read_write_methods::{
    debug_read_config_from_driver, get_registers_changed_in_config,
    set_vactual, write_registers_changed_in_config,
};
use super::reg_processor::process_reg_config;
use crate::structures::{
    config::TMC2209_Config, debug_readed_config::TMC2209_DebugConfig,
    driver::TMC2209UART,
};
use crate::traits::tmc2209_uart_traits::{
    EnableTMC2209UartControl, TMC2209UartControl,
};
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::digital::OutputPin;
use embedded_io::{Read, Write};

impl<'a, Reset, Uart, Step, Dir, OutputPinError>
    EnableTMC2209UartControl<'a, Uart>
    for TMC2209UART<(), (), (), Reset, (), Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
    Uart: Read<Error = OutputPinError> + Write<Error = OutputPinError> + 'a,
{
    type UartRef = &'a Mutex<RefCell<Option<Uart>>>; // Reference to uart mutex
    type WithUartControl =
        TMC2209UART<(), (), (), Reset, Self::UartRef, Step, Dir>; // UartUartRef, Step, Dir>;

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
    fn enable_uart_control(
        self,
        uart: &'a Mutex<RefCell<Option<Uart>>>,
    ) -> Self::WithUartControl {
        TMC2209UART {
            enable: self.enable,
            fault: self.fault,
            sleep: self.sleep,
            reset: self.reset,
            shared_uart: uart,
            base_config: self.base_config,
            saved_config: self.saved_config,
            step: self.step,
            dir: self.dir,
        }
    }
}

impl<'a, Reset, Uart, Step, Dir, OutputPinError> TMC2209UartControl
    for TMC2209UART<
        (),
        (),
        (),
        Reset,
        &'a Mutex<RefCell<Option<Uart>>>,
        Step,
        Dir,
    >
where
    Uart: Read<Error = OutputPinError> + Write<Error = OutputPinError>,
{
    fn apply_config(&mut self, config: &TMC2209_Config) -> Result<(), ()> {
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

    fn debug_read_config_from_driver(
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

    fn vactual(&mut self, v_actual: i32) -> Result<(), ()> {
        critical_section::with(|cs| {
            let mut uart_cell = self.shared_uart.borrow(cs).borrow_mut();
            if let Some(uart) = uart_cell.as_mut() {
                set_vactual(uart, self.base_config.uart_address, v_actual)
            } else {
                return Err(());
            }
        })
    }

    fn set_shaft(&mut self, shaft: bool) -> Result<(), ()> {
        let config = TMC2209_Config {
            shaft: Some(shaft),
            ..Default::default()
        };
        self.apply_config(&config)
    }

    fn shaft(&mut self) -> Result<(), ()> {
        let config = TMC2209_Config {
            shaft: Some(!self.saved_config.shaft),
            ..Default::default()
        };
        self.apply_config(&config)
    }
}
