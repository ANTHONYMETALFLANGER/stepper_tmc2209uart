#![allow(missing_docs)]
use crate::structures::saved_config::TMC2209_SavedConfig;
use crate::structures::{
    base_config::TMC2209_BaseConfig, config::TMC2209_Config,
    registers_collection::TMC2209_ConfigRegisters,
};
use crate::utils::calc::{
    irun_to_rms_current, microsteps_to_mres, mres_to_microsteps,
    rms_current_to_ihold_irun_vsense,
    RmsCurrentToIholdIrunVsenseOutput as IholdIrunVsense,
};

/// Reg processor read user defined TMC2209_Config and make changes in loaded registers from driver.
/// Than modified instance of TMC2209_WritableRegisters can be pass into func that send it back to driver
pub fn process_reg_config(
    mutable_previous_regs: &mut TMC2209_ConfigRegisters,
    config: &TMC2209_Config,
    driver_base_config: &mut TMC2209_BaseConfig,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    process_driver_base_config(driver_base_config, config);

    if let Some(mut gconf) = mutable_previous_regs.gconf {
        process_gconf(&mut gconf, config, save_config_to);
    }

    if let Some(mut chopconf) = mutable_previous_regs.chopconf {
        process_chopconf(&mut chopconf, config, save_config_to);

        if let Some(mut ihold_irun) = mutable_previous_regs.ihold_irun {
            process_ihold_irun(
                &mut ihold_irun,
                &mut chopconf,
                driver_base_config,
                config,
                save_config_to,
            );
        }
    }

    if let Some(mut slaveconf) = mutable_previous_regs.slaveconf {
        process_slaveconf(&mut slaveconf, config, save_config_to);
    }

    if let Some(mut factoryconf) = mutable_previous_regs.factory_conf {
        process_factoryconf(&mut factoryconf, config, save_config_to);
    }

    if let Some(mut coolconf) = mutable_previous_regs.coolconf {
        process_coolconf(&mut coolconf, config, save_config_to);
    }

    if let Some(mut pwmconf) = mutable_previous_regs.pwmconf {
        process_pwmconf(&mut pwmconf, config, save_config_to);
    }

    if let Some(mut tpowerdown) = mutable_previous_regs.tpowerdown {
        process_tpowerdown(&mut tpowerdown, config, save_config_to);
    }

    if let Some(mut tpwmthrs) = mutable_previous_regs.tpwmthrs {
        process_tpwmthrs(&mut tpwmthrs, config, save_config_to);
    }

    if let Some(mut sgthrs) = mutable_previous_regs.sgthrs {
        process_sgthrs(&mut sgthrs, config, save_config_to);
    }

    if let Some(mut tcoolthrs) = mutable_previous_regs.tcoolthrs {
        process_tcoolthrs(&mut tcoolthrs, config, save_config_to);
    }
}

// Processors for different registers

pub fn process_driver_base_config(
    driver_base_config: &mut TMC2209_BaseConfig,
    config: &TMC2209_Config,
) {
    if let Some(uart_address) = config.uart_address {
        driver_base_config.uart_address = uart_address;
    }

    if let Some(ihold_multiplier) = config.ihold_multiplier {
        driver_base_config.ihold_multiplier = ihold_multiplier;
    }

    if let Some(r_sense) = config.r_sense {
        driver_base_config.r_sense = r_sense;
    }
}

pub fn process_gconf(
    gconf: &mut tmc2209::reg::GCONF,
    config: &TMC2209_Config,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    if let Some(i_scale_analog) = config.i_scale_analog {
        gconf.set_i_scale_analog(i_scale_analog);
        save_config_to.i_scale_analog = i_scale_analog;
    }

    if let Some(internal_rsense) = config.internal_rsense {
        gconf.set_internal_rsense(internal_rsense);
        save_config_to.internal_rsense = internal_rsense;
    }

    if let Some(en_spreadcycle) = config.en_spreadcycle {
        gconf.set_en_spread_cycle(en_spreadcycle);
        save_config_to.en_spreadcycle = en_spreadcycle;
    }

    if let Some(index_otpw) = config.index_otpw {
        gconf.set_index_otpw(index_otpw);
        save_config_to.index_otpw = index_otpw;
    }

    if let Some(index_step) = config.index_step {
        gconf.set_index_step(index_step);
        save_config_to.index_step = index_step;
    }

    if let Some(pdn_disable) = config.pdn_disable {
        gconf.set_pdn_disable(pdn_disable);
        save_config_to.pdn_disable = pdn_disable;
    }

    if let Some(mstep_reg_select) = config.mstep_reg_select {
        gconf.set_mstep_reg_select(mstep_reg_select);
        save_config_to.mstep_reg_select = mstep_reg_select;
    }

    if let Some(multistep_filt) = config.multistep_filt {
        gconf.set_multistep_filt(multistep_filt);
        save_config_to.multistep_filt = multistep_filt;
    }

    if let Some(shaft) = config.shaft {
        gconf.set_shaft(shaft);
        save_config_to.shaft = shaft;
    }
}

pub fn process_chopconf(
    chopconf: &mut tmc2209::reg::CHOPCONF,
    config: &TMC2209_Config,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    if let Some(dedge) = config.dedge {
        chopconf.set_dedge(dedge);
        save_config_to.dedge = dedge;
    }

    if let Some(diss2g) = config.diss2g {
        chopconf.set_diss2g(diss2g);
        save_config_to.diss2g = diss2g;
    }

    if let Some(diss2vs) = config.diss2vs {
        chopconf.set_diss2vs(diss2vs);
        save_config_to.diss2vs = diss2vs;
    }

    if let Some(hysteresis_end) = config.hysteresis_end {
        chopconf.set_hend(hysteresis_end);
        save_config_to.hysteresis_end = hysteresis_end;
    }

    if let Some(hysteresis_start) = config.hysteresis_start {
        chopconf.set_hstrt(hysteresis_start);
        save_config_to.hysteresis_start = hysteresis_start;
    }

    if let Some(interpolation) = config.interpolation {
        chopconf.set_intpol(interpolation);
        save_config_to.interpolation = interpolation;
    }

    if let Some(microsteps) = config.microsteps {
        chopconf.set_mres(microsteps_to_mres(microsteps));
        save_config_to.microsteps =
            mres_to_microsteps(microsteps_to_mres(microsteps));
    }

    if let Some(blank_time) = config.blank_time {
        chopconf.set_tbl(blank_time);
        save_config_to.blank_time = blank_time;
    }

    if let Some(toff) = config.toff {
        chopconf.set_toff(toff);
        save_config_to.toff = toff;
    }

    if let Some(vsense) = config.vsense {
        chopconf.set_vsense(vsense);
        save_config_to.vsense = vsense;
    }
}

pub fn process_ihold_irun(
    ihold_irun: &mut tmc2209::reg::IHOLD_IRUN,
    chopconf: &mut tmc2209::reg::CHOPCONF,
    driver_base_config: &TMC2209_BaseConfig,
    config: &TMC2209_Config,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    // Assuming that process_driver_values() runned before process_ihold_irun()

    if let Some(rms_current) = config.rms_current {
        let IholdIrunVsense {
            ihold,
            irun,
            vsense,
        } = rms_current_to_ihold_irun_vsense(
            rms_current,
            driver_base_config.r_sense,
            driver_base_config.ihold_multiplier,
        );
        ihold_irun.set_ihold(ihold);
        ihold_irun.set_irun(irun);
        chopconf.set_vsense(vsense);

        save_config_to.rms_current =
            irun_to_rms_current(irun, vsense, driver_base_config.r_sense);
    }

    if let Some(ihold_delay) = config.ihold_delay {
        ihold_irun.set_ihold_delay(ihold_delay);
        save_config_to.ihold_delay = ihold_delay;
    }
}

pub fn process_slaveconf(
    slaveconf: &mut tmc2209::reg::SLAVECONF,
    config: &TMC2209_Config,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    if let Some(senddelay) = config.senddelay {
        slaveconf.set(senddelay);
        save_config_to.senddelay = senddelay;
    }
}

pub fn process_factoryconf(
    factoryconf: &mut tmc2209::reg::FACTORY_CONF,
    config: &TMC2209_Config,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    if let Some(fclktrim) = config.fclktrim {
        factoryconf.set_fclktrim(fclktrim);
        save_config_to.fclktrim = fclktrim;
    }

    if let Some(ottrim) = config.ottrim {
        factoryconf.set_ottrim(ottrim);
        save_config_to.ottrim = ottrim;
    }
}

pub fn process_coolconf(
    coolconf: &mut tmc2209::reg::COOLCONF,
    config: &TMC2209_Config,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    if let Some(sedn) = config.sedn {
        coolconf.set_sedn(sedn);
        save_config_to.sedn = sedn;
    }

    if let Some(semin) = config.semin {
        coolconf.set_semin(semin);
        save_config_to.semin = semin;
    }

    if let Some(seup) = config.seup {
        coolconf.set_seup(seup);
        save_config_to.seup = seup;
    }

    if let Some(seimin) = config.seimin {
        coolconf.set_seimin(seimin);
        save_config_to.seimin = seimin;
    }

    if let Some(semax) = config.semax {
        coolconf.set_semax(semax);
        save_config_to.semax = semax;
    }
}

pub fn process_pwmconf(
    pwmconf: &mut tmc2209::reg::PWMCONF,
    config: &TMC2209_Config,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    if let Some(freewheel) = config.freewheel {
        pwmconf.set_freewheel(freewheel);
        save_config_to.freewheel = freewheel;
    }

    if let Some(pwm_autoscale) = config.pwm_autoscale {
        pwmconf.set_pwm_autoscale(pwm_autoscale);
        save_config_to.pwm_autoscale = pwm_autoscale;
    }

    if let Some(pwm_autograd) = config.pwm_autograd {
        pwmconf.set_pwm_autograd(pwm_autograd);
        save_config_to.pwm_autograd = pwm_autograd;
    }

    if let Some(pwm_freq) = config.pwm_freq {
        pwmconf.set_pwm_freq(pwm_freq);
        save_config_to.pwm_freq = pwm_freq;
    }

    if let Some(pwm_grad) = config.pwm_grad {
        pwmconf.set_pwm_grad(pwm_grad);
        save_config_to.pwm_grad = pwm_grad;
    }

    if let Some(pwm_lim) = config.pwm_lim {
        pwmconf.set_pwm_lim(pwm_lim);
        save_config_to.pwm_lim = pwm_lim;
    }

    if let Some(pwm_ofs) = config.pwm_ofs {
        pwmconf.set_pwm_ofs(pwm_ofs);
        save_config_to.pwm_ofs = pwm_ofs;
    }

    if let Some(pwm_reg) = config.pwm_reg {
        pwmconf.set_pwm_reg(pwm_reg);
        save_config_to.pwm_reg = pwm_reg;
    }
}

pub fn process_tpowerdown(
    tpowerdown: &mut tmc2209::reg::TPOWERDOWN,
    config: &TMC2209_Config,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    if let Some(tpowerdown_val) = config.tpowerdown {
        tpowerdown.0 = tpowerdown_val;
        save_config_to.tpowerdown = tpowerdown_val;
    }
}

pub fn process_tpwmthrs(
    tpwmthrs: &mut tmc2209::reg::TPWMTHRS,
    config: &TMC2209_Config,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    if let Some(tpwmthrs_val) = config.tpwmthrs {
        tpwmthrs.set(tpwmthrs_val);
        save_config_to.tpwmthrs = tpwmthrs_val;
    }
}

pub fn process_sgthrs(
    sgthrs: &mut tmc2209::reg::SGTHRS,
    config: &TMC2209_Config,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    if let Some(sgthrs_val) = config.sgthrs {
        sgthrs.0 = sgthrs_val;
        save_config_to.sgthrs = sgthrs_val;
    }
}

pub fn process_tcoolthrs(
    tcoolthrs: &mut tmc2209::reg::TCOOLTHRS,
    config: &TMC2209_Config,
    save_config_to: &mut TMC2209_SavedConfig,
) {
    if let Some(toolthrs_val) = config.tcoolthrs {
        tcoolthrs.set(toolthrs_val);
        save_config_to.tcoolthrs = toolthrs_val;
    }
}
