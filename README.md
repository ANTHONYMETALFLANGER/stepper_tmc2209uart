# stepper_tmc2209uart

A high-level tmc2209 uart control interface based on the mitchmindtree/tmc2209

## Warning

This is an experimental library at an early stage of development. I haven't had time to test it on real hardware yet

## Usage (no_std)

We need Mutex and RefCell for store the Uart instance globally

```rust
use core::cell::RefCell;
use critical_section::Mutex;
```

Declare constant for store the Uart instance

```rust
pub static SERIAL: Mutex<RefCell<Option<Uart<UART2, Blocking>>>> = Mutex::new(RefCell::new(None));
```

tmc2209uart structs we are going to use
```rust
use tmc2209uart::{
    TMC2209UART, 
    structures::{
        base_config::TMC2209_BaseConfig, 
        saved_config::TMC2209_SavedConfig,
        }
};
```

Construct TMC2209UART
```rust
#[entry]
fn main() -> ! {
    // Construct uart instance as described in your platform docs
    // For example for esp32: https://docs.esp-rs.org/esp-hal/esp-hal/0.20.1/esp32/esp_hal/uart/index.html
    // We should get a structure implementing embedded_io::Read and embedded_io::Write traits
    // Then we can put it into our constant using critical_section::with
    critical_section::with(|cs| {
        SERIAL.borrow_ref_mut(cs).replace(tmc_uart);
    });

    // Construct TMC2209UART
    let base_config = TMC2209_BaseConfig {
        uart_address: 0,
        r_sense: 0.11,
        ..Default::default()
    };
    let mut tmc_driver = TMC2209UART::new(&SERIAL, base_config);
    let is_connected = tmc_driver.test_connection();
    if !is_connected {
        panic!("Tmc2209 not connected");
    }

    // Now you can provide detailed configuration to TMC2209 driver using tmc_driver.apply_config()
    // Most of the field names in the config structure are similar to those in the TMCStepper library
    let config = TMC2209_Config {
        rms_current: Some(700), // 700 mA run current
        microsteps: Some(16), // 16 microsteps
        en_spreadcycle: Some(false), // disable spread cycle
        pwm_autoscale: Some(true), // need for stealthChop
        sgthrs: Some(120), // can be used for sensorless homing
        ..Default::default()
    };
    let result = tmc_driver1.apply_config(&config);
    match result {
        Ok(_) => {log::info!("Tmc2209 config applied");}
        Err(_) => {panic!("Something went wrong!");}
    }

    // We can get curent configuration using get_saved_config()
    let saved_config = tmc_driver1.get_saved_config();
    log::info!("{}", saved_config.microsteps == config.microsteps.unwrap());

    // We can also read SG_RESULT, invert shaft, move motor using vactual.
    tmc_driver1.vactual(1000).unwrap();
    tmc_driver1.set_shaft(true).unwrap();
    tmc_driver1.shaft().unwrap(); // Inverts shaft
    log::info!("{}", tmc_driver1.read_sg_result().unwrap());
}
```

## License

This project is open source software, licensed under the terms of the [MIT License]. This basically means you can do anything with the software, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[LICENSE.md]: LICENSE.md
[@ANTHONYMETALFLANGER]: https://github.com/ANTHONYMETALFLANGER
