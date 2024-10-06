//! TMC2209UART Driver
//!
//! Platform-agnostic driver API for the TMC2209UART stepper motor driver. Can be
//! used on any platform for which implementations of the required
//! [embedded-hal] and [embedded_io] traits are available.

use core::convert::Infallible;

use embedded_hal::digital::{OutputPin, PinState};
use fugit::NanosDurationU32 as Nanoseconds;

use crate::{
    step_mode::StepMode256,
    traits::{
        EnableDirectionControl, EnableStepControl, EnableStepModeControl,
        SetDirection, SetStepMode, Step as StepTrait,
    },
};

use super::tmc_specific::tmc2209uart_structures::{
    TMC2209_BaseConfig, TMC2209_Config, TMC2209_ConfigRegisters,
    TMC2209_ConfigRegistersChangesDetected,
};
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_io::{Read, Write}; // Readable and Writable Uart

use stepper::traits::{
    EnableDirectionControl, EnableMotionControl, EnableStepControl,
    EnableStepModeControl, SetDirection, SetStepMode, Step as StepTrait,
};

/// The TMC2209UART driver API
///
/// Users are not expected to use this API directly, except to create an
/// instance using [`TMC2209UART::new`]. Please check out
/// [`Stepper`](crate::Stepper) instead.
///
/// There is a `Uart` parameter here.
/// You are supposed to use a special pettern to share Uart instance
/// between different parts of the code (or tasks, interrupts)
/// (see example: https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/serial_interrupts.rs).
/// Basically the TMC2208UART structure just stores an immutable reference to
/// `Mutex<RefCell<Option<Uart>>>` which is used to get a mutable reference to
/// Uart inside critical_section::with() when needed
pub struct TMC2209UART<'a, Enable, Fault, Sleep, Reset, Uart, Step, Dir> {
    enable: Enable,
    fault: Fault,
    shared_uart: &'a Mutex<RefCell<Option<Uart>>>,
    base_config: TMC2209_BaseConfig,
    sleep: Sleep,
    reset: Reset,
    step: Step,
    dir: Dir,
}

impl<'a, Uart> TMC2209UART<'a, (), (), (), (), Uart, (), ()>
where
    Uart: Read + Write,
{
    /// Create a new instance of `TMC2209UART`
    pub fn new(
        shared_uart: &'a Mutex<RefCell<Option<Uart>>>,
        base_config: TMC2209_BaseConfig,
    ) -> Self {
        Self {
            enable: (),
            fault: (),
            shared_uart,
            base_config,
            sleep: (),
            reset: (),
            step: (),
            dir: (),
        }
    }
}

impl<'a, Reset, Uart, Step, Dir, OutputPinError> EnableStepModeControl<Reset>
    for TMC2209UART<'a, (), (), (), (), Uart, Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
    Uart: Read<Error = OutputPinError> + Write<Error = OutputPinError>,
{
    type WithStepModeControl =
        TMC2209UART<'a, (), (), (), Reset, Uart, Step, Dir>;

    fn enable_step_mode_control(
        self,
        reset: Reset,
    ) -> Self::WithStepModeControl {
        TMC2209UART {
            enable: self.enable,
            fault: self.fault,
            sleep: self.sleep,
            reset,
            shared_uart: self.shared_uart,
            base_config: self.base_config,
            step: self.step,
            dir: self.dir,
        }
    }
}

impl<'a, Reset, Uart, Step, Dir, OutputPinError> SetStepMode
    for TMC2209UART<'a, (), (), (), Reset, Uart, Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
    Uart: Read<Error = OutputPinError> + Write<Error = OutputPinError>,
{
    // Timing Requirements (page 6)
    // https://www.pololu.com/file/0J450/A4988.pdf
    const SETUP_TIME: Nanoseconds = Nanoseconds::from_ticks(200);
    const HOLD_TIME: Nanoseconds = Nanoseconds::from_ticks(200);

    type Error = OutputPinError;
    type StepMode = StepMode256;

    /// This method not set step mode. To change microstepping use
    /// `configure_tmc2209() method`
    fn apply_mode_config(
        &mut self,
        _: Self::StepMode,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn enable_driver(&mut self) -> Result<(), Self::Error> {
        self.reset.set_high()
    }
}

impl<'a, Reset, Uart, Step, Dir, OutputPinError> EnableDirectionControl<Dir>
    for TMC2209UART<'a, (), (), (), Reset, Uart, Step, ()>
where
    Dir: OutputPin<Error = OutputPinError>,
{
    type WithDirectionControl =
        TMC2209UART<'a, (), (), (), Reset, Uart, Step, Dir>;

    fn enable_direction_control(self, dir: Dir) -> Self::WithDirectionControl {
        TMC2209UART {
            enable: self.enable,
            fault: self.fault,
            sleep: self.sleep,
            reset: self.reset,
            shared_uart: self.shared_uart,
            base_config: self.base_config,
            step: self.step,
            dir,
        }
    }
}

impl<'a, Reset, Uart, Step, Dir, OutputPinError> SetDirection
    for TMC2209UART<'a, (), (), (), Reset, Uart, Step, Dir>
where
    Dir: OutputPin<Error = OutputPinError>,
{
    // Timing Requirements (page 6)
    // https://www.pololu.com/file/0J450/TMC2209UART.pdf
    const SETUP_TIME: Nanoseconds = Nanoseconds::from_ticks(200);

    type Dir = Dir;
    type Error = Infallible;

    fn dir(&mut self) -> Result<&mut Self::Dir, Self::Error> {
        Ok(&mut self.dir)
    }
}

impl<'a, Reset, Uart, Step, Dir, OutputPinError> EnableStepControl<Step>
    for TMC2209UART<'a, (), (), (), Reset, Uart, (), Dir>
where
    Step: OutputPin<Error = OutputPinError>,
{
    type WithStepControl = TMC2209UART<'a, (), (), (), Reset, Uart, Step, Dir>;

    fn enable_step_control(self, step: Step) -> Self::WithStepControl {
        TMC2209UART {
            enable: self.enable,
            fault: self.fault,
            sleep: self.sleep,
            reset: self.reset,
            shared_uart: self.shared_uart,
            base_config: self.base_config,
            step,
            dir: self.dir,
        }
    }
}

impl<'a, Reset, Uart, Step, Dir, OutputPinError> StepTrait
    for TMC2209UART<'a, (), (), (), Reset, Uart, Step, Dir>
where
    Step: OutputPin<Error = OutputPinError>,
{
    // Timing Requirements (page 6)
    // https://www.pololu.com/file/0J450/TMC2209UART.pdf
    const PULSE_LENGTH: Nanoseconds = Nanoseconds::from_ticks(1000); // 1µs

    type Step = Step;
    type Error = Infallible;

    fn step(&mut self) -> Result<&mut Self::Step, Self::Error> {
        Ok(&mut self.step)
    }
}
