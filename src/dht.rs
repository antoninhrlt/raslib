// This file is part of "raslib"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Anything related to DHT sensors (either DHT22 or DHT11).
//!
//! **DHT** stands for **D**igital-output relative **H**umidity &
//! **T**emperature sensor/module.

use std::{
    io,
    os::linux::raw,
    thread,
    time::{self, Duration, Instant, SystemTime},
};

use crate::{Direction, Gpio};

/// Values returned by a [`Dht`], structured.
#[derive(Debug)]
pub struct Data {
    /// The temperature returned by the DHT sensor.
    pub temperature: f32,
    /// The humidity value returned by the DHT sensor.
    pub humidity: f32,
}

impl Data {
    fn new() -> Self {
        Self {
            temperature: 0.0,
            humidity: 0.0,
        }
    }
}

/// Manages a DHT22 or DHT11 sensor to read the given values.
#[derive(Debug)]
pub struct Dht {
    /// Either "22" or "11" depending on the kind of DHT sensor.
    sensor: u32,
    /// GPIO pin that receives the data of the sensor from its "out" pin.
    rdata: Gpio,
}

impl Dht {
    /// Creates a new [`DHT`] object for a specific sensor with data pin
    /// connected to the given GPIO pin.
    pub fn new(sensor: u32, pin: u32) -> Result<Self, io::Error> {
        // Only DHT11 and DHT22 exist.
        assert!(
            sensor == 22 || sensor == 11,
            "The DHT sensor must be DHT22 or DHT11 but found DHT{}",
            sensor
        );

        Ok(Self {
            sensor: sensor,
            rdata: Gpio::with_direction(pin, Direction::In)?,
        })
    }

    /// Reads the data that the GPIO pin receives and returns it as [`DHTData`]
    /// which is basically containing the temperature and humidity received
    /// from the sensor.
    pub fn read(&self) -> Result<self::Data, io::Error> {
        // Works differently following the sensor.
        match self.sensor {
            22 => self.read_22(),
            11 => self.read_11(),
            _ => panic!(), // never happens
        }
    }
}

impl Dht {
    /// Single-bus data is used for communication between the Raspberry PI and
    /// the DHT22 sensor. It costs 5mS for single time communication.
    ///
    /// The sensor sends out higher data bit firstly.
    ///
    /// ## Links
    /// https://www.waveshare.com/wiki/DHT22_Temperature-Humidity_Sensor
    fn read_22(&self) -> Result<self::Data, io::Error> {
        // Sets direction to "out" to write a "low".
        self.rdata.change_direction(Direction::Out)?;
        self.rdata.write(crate::LOW)?;
        // Needs to wait more than 800μs.
        crate::sleep(18);

        self.rdata.write(crate::HIGH)?;

        // Comes back to "in" to release the bus.
        self.rdata.change_direction(Direction::In)?;
        // The GPIO pin goes "high". 
        
        // Now the bus is released, the sensor sends out a response: "low" 
        // for 80ms. Then, it outputs a "high" for 80ms.

        let mut data: Data = Data::new();
        let mut raw_data: u16 = 0;

        // The sensor sends a string of 40 bits of serial data continuously.
        for i in 0..80 {
            let mut live: f32;

            let start_time = Instant::now();

            loop {
                live = (Instant::now() - start_time).as_secs_f32();
                println!("live == {} (0.00009)", live);

                if live > 90.0 / 1000000.0 {
                    // return Err(io::Error::new(
                    //     io::ErrorKind::TimedOut,
                    //     "take too much time to read data",
                    // ));
                }

                let val = self.rdata.read()?;
                println!("({} % 2 != 0) == {}; rdata.read() = {}", i, i % 2 != 0, val);

                // Note: (i % 2 != 0) == (i & 1)
                if val == (i % 2 != 0) {
                    break;
                }
            }

            if i >= 0 && (i % 2 != 0) {
                raw_data <<= 1;

                if live > 30.0 / 1000000.0 {
                    raw_data |= 1;
                }
            }

            match i {
                31 => data.humidity = raw_data as f32,
                63 => data.temperature = raw_data as f32,
                79 => break,
                _ => {}
            }
        }

        // Then it remains "low" for 50ms, switches to "in" and goes "high".

        // The sensors goes to sleep and will wake up next time we send a start
        // signal.

        Ok(data)
    }

    fn read_11(&self) -> Result<self::Data, io::Error> {
        todo!()
    }
}
