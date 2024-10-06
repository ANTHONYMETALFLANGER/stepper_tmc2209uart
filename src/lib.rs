//! tmc2209uart_stepper
//!
//! A high-level tmc2209 driver control interface
//! based on the mitchmindtree/tmc2209 library
//! and compatible with the stepper library

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs, rustdoc::broken_intra_doc_links)]

pub mod driver;
pub mod implementation;
pub mod reg_processor;
pub mod structures;
pub mod traits;
pub mod utils;

pub use self::driver::*;
