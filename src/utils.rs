#![allow(non_snake_case)]
#![allow(missing_docs)]

use embedded_io::{Read, Write};

// Read register (wait in while loop until response is received)
pub fn read_reg_blocking<
    Reg: tmc2209::reg::ReadableRegister,
    Uart: Read + Write,
>(
    uart: &mut Uart,
    uart_address: u8,
) -> Result<Reg, &str> {
    tmc2209::send_read_request::<Reg, _>(uart_address, uart).expect(
        "Failed to send request to driver. Check tmc2209 uart connection",
    );

    // Wait for response
    let mut reader = tmc2209::Reader::default();
    let mut buff = [0u8; 1];
    while uart.read(&mut buff).is_ok() {
        if let (_, Some(response)) = reader.read_response(&buff) {
            if !response.crc_is_valid() {
                return Err("Error read chopconf. CRC is not valid");
            }

            match response.reg_addr() {
                Ok(_) => {
                    let reg = response.register::<Reg>().unwrap();
                    return Ok(reg);
                }
                _ => return Err("Error read chopconf. Unknown register"),
            }
        }
    }
    return Err("Error read chopconf. Failed to read response");
}

// Write register to tmc2209 driver
pub fn write_reg<Reg: tmc2209::reg::WritableRegister, Uart: Read + Write>(
    uart: &mut Uart,
    uart_address: u8,
    reg: Reg,
) -> Result<(), &str> {
    tmc2209::send_write_request(uart_address, reg, uart).expect(
        "Failed to send request to driver. Check tmc2209 uart connection",
    );
    Ok(())
}

pub fn microstepps_to_mres(microsteps: u32) -> u32 {
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

pub struct RmsCurrentToIholdIrunVsenseOutput {
    pub ihold: u8,
    pub irun: u8,
    pub vsense: bool,
}
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
