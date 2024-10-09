use crate::structures::{
    config::TMC2209_Config, debug_readed_config::TMC2209_DebugConfig,
    registers_collection::TMC2209_ConfigRegisters,
};
use crate::utils::{
    calc::mres_to_microsteps,
    tmc_read_write::test_uart_connection,
    tmc_read_write::{read_reg_blocking, write_reg},
};
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_io::{Read, Write};

pub fn get_registers_changed_in_config<'a, Uart: Read + Write>(
    uart: &'a mut Uart,
    uart_address: u8,
    config: &TMC2209_Config,
) -> Result<TMC2209_ConfigRegisters, ()> {
    let changes = config.which_registers_changed();
    let mut output = TMC2209_ConfigRegisters::new();

    if changes.gconf {
        let gconf =
            read_reg_blocking::<tmc2209::reg::GCONF, _>(uart, uart_address)?;
        output.gconf = Some(gconf);
    }

    if changes.chopconf {
        let chopconf =
            read_reg_blocking::<tmc2209::reg::CHOPCONF, _>(uart, uart_address)?;
        output.chopconf = Some(chopconf);
    }

    if changes.slaveconf {
        let slaveconf = tmc2209::reg::SLAVECONF::default();
        output.slaveconf = Some(slaveconf);
    }

    if changes.factory_conf {
        let factory_conf =
            read_reg_blocking::<tmc2209::reg::FACTORY_CONF, _>(uart, 0)?;
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
        let pwmconf =
            read_reg_blocking::<tmc2209::reg::PWMCONF, _>(uart, uart_address)?;
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
    Ok(output)
}

pub fn debug_read_config_from_driver<'a, Uart: Read + Write>(
    uart: &'a mut Uart,
    uart_address: u8,
) -> Result<TMC2209_DebugConfig, ()> {
    let gconf =
        read_reg_blocking::<tmc2209::reg::GCONF, _>(uart, uart_address)?;
    let chopconf =
        read_reg_blocking::<tmc2209::reg::CHOPCONF, _>(uart, uart_address)?;
    let factory_conf =
        read_reg_blocking::<tmc2209::reg::FACTORY_CONF, _>(uart, 0)?;
    let pwmconf =
        read_reg_blocking::<tmc2209::reg::PWMCONF, _>(uart, uart_address)?;

    Ok(TMC2209_DebugConfig {
        microsteps: mres_to_microsteps(chopconf.mres()),
        interpolation: chopconf.ntpol(),
        blank_time: chopconf.tbl(),
        hysteresis_end: chopconf.hend(),
        hysteresis_start: chopconf.hstrt(),
        en_spreadcycle: gconf.en_spread_cycle(),
        pdn_disable: gconf.pdn_disable(),
        pwm_ofs: pwmconf.pwm_ofs(),
        pwm_grad: pwmconf.pwm_grad(),
        pwm_freq: pwmconf.pwm_freq(),
        pwm_autoscale: pwmconf.pwm_autoscale(),
        pwm_autograd: pwmconf.pwm_autograd(),
        pwm_reg: pwmconf.pwm_reg(),
        pwm_lim: pwmconf.pwm_lim(),
        freewheel: pwmconf.freewheel(),
        internal_rsense: gconf.internal_rsense(),
        i_scale_analog: gconf.i_scale_analog(),
        mstep_reg_select: gconf.mstep_reg_select(),
        multistep_filt: gconf.multistep_filt(),
        index_otpw: gconf.index_otpw(),
        index_step: gconf.index_step(),
        toff: chopconf.toff(),
        vsense: chopconf.vsense(),
        dedge: chopconf.dedge(),
        diss2g: chopconf.diss2g(),
        diss2vs: chopconf.diss2vs(),
        fclktrim: factory_conf.fclktrim(),
        ottrim: factory_conf.ottrim(),
        shaft: gconf.shaft(),
    })
}

pub fn write_registers_changed_in_config<'a, Uart: Read + Write>(
    uart: &'a mut Uart,
    uart_address: u8,
    registers: &TMC2209_ConfigRegisters,
) -> Result<(), ()> {
    if let Some(gconf) = registers.gconf {
        write_reg(uart, uart_address, gconf)?;
    }

    if let Some(chopconf) = registers.chopconf {
        write_reg(uart, uart_address, chopconf)?;
    }

    if let Some(slaveconf) = registers.slaveconf {
        write_reg(uart, uart_address, slaveconf)?;
    }

    if let Some(factoryconf) = registers.factory_conf {
        write_reg(uart, 0, factoryconf)?;
    }

    if let Some(ihold_irun) = registers.ihold_irun {
        write_reg(uart, uart_address, ihold_irun)?;
    }

    if let Some(coolconf) = registers.coolconf {
        write_reg(uart, uart_address, coolconf)?;
    }

    if let Some(pwmconf) = registers.pwmconf {
        write_reg(uart, uart_address, pwmconf)?;
    }

    if let Some(tpowerdown) = registers.tpowerdown {
        write_reg(uart, uart_address, tpowerdown)?;
    }

    if let Some(tpwmthrs) = registers.tpwmthrs {
        write_reg(uart, uart_address, tpwmthrs)?;
    }

    if let Some(sgthrs) = registers.sgthrs {
        write_reg(uart, uart_address, sgthrs)?;
    }

    if let Some(tcoolthrs) = registers.tcoolthrs {
        write_reg(uart, uart_address, tcoolthrs)?;
    }

    Ok(())
}

pub fn read_sg_result<'a, Uart: Read + Write>(
    uart: &'a mut Uart,
    uart_address: u8,
) -> Result<u16, ()> {
    return Ok(read_reg_blocking::<tmc2209::reg::SG_RESULT, _>(
        uart,
        uart_address,
    )?
    .get());
}

pub fn test_connection<'a, Uart: Read + Write>(
    uart: &'a mut Uart,
    uart_address: u8,
) -> bool {
    return test_uart_connection(uart, uart_address);
}

pub fn set_vactual<Uart: Read + Write>(
    uart: &mut Uart,
    uart_address: u8,
    v_actual: i32,
) -> Result<(), ()> {
    let mut v_actual_reg = tmc2209::reg::VACTUAL::default();
    v_actual_reg.set(v_actual);
    write_reg(uart, uart_address, v_actual_reg)?;
    Ok(())
}
