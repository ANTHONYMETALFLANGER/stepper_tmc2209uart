use crate::structures::registers_collection::{
    TMC2209_ConfigRegisters, TMC2209_ConfigRegistersChangesDetected,
};

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
