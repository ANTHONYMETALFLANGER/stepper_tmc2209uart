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
pub mod reg_processor;
pub mod structures;
pub mod tmc2209_driver;
pub mod utils;

pub use crate::tmc2209_driver::*;
