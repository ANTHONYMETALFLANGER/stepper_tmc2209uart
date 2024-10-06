#![allow(non_camel_case_types)]
#![allow(missing_docs)]

//. Some values that are not sent to the driver, but are involved in the calculations
pub struct TMC2209_BaseConfig {
    /// You can connect multiple drivers to one uart (see tmc2209 datasheet, page 17, 18)
    pub uart_address: u8,

    /// Sense resistor (see tmc2209 datasheet, page 49)
    pub r_sense: f32,

    /// You can decrease hold current (in comparison to run current) with this multiplier
    pub ihold_multiplier: f32,
}

/// All writable registers for the TMC2209.
pub struct TMC2209_ConfigRegisters {
    pub gconf: Option<tmc2209::reg::GCONF>,
    pub chopconf: Option<tmc2209::reg::CHOPCONF>,
    pub slaveconf: Option<tmc2209::reg::SLAVECONF>,
    pub factory_conf: Option<tmc2209::reg::FACTORY_CONF>,
    pub ihold_irun: Option<tmc2209::reg::IHOLD_IRUN>,
    pub coolconf: Option<tmc2209::reg::COOLCONF>,
    pub pwmconf: Option<tmc2209::reg::PWMCONF>,
    pub tpowerdown: Option<tmc2209::reg::TPOWERDOWN>,
    pub tstep: Option<tmc2209::reg::TSTEP>,
    pub tpwmthrs: Option<tmc2209::reg::TPWMTHRS>,
    pub sgthrs: Option<tmc2209::reg::SGTHRS>,
    pub tcoolthrs: Option<tmc2209::reg::TCOOLTHRS>,
}

/// Full config of tmc2209.
///
/// Possible usage:
/// ```
/// let config = TMC2209_Config {
///     rms_current: Some(700), // 700 mA run current
///     microsteps: Some(16), // 16 microsteps
///     en_spreadcycle: Some(false), // disable spread cycle
///     pwm_autoscale: Some(true), // need for stealthChop
///     sgtrs: Some(120), // 120 sgtrs (can be used for sensorless homing)
///     ..Default::default()
/// };
/// ```
pub struct TMC2209_Config {
    pub uart_address: Option<u8>,
    pub r_sense: Option<f32>,
    pub rms_current: Option<u16>,
    pub ihold_multiplier: Option<f32>,
    pub ihold_delay: Option<u8>,
    pub microsteps: Option<u32>,
    pub interpolation: Option<bool>,
    pub blank_time: Option<u32>,
    pub hysteresis_end: Option<u32>,
    pub hysteresis_start: Option<u32>,
    pub tpowerdown: Option<u32>,
    pub tpwmthrs: Option<u32>,
    pub sgthrs: Option<u32>,
    pub tcoolthrs: Option<u32>,
    pub en_spreadcycle: Option<bool>,
    pub pdn_disable: Option<bool>,
    pub pwm_ofs: Option<u8>,
    pub pwm_grad: Option<u8>,
    pub pwm_freq: Option<u8>,
    pub pwm_autoscale: Option<bool>,
    pub pwm_autograd: Option<bool>,
    pub pwm_reg: Option<u8>,
    pub pwm_lim: Option<u8>,
    pub freewheel: Option<u8>,
    pub internal_rsense: Option<bool>,
    pub i_scale_analog: Option<bool>,
    pub mstep_reg_select: Option<bool>,
    pub multistep_filt: Option<bool>,
    pub index_otpw: Option<bool>,
    pub index_step: Option<bool>,
    pub senddelay: Option<u8>,
    pub semin: Option<u16>,
    pub seup: Option<u16>,
    pub semax: Option<u16>,
    pub sedn: Option<u16>,
    pub seimin: Option<bool>,
    pub toff: Option<u32>,
    pub vsense: Option<bool>,
    pub dedge: Option<bool>,
    pub diss2g: Option<bool>,
    pub diss2vs: Option<bool>,
    pub fclktrim: Option<u8>,
    pub ottrim: Option<u8>,
    pub tstep: Option<u32>,
}

pub struct TMC2209_ConfigRegistersChangesDetected {
    pub gconf: bool,
    pub chopconf: bool,
    pub slaveconf: bool,
    pub otp_prog: bool,
    pub factory_conf: bool,
    pub ihold_irun: bool,
    pub coolconf: bool,
    pub pwmconf: bool,
    pub tpowerdown: bool,
    pub tstep: bool,
    pub tpwmthrs: bool,
    pub sgthrs: bool,
    pub vactual: bool,
    pub tcoolthrs: bool,
}
