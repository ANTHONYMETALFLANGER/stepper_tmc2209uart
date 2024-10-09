//! tmc2209uart_stepper
//!
//! A high-level tmc2209 driver control interface
//! based on the mitchmindtree/tmc2209 library
//! and compatible with the stepper library

#![no_std]
#![allow(dead_code)]

pub extern crate critical_section;
pub extern crate embedded_hal;
pub extern crate embedded_io;
pub extern crate tmc2209;

pub mod implementation;
pub mod structures;
pub mod traits;
pub mod utils;

use crate::structures::{
    base_config::TMC2209_BaseConfig, saved_config::TMC2209_SavedConfig,
};
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_io::{Read, Write};

/// The TMC2209UART driver API
///
/// Users are not expected to use this API directly, except to create an
/// instance using [`TMC2209UART::new`].
pub struct TMC2209UART<'a, Uart: Read + Write> {
    shared_uart: &'a Mutex<RefCell<Option<Uart>>>,
    base_config: TMC2209_BaseConfig,
    saved_config: TMC2209_SavedConfig,
}
