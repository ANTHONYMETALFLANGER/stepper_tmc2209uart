use embedded_io::{Read, Write};

// Read register (wait in while loop until response is received)
pub fn read_reg_blocking<
    Reg: tmc2209::reg::ReadableRegister,
    Uart: Read + Write,
>(
    uart: &mut Uart,
    uart_address: u8,
) -> Result<Reg, ()> {
    let result = tmc2209::send_read_request::<Reg, _>(uart_address, uart);
    if result.is_err() {
        return Err(());
    }

    // Wait for response
    let mut reader = tmc2209::Reader::default();
    let mut buff = [0u8; 1];
    while uart.read(&mut buff).is_ok() {
        if let (_, Some(response)) = reader.read_response(&buff) {
            if !response.crc_is_valid() {
                return Err(());
            }

            match response.reg_addr() {
                Ok(_) => {
                    let reg = response.register::<Reg>().unwrap();
                    return Ok(reg);
                }
                _ => return Err(()),
            }
        }
    }
    return Err(());
}

pub fn test_uart_connection<Uart: Read + Write>(
    uart: &mut Uart,
    uart_address: u8,
) -> bool {
    let result = tmc2209::send_read_request::<tmc2209::reg::DRV_STATUS, _>(
        uart_address,
        uart,
    );
    if result.is_err() {
        return false;
    }

    // Wait for response
    let mut reader = tmc2209::Reader::default();
    let mut buff = [0u8; 1];
    while uart.read(&mut buff).is_ok() {
        if let (_, Some(response)) = reader.read_response(&buff) {
            if response.crc_is_valid() {
                if let Ok(addr) = response.reg_addr() {
                    if addr == tmc2209::reg::Address::DRV_STATUS {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

// Write register to tmc2209 driver
pub fn write_reg<Reg: tmc2209::reg::WritableRegister, Uart: Read + Write>(
    uart: &mut Uart,
    uart_address: u8,
    reg: Reg,
) -> Result<(), ()> {
    let result = tmc2209::send_write_request(uart_address, reg, uart);
    if result.is_err() {
        return Err(());
    }
    Ok(())
}
