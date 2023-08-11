// This file is part of "raslib"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to network connections of the device.

use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

/// State of net interfaces.
#[derive(Debug, PartialEq)]
enum NetInterfaceState {
    /// The interface is not used or connected.
    Down,
    /// The interface is connected but not used.
    Dormant,
    /// No information about the interface state.
    Unknown,
    /// The interface is connected and used.
    Up,
}

impl NetInterfaceState {
    /// Creates an [`NetInterfaceState`] object from a string usually retrieved 
    /// by reading the state file of the interface.
    fn from(string: String) -> Self {
        match string.as_str() {
            "down" => Self::Down,
            "dormant" => Self::Dormant,
            "unknown" => Self::Unknown,
            "up" => Self::Up,
            _ => panic!("invalid state for net interface: {}", string), 
        }
    }
}

/// Path to use to manipulate net interfaces.
const PATH: &str = "/sys/class/net";

/// Wether at least one net interface is connected to the Internet.
pub fn is_connected() -> Result<bool, io::Error> {
    // Retrieves all the net interfaces.
    let interfaces = fs::read_dir(PATH)?;

    for interface_dir in interfaces.map(|i| i.unwrap().path()) {
        let interface_path: &Path = interface_dir.as_path();
        let interface_name = interface_path.file_name().unwrap().to_str().unwrap();

        // Checks if the interface is connected to the Internet.
        if is_interface_connected(interface_name)? {
            return Ok(true);
        }
    }

    // No interface is connected to the Internet.
    Ok(false)
}

/// Wether the net interface is connected to the Internet.
pub fn is_interface_connected(interface: &str) -> Result<bool, io::Error> {
    let mut stream = File::open(format!("{}/{}/operstate", PATH, interface))?;

    // Reads the content of the "carrier" file for the net interface.
    let mut retrieved = String::new();
    stream.read_to_string(&mut retrieved)?;

    // Removes the last character which is `\n`.
    retrieved.pop();

    // Retrieves the connection state for the interface.
    let state = NetInterfaceState::from(retrieved);
    Ok(state == NetInterfaceState::Up)
}

#[cfg(test)]
mod tests {
    use super::is_connected;

    #[test]
    fn net() {
        println!("is connected ? {}", is_connected().unwrap());
    }
}
