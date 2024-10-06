pub fn microsteps_to_mres(microsteps: u32) -> u32 {
    match microsteps {
        256 => 0,
        128 => 1,
        64 => 2,
        32 => 3,
        16 => 4,
        8 => 5,
        4 => 6,
        2 => 7,
        0 => 8,
        _ => 4, // 16 microstep is default
    }
}

pub fn mres_to_microsteps(mres: u32) -> u32 {
    match mres {
        0 => 256,
        1 => 128,
        2 => 64,
        3 => 32,
        4 => 16,
        5 => 8,
        6 => 4,
        7 => 2,
        8 => 0,
        _ => 16, // 16 microstep is default
    }
}

pub fn irun_to_rms_current(irun: u8, vsense: bool, r_sense: f32) -> u16 {
    let vsense_val = if vsense { 0.180 } else { 0.325 };
    ((irun + 1) as f32 / 32.0 * vsense_val / (r_sense + 0.02) / 1.41421
        * 1000.0) as u16
}

pub struct RmsCurrentToIholdIrunVsenseOutput {
    pub ihold: u8,
    pub irun: u8,
    pub vsense: bool,
}

#[allow(non_snake_case)]
pub fn rms_current_to_ihold_irun_vsense(
    rms_current_mA: u16,
    rsense: f32,
    hold_multiplier: f32,
) -> RmsCurrentToIholdIrunVsenseOutput {
    let mut output = RmsCurrentToIholdIrunVsenseOutput {
        ihold: 0,
        irun: 0,
        vsense: false,
    };

    let mut cs: u8 = (32.0 * 1.41421 * rms_current_mA as f32 / 1000.0
        * (rsense + 0.02)
        / 0.325
        - 1.0) as u8;
    // If Current Scale is too low, turn on high sensitivity rsense and calculate again
    if cs < 16 {
        output.vsense = true;
        cs = (32.0 * 1.41421 * rms_current_mA as f32 / 1000.0 * (rsense + 0.02)
            / 0.180
            - 1.0) as u8;
    } else {
        // If cs >= 16, turn off high_sense_r
        output.vsense = false;
    }

    if cs > 31 {
        cs = 31;
    }

    output.irun = cs;
    output.ihold = (cs as f32 * hold_multiplier) as u8;
    output
}
