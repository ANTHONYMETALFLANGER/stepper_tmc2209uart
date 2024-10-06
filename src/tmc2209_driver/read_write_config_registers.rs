use crate::structures::*;
use crate::utils::{read_reg_blocking, write_reg};
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_io::{Read, Write};

pub fn get_registers_changed_in_config<'a, Uart: Read + Write>(
    uart: &'a Mutex<RefCell<Option<Uart>>>,
    uart_address: u8,
    config: &TMC2209_Config,
) -> TMC2209_ConfigRegisters {
    let changes = config.which_registers_changed();
    let mut output = TMC2209_ConfigRegisters::new();

    critical_section::with(|cs| {
        let mut uart_cell = uart.borrow(cs).borrow_mut();

        if let Some(uart) = uart_cell.as_mut() {
            if changes.gconf {
                let gconf = read_reg_blocking::<tmc2209::reg::GCONF, _>(
                    uart,
                    uart_address,
                )
                .unwrap();
                output.gconf = Some(gconf);
            }

            if changes.chopconf {
                let chopconf = read_reg_blocking::<tmc2209::reg::CHOPCONF, _>(
                    uart,
                    uart_address,
                )
                .unwrap();
                output.chopconf = Some(chopconf);
            }

            if changes.slaveconf {
                let slaveconf = tmc2209::reg::SLAVECONF::default();
                output.slaveconf = Some(slaveconf);
            }

            if changes.factory_conf {
                let factory_conf =
                    read_reg_blocking::<tmc2209::reg::FACTORY_CONF, _>(uart, 0)
                        .unwrap();
                output.factory_conf = Some(factory_conf);
            }

            if changes.ihold_irun {
                let ihold_irun = tmc2209::reg::IHOLD_IRUN::default();
                output.ihold_irun = Some(ihold_irun);
            }

            if changes.coolconf {
                let coolconf = tmc2209::reg::COOLCONF::default();
                output.coolconf = Some(coolconf);
            }

            if changes.pwmconf {
                let pwmconf = read_reg_blocking::<tmc2209::reg::PWMCONF, _>(
                    uart,
                    uart_address,
                )
                .unwrap();
                output.pwmconf = Some(pwmconf);
            }

            if changes.tpowerdown {
                let tpowerdown = tmc2209::reg::TPOWERDOWN::default();
                output.tpowerdown = Some(tpowerdown);
            }

            if changes.tpwmthrs {
                let tpwmthrs = tmc2209::reg::TPWMTHRS::default();
                output.tpwmthrs = Some(tpwmthrs);
            }

            if changes.sgthrs {
                let sgthrs = tmc2209::reg::SGTHRS::default();
                output.sgthrs = Some(sgthrs);
            }

            if changes.tcoolthrs {
                let tcoolthrs = tmc2209::reg::TCOOLTHRS::default();
                output.tcoolthrs = Some(tcoolthrs);
            }
        }
    });
    output
}

pub fn write_registers_changed_in_config<'a, Uart: Read + Write>(
    uart: &'a Mutex<RefCell<Option<Uart>>>,
    uart_address: u8,
    registers: &TMC2209_ConfigRegisters,
) {
    critical_section::with(|cs| {
        let mut uart_cell = uart.borrow(cs).borrow_mut();

        if let Some(uart) = uart_cell.as_mut() {
            if let Some(gconf) = registers.gconf {
                write_reg(uart, uart_address, gconf).unwrap();
            }

            if let Some(chopconf) = registers.chopconf {
                write_reg(uart, uart_address, chopconf).unwrap();
            }

            if let Some(slaveconf) = registers.slaveconf {
                write_reg(uart, uart_address, slaveconf).unwrap();
            }

            if let Some(factoryconf) = registers.factory_conf {
                write_reg(uart, 0, factoryconf).unwrap();
            }

            if let Some(ihold_irun) = registers.ihold_irun {
                write_reg(uart, uart_address, ihold_irun).unwrap();
            }

            if let Some(coolconf) = registers.coolconf {
                write_reg(uart, uart_address, coolconf).unwrap();
            }

            if let Some(pwmconf) = registers.pwmconf {
                write_reg(uart, uart_address, pwmconf).unwrap();
            }

            if let Some(tpowerdown) = registers.tpowerdown {
                write_reg(uart, uart_address, tpowerdown).unwrap();
            }

            if let Some(tpwmthrs) = registers.tpwmthrs {
                write_reg(uart, uart_address, tpwmthrs).unwrap();
            }

            if let Some(sgthrs) = registers.sgthrs {
                write_reg(uart, uart_address, sgthrs).unwrap();
            }

            if let Some(tcoolthrs) = registers.tcoolthrs {
                write_reg(uart, uart_address, tcoolthrs).unwrap();
            }
        }
    });
}
