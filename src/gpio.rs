// This file is part of "raslib"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;

use crate::Direction;

const PATH: &str = "/sys/class/gpio";

/// Manages a gpio pin to write on (in order set it high or low) and read its
/// value.
#[derive(Debug, Clone, Copy)]
pub struct Gpio {
    /// The GPIO pin managed.
    pin: u32,
    /// Direction of the GPIO pin.
    direction: Direction,
}

impl Gpio {
    /// Initialises the GPIO pin and sets its direction to "out". 
    pub fn new(pin: u32) -> Result<Self, io::Error> {
        Self::with_direction(pin, Direction::Out)
    }
    
    /// Initialises the GPIO pin with the given [`Direction`].
    pub fn with_direction(pin: u32, direction: Direction)-> Result<Self, io::Error> {
        // Checks if the GPIO pin is valid.
        // Raspberry PI devices have only 40 GPIO pins.
        assert!(pin <= 40, "Invalid GPIO pin identifier (must be < 40)");

        // Inits the GPIO pin files.
        Self {
            pin: pin,
            direction,
        }
        .init()
    }

    /// Writes the value to the GPIO value file and then activate or deactivate
    /// the port.
    pub fn write(&self, value: bool) -> Result<(), io::Error> {
        // Opens the value file for the GPIO pin.
        let mut stream = File::create(self.gpio_file("value"))?;
        // Writes the given value as `0` or `1` instead of boolean.
        write!(stream, "{}", value as i8)
    }

    /// Reads the value contained into the GPIO value file and returns it as a
    /// boolean.
    pub fn read(&self) -> Result<bool, io::Error> {
        let now = Instant::now();

        // Opens the value file for the GPIO pin.
        let mut stream = File::open(self.gpio_file("value"))?;

        println!("{:?}", now.elapsed());

        // Reads the file as a string.
        let mut retrieved = String::new();
        stream.read_to_string(&mut retrieved)?;

        println!("{:?}", now.elapsed());


        // Removes the last character which is `\n`.
        retrieved.pop();

        let x = crate::str_to_bool(&retrieved);
        println!("{:?}\n", now.elapsed());
        Ok(x)
    }

    /// Changes the direction of the GPIO pin. Can be either "out" or "in".
    pub fn change_direction(&self, direction: Direction) -> Result<(), io::Error> {
        // Opens or creates then opens the direction file to set the GPIO pin
        // direction.
        let mut stream = File::create(self.gpio_file("direction"))?;
        // Writes the direction for the GPIO pin.
        write!(stream, "{}", direction.to_string())
    }

    /// Returns the GPIO pin number.
    ///
    /// It was specified in [`new`](Gpio::new).
    pub fn pin(&self) -> u32 {
        self.pin
    }
}

impl Gpio {
    /// Initialises the export and direction files for the GPIO pin.
    fn init(self) -> Result<Self, io::Error> {
        // No need to export the gpio port again.
        // Otherwise, it will throw an error.
        if !Path::new(&format!("{}/gpio{}", PATH, self.pin)).exists() {
            // Exports the pin to be able to access to it.
            let mut stream = File::create(format!("{}/export", PATH))?;
            // Writes the pin number in the export file.
            write!(stream, "{}", self.pin)?;
        }

        self.change_direction(self.direction)?;

        // Returns self to be returned by `new`.
        Ok(self)
    }

    /// Gets a specific file path as String from the GPIO directory.
    fn gpio_file(&self, filename: &str) -> String {
        match filename {
            "value" => {}
            "direction" => {}
            _ => panic!(),
        }

        format!("{}/gpio{}/{}", PATH, self.pin, filename)
    }
}
