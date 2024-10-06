#![allow(non_camel_case_types)]
/// Which registers changed by user config
pub struct TMC2209_ConfigRegistersChangesDetected {
    pub gconf: bool,
    pub chopconf: bool,
    pub slaveconf: bool,
    pub factory_conf: bool,
    pub ihold_irun: bool,
    pub coolconf: bool,
    pub pwmconf: bool,
    pub tpowerdown: bool,
    pub tpwmthrs: bool,
    pub sgthrs: bool,
    pub tcoolthrs: bool,
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
    pub tpwmthrs: Option<tmc2209::reg::TPWMTHRS>,
    pub sgthrs: Option<tmc2209::reg::SGTHRS>,
    pub tcoolthrs: Option<tmc2209::reg::TCOOLTHRS>,
}
