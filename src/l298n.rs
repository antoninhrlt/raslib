// This file is part of "raslib"
// Under the MIT License
// Copyright (c) Antonin Hérault

use crate::Gpio;
use crate::{HIGH, LOW};

use std::io;

/// Controls a L298N circuit motor connected on GPIO ports.
///
/// Get an advanced explanations about L298N circuit motors on
/// https://www.dprg.org/l298n-motor-driver-board-drive-modes/.
#[derive(Debug, Clone, Copy)]
pub struct L298n {
    /// Pin for the direction settings as well as `in2`.
    in1: Gpio,
    /// Pin for the direction settings as well as `in1`.
    in2: Gpio,
    /// Pin to modulate the speed of the motor.
    ena: Gpio,
}

impl L298n {
    /// Creates a new [`L298n`] object for a motor on three GPIO pins from
    /// their number.
    pub fn new(in1: u32, in2: u32, ena: u32) -> Self {
        Self {
            in1: Gpio::new(in1).unwrap(),
            in2: Gpio::new(in2).unwrap(),
            ena: Gpio::new(ena).unwrap(),
        }
    }

    /// Sets the first direction pin to high and the second to low.
    ///
    /// Sets the speed pin to high in order to activate the motor.
    pub fn forward(&mut self) -> Result<(), io::Error> {
        self.in1.write(HIGH)?;
        self.in2.write(LOW)?;
        self.ena.write(HIGH)
    }

    /// Sets the first direction pin to low and the second to high.
    ///
    /// Sets the speed pin to high in order to activate the motor.
    pub fn backward(&mut self) -> Result<(), io::Error> {
        self.in1.write(LOW)?;
        self.in2.write(HIGH)?;
        self.ena.write(HIGH)
    }

    /// Sets the speed pin to low in order to deactivate the motor.
    pub fn stop(&mut self) -> Result<(), io::Error> {
        self.ena.write(LOW)
    }
}
