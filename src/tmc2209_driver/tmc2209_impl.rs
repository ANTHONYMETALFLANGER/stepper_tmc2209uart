use super::read_write_config_registers::{
    get_registers_changed_in_config, write_registers_changed_in_config,
};
use crate::reg_processor::process_reg_config;
use crate::structures::*;
use crate::tmc2209_driver::TMC2209UART;
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::digital::OutputPin;
use embedded_io::{Read, Write};
use tmc2209_driver::tmc2209_traits::{
    EnableTMC2209UartControl, TMC2209UartControl,
};

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
    /// Apply configuration to TMC2209 driver
    /// Only changed registers will be written
    fn apply_config(&mut self, config: &TMC2209_Config) {
        let mut ready_registers = get_registers_changed_in_config(
            self.shared_uart,
            self.base_config.uart_address,
            config,
        );
        process_reg_config(&mut ready_registers, config, &mut self.base_config);
        write_registers_changed_in_config(
            self.shared_uart,
            self.base_config.uart_address,
            &ready_registers,
        );
    }
}
