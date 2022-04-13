// This file is part of "raslib"
// Under the MIT License
// Copyright (c) Antonin Hérault

use std::fs::File;
use std::io;
use std::io::{Read, Write};

static PATH: &str = "/sys/class/gpio";

#[derive(Copy, Clone)]
pub struct Gpio {
    pin: u32,
}

impl Gpio {
    pub fn new(pin: u32) -> Result<Self, io::Error> {
        if pin > 40 { // Raspberry PI devices have only 40 GPIO pins
            panic!("Invalid GPIO pin identifier (must be < 40)");
        }

        Self { pin: pin }.init()
    }

    /// Init the export and direction files \
    /// Created to be called in the constructor, could not be called by the 
    /// user
    fn init(&self) -> Result<Self, io::Error> {
        let mut stream = File::create(format!("{}/export", PATH))?;
        write!(stream, "{}", self.pin)?;

        let mut stream = File::create(self.gpio_file("direction"))?;
        write!(stream, "out")?;

        Ok(self.clone())
    }

    /// Write the value to the GPIO value file
    pub fn write(&self, value: bool) -> Result<(), io::Error> {
        let mut stream = File::create(self.gpio_file("value"))?;
        write!(stream, "{}", value as i32)?;
        Ok(())
    }

    /// Read the value contained into the GPIO value file
    pub fn read(&self) -> Result<bool, io::Error> {
        let mut stream = File::open(self.gpio_file("value"))?;
        let mut retrieved = String::new();
        stream.read_to_string(&mut retrieved)?;
        Ok(retrieved.parse::<bool>().expect("Invalid GPIO file value"))
    }

    /// Get a specific file path as String from the GPIO directory
    fn gpio_file(&self, filename: &str) -> String {
        match filename {
            "value" => {},
            "direction" => {},
            _ => panic!(),
        }
        format!("{}/gpio{}/{}", PATH, self.pin, filename)
    }

    pub fn pin(&self) -> u32 {
        self.pin
    }
}
