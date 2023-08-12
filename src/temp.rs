// This file is part of "raslib"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to device's temperatures.

use std::{io::{self, Read}, fs::File};

const PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

/// Returns the current temperature in degrees given by the default thermal 
/// sensor of the Raspberry PI.
pub fn get_current() -> Result<f32, io::Error> {
    let mut stream = File::open(PATH)?;

    // Reads the content of the "carrier" file for the net interface.
    let mut retrieved = String::new();
    stream.read_to_string(&mut retrieved)?;

    // Removes the last character which is `\n`.
    retrieved.pop();

    // Gets the current temperature formatted in millidegree celsius.
    let temp = retrieved.parse::<u32>().expect("Invalid temperature file for default thermal sensor");

    Ok(temp as f32 / 1000.0)
}

#[test]
fn get_current_temp() {
    let temp: f32 = get_current().unwrap();
    println!("{}", temp);
}