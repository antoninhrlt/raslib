// This file is part of "raslib"
// Under the MIT License
// Copyright (c) Antonin Hérault

use crate::{HIGH, LOW};
use crate::gpio::Gpio;

use std::io;

/// SEE https://www.dprg.org/l298n-motor-driver-board-drive-modes/
/// The GPIO pins `in1` and `in2` are for the direction settings \
/// The GPIO pin `ena` is to modulate the speed
pub struct L298n {
    in1: Gpio,
    in2: Gpio,
    ena: Gpio,
}

impl L298n {
    pub fn new(in1: u32, in2: u32, ena: u32) -> Self {
        Self {
            in1: Gpio::new(in1).unwrap(), 
            in2: Gpio::new(in2).unwrap(), 
            ena: Gpio::new(ena).unwrap(),
        }
    }

    /// According to this configuration :
    /// - in1: HIGH
    /// - in2: LOW
    /// - ena: HIGH
    pub fn forward(&mut self) -> Result<(), io::Error> {
        self.in1.write(HIGH)?;
        self.in2.write(LOW)?;
        self.ena.write(HIGH)?;
        Ok(())
    }

    /// According to this configuration :
    /// - in1: LOW
    /// - in2: HIGH
    /// - ena: HIGH
    pub fn backward(&mut self) -> Result<(), io::Error> {
        self.in1.write(LOW)?;
        self.in2.write(HIGH)?;
        self.ena.write(HIGH)?;
        Ok(())
    }
    
    /// According to this configuration :
    /// - ena: LOW
    pub fn stop(&mut self) -> Result<(), io::Error> {
        self.ena.write(LOW)?;
        Ok(())
    }
}
