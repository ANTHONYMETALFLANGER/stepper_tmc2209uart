//! tmc2209uart_stepper
//!
//! A high-level tmc2209 driver control interface
//! based on the mitchmindtree/tmc2209 library
//! and compatible with the stepper library

#![no_std]

pub extern crate critical_section;
pub extern crate embedded_hal;
pub extern crate embedded_io;
pub extern crate fugit;
pub extern crate stepper;
pub extern crate tmc2209;

pub mod implementation;
pub mod structures;
pub mod traits;
pub mod utils;

use crate::structures::{
    base_config::TMC2209_BaseConfig, saved_config::TMC2209_SavedConfig,
};

/// The TMC2209UART driver API
///
/// Users are not expected to use this API directly, except to create an
/// instance using [`TMC2209UART::new_with_config`].
pub struct TMC2209UART<Enable, Fault, Sleep, Reset, UartRef, Step, Dir> {
    enable: Enable,
    fault: Fault,
    shared_uart: UartRef,
    base_config: TMC2209_BaseConfig,
    saved_config: TMC2209_SavedConfig,
    sleep: Sleep,
    reset: Reset,
    step: Step,
    dir: Dir,
}
