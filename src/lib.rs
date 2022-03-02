// This file is part of "raslib"
// Under the MIT License
// Copyright (c) Antonin Hérault

pub mod gpio;
pub mod l298n;

pub const HIGH: bool = true;
pub const LOW: bool = false;

pub fn sleep(milliseconds: u64) {
    use std::thread;
    use std::time::Duration;
    thread::sleep(Duration::from_millis(milliseconds));
}

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
