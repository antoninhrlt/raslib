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
