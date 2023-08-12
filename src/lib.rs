// This file is part of "raslib"
// Under the MIT License
// Copyright (c) Antonin Hérault

//! # raslib
//! To discover the library and its purpose, check the
//! [repository (github.com)](https://github.com/antoninhrlt/raslib).
//!
//! The documentation here is for developers contribute or intend to contribute
//! to the project and curious who want to know how it works.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/antoninhrlt/raslib/main/assets/raslib.png"
)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

mod gpio;
mod l298n;
pub mod net;

pub use gpio::Gpio;
pub use l298n::L298n;

/// Constant to use when writing on pins.
pub const HIGH: bool = true;

/// Constant to use when writing on pins.
pub const LOW: bool = false;

/// Blocks the current thread for a duration in milliseconds.
pub fn sleep(milliseconds: u64) {
    use std::thread;
    use std::time::Duration;
    thread::sleep(Duration::from_millis(milliseconds));
}

/// Converts a string value into a boolean value.
/// 
/// The string must be either "0" or "1".
/// 
/// ## Note
/// This function is different than [str::parse](core::str)! Indeed, it parses 
/// a string that represents a boolean value as a number and not "true" or 
/// "false". 
pub(crate) fn str_to_bool(str: &str) -> bool {
    // Parses string to an u8 value.
    let value = str.parse::<u8>().expect("given str must represent an unsigned integer value");

    // The u8 value must be 0 or 1.
    assert!(value == 0 || value == 1, "invalid cast {} to boolean, value must be '0' or '1'", str);

    // Returns the value as a boolean.
    value != 0
}
