#[allow(non_camel_case_types)]
/// Main high-level tmc driver configuration
///
/// Usage example:
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
    pub shaft: Option<bool>,
}
