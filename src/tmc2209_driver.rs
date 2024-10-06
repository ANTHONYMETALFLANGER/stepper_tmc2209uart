//! TMC2209UART Driver
//!
//! Platform-agnostic driver API for the TMC2209UART stepper motor driver. Can be
//! used on any platform for which implementations of the required
//! [embedded-hal] and [embedded_io] traits are available.

use core::convert::Infallible;

use embedded_hal::digital::OutputPin;
use fugit::NanosDurationU32 as Nanoseconds;

use stepper::{
    step_mode::StepMode256,
    traits::{
        EnableDirectionControl, EnableStepControl, EnableStepModeControl,
        SetDirection, SetStepMode, Step as StepTrait,
    },
};

use crate::structures::TMC2209_BaseConfig;
pub mod tmc2209_impl;
pub mod tmc2209_traits;
pub mod read_write_config_registers;

/// The TMC2209UART driver API
///
/// Users are not expected to use this API directly, except to create an
/// instance using [`TMC2209UART::new`].
pub struct TMC2209UART<Enable, Fault, Sleep, Reset, UartRef, Step, Dir> {
    enable: Enable,
    fault: Fault,
    shared_uart: UartRef,
    base_config: TMC2209_BaseConfig,
    sleep: Sleep,
    reset: Reset,
    step: Step,
    dir: Dir,
}

impl TMC2209UART<(), (), (), (), (), (), ()> {
    /// Create a new instance of `TMC2209UART`
    pub fn new_with_config(base_config: TMC2209_BaseConfig) -> Self {
        Self {
            enable: (),
            fault: (),
            shared_uart: (),
            base_config,
            sleep: (),
            reset: (),
            step: (),
            dir: (),
        }
    }
}

impl<Reset, Step, Dir, OutputPinError> EnableStepModeControl<Reset>
    for TMC2209UART<(), (), (), (), (), Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
{
    type WithStepModeControl = TMC2209UART<(), (), (), Reset, (), Step, Dir>;

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

impl<Reset, Step, Dir, OutputPinError> SetStepMode
    for TMC2209UART<(), (), (), Reset, (), Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
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

impl<Reset, Step, Dir, OutputPinError> EnableDirectionControl<Dir>
    for TMC2209UART<(), (), (), Reset, (), Step, ()>
where
    Dir: OutputPin<Error = OutputPinError>,
{
    type WithDirectionControl = TMC2209UART<(), (), (), Reset, (), Step, Dir>;

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

impl<Reset, Step, Dir, OutputPinError> SetDirection
    for TMC2209UART<(), (), (), Reset, (), Step, Dir>
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

impl<Reset, Uart, Step, Dir, OutputPinError> EnableStepControl<Step>
    for TMC2209UART<(), (), (), Reset, Uart, (), Dir>
where
    Step: OutputPin<Error = OutputPinError>,
{
    type WithStepControl = TMC2209UART<(), (), (), Reset, Uart, Step, Dir>;

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

impl<Reset, Uart, Step, Dir, OutputPinError> StepTrait
    for TMC2209UART<(), (), (), Reset, Uart, Step, Dir>
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
