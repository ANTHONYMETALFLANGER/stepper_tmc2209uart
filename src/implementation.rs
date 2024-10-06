#![allow(missing_docs)]
use crate::structures::{
    TMC2209_BaseConfig, TMC2209_Config, TMC2209_ConfigRegisters,
    TMC2209_ConfigRegistersChangesDetected,
};

impl<'a> Default for TMC2209_BaseConfig {
    fn default() -> Self {
        TMC2209_BaseConfig {
            uart_address: 0,
            r_sense: 0.11, // Default for SilentStepStick series drivers
            ihold_multiplier: 0.5, // Decreas hold current with 50%
        }
    }
}

impl TMC2209_ConfigRegisters {
    pub fn new() -> TMC2209_ConfigRegisters {
        TMC2209_ConfigRegisters {
            gconf: None,
            chopconf: None,
            slaveconf: None,
            factory_conf: None,
            ihold_irun: None,
            coolconf: None,
            pwmconf: None,
            tpowerdown: None,
            tpwmthrs: None,
            sgthrs: None,
            tcoolthrs: None,
        }
    }
}

impl TMC2209_ConfigRegistersChangesDetected {
    pub fn new() -> TMC2209_ConfigRegistersChangesDetected {
        TMC2209_ConfigRegistersChangesDetected {
            gconf: false,
            chopconf: false,
            slaveconf: false,
            factory_conf: false,
            ihold_irun: false,
            coolconf: false,
            pwmconf: false,
            tpowerdown: false,
            tpwmthrs: false,
            sgthrs: false,
            tcoolthrs: false,
        }
    }
}

impl Default for TMC2209_Config {
    fn default() -> Self {
        TMC2209_Config {
            uart_address: None,
            r_sense: None,
            rms_current: None,
            ihold_multiplier: None,
            ihold_delay: None,
            interpolation: None,
            microsteps: None,
            blank_time: None,
            hysteresis_end: None,
            hysteresis_start: None,
            tpowerdown: None,
            tpwmthrs: None,
            sgthrs: None,
            tcoolthrs: None,
            en_spreadcycle: None,
            pdn_disable: None,
            pwm_ofs: None,
            pwm_grad: None,
            pwm_freq: None,
            pwm_autoscale: None,
            pwm_autograd: None,
            pwm_reg: None,
            pwm_lim: None,
            freewheel: None,
            internal_rsense: None,
            i_scale_analog: None,
            mstep_reg_select: None,
            multistep_filt: None,
            index_otpw: None,
            index_step: None,
            senddelay: None,
            semin: None,
            seup: None,
            semax: None,
            sedn: None,
            seimin: None,
            toff: None,
            vsense: None,
            dedge: None,
            diss2g: None,
            diss2vs: None,
            fclktrim: None,
            ottrim: None,
        }
    }
}

impl TMC2209_Config {
    pub fn which_registers_changed(
        &self,
    ) -> TMC2209_ConfigRegistersChangesDetected {
        let mut changes = TMC2209_ConfigRegistersChangesDetected::new();

        changes.gconf = is_gconf_changed(self);
        changes.chopconf = is_chopconf_changed(self);
        changes.ihold_irun = is_ihold_irun_changed(self);
        changes.slaveconf = is_slaveconf_changed(self);
        changes.factory_conf = is_factoryconf_changed(self);
        changes.coolconf = is_coolconf_changed(self);
        changes.pwmconf = is_pwmconf_changed(self);
        changes.tpowerdown = is_tpowerdown_changed(self);
        changes.tpwmthrs = is_tpwmthrs_changed(self);
        changes.sgthrs = is_sgthrs_changed(self);
        changes.tcoolthrs = is_tcoolthrs_changed(self);

        changes
    }
}

// which_registers_changed()

fn is_gconf_changed(config: &TMC2209_Config) -> bool {
    config.i_scale_analog.is_some()
        || config.internal_rsense.is_some()
        || config.en_spreadcycle.is_some()
        || config.index_otpw.is_some()
        || config.index_step.is_some()
        || config.pdn_disable.is_some()
        || config.mstep_reg_select.is_some()
        || config.multistep_filt.is_some()
}

fn is_chopconf_changed(config: &TMC2209_Config) -> bool {
    config.diss2g.is_some()
        || config.diss2g.is_some()
        || config.diss2vs.is_some()
        || config.hysteresis_end.is_some()
        || config.hysteresis_start.is_some()
        || config.hysteresis_start.is_some()
        || config.interpolation.is_some()
        || config.microsteps.is_some()
        || config.blank_time.is_some()
        || config.toff.is_some()
        || config.vsense.is_some()
}

fn is_ihold_irun_changed(config: &TMC2209_Config) -> bool {
    config.ihold_delay.is_some() || config.rms_current.is_some()
}

fn is_slaveconf_changed(config: &TMC2209_Config) -> bool {
    config.senddelay.is_some()
}

fn is_factoryconf_changed(config: &TMC2209_Config) -> bool {
    config.fclktrim.is_some() || config.ottrim.is_some()
}

fn is_coolconf_changed(config: &TMC2209_Config) -> bool {
    config.sedn.is_some()
        || config.semin.is_some()
        || config.seup.is_some()
        || config.semax.is_some()
        || config.seimin.is_some()
}

fn is_pwmconf_changed(config: &TMC2209_Config) -> bool {
    config.freewheel.is_some()
        || config.pwm_autograd.is_some()
        || config.pwm_autoscale.is_some()
        || config.pwm_freq.is_some()
        || config.pwm_grad.is_some()
        || config.pwm_lim.is_some()
        || config.pwm_ofs.is_some()
        || config.pwm_reg.is_some()
}

fn is_tpowerdown_changed(config: &TMC2209_Config) -> bool {
    config.tpowerdown.is_some()
}

fn is_tpwmthrs_changed(config: &TMC2209_Config) -> bool {
    config.tpwmthrs.is_some()
}

fn is_sgthrs_changed(config: &TMC2209_Config) -> bool {
    config.sgthrs.is_some()
}

fn is_tcoolthrs_changed(config: &TMC2209_Config) -> bool {
    config.tcoolthrs.is_some()
}
