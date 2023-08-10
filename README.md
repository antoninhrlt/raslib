<div align="center">

<img src="https://raw.githubusercontent.com/antoninhrlt/raslib/main/assets/raslib.png" align="center" width="15%" alt="raslib logo">

# raslib
[![raslib's crate badge](https://img.shields.io/crates/v/raslib.svg)](https://crates.io/crates/raslib)
[![license badge](https://img.shields.io/github/license/antoninhrlt/raslib)](LICENSE)
[![documentation badge](https://img.shields.io/badge/documentation-docs.rs-blue)](https://docs.rs/raslib/latest/raslib/)

**raslib** is a library to manage **Raspberry PI** devices, written in [Rust](https://rust-lang.org).

It provides GPIO[¹](https://en.wikipedia.org/wiki/General-purpose_input/output) ports access in order to manage [leds](#blink-a-led) or motors (direct support for [L298N](#l298n-motor) circuit motors).

[Installation](#installation) •
[Overview](#overview) •
[Examples](#examples)

</div>

## Installation
In your "Cargo.toml" file :

```toml
[dependencies]
raslib = "*"
```
Check the current version on [crates.io](https://crates.io/crates/raslib).

## Overview
<div align="center">

[GPIO](#gpio) • 
[L298N](#l298n) •
[Utils](#utils)

</div>

- ## GPIO
  The library provides a structure to manipulate GPIO ports simply called `Gpio`.

  ```rust
  #[derive(Copy, Clone)]
  struct Gpio {
      pin: u32,
  }
  ```

  #### Implementation
  
  ```rust
  fn new(pin: u32) -> Result<Self, std::io::Error>;

  fn write(&self, value: bool) -> Result<(), std::io::Error>;

  fn read(&self) -> Result<bool, std::io::Error>;

  fn pin(&self) -> u32;
  ```

  #### Usage

  ```rust
  use raslib::Gpio;
  ```
  ```rust
  let gpio = Gpio::new(16)?;
  gpio.write(raslib::HIGH)?;

  let pin: u32 = gpio.pin();
  ```

  The Raspberry PI has different GPIO pins following the version. Make sure to  connect to the right numbers.

  > This example is tested on Raspberry PI 4 Model B and the port 16 is not a power (PWR) nor a ground (GND) pin !

  See its [example](examples/blink_a_led.rs).

- ## L298N
  The library provides a simple way to manipulate motors using the L298N circuit. Because the motors wires are connected to the GPIO pins, `L298n` actually uses [`Gpio`](#gpio).

  ```rust
  struct L298n {
    in1: Gpio,
    in2: Gpio,
    ena: Gpio,
  }
  ```

  #### Implementation
  
  ```rust
  fn new(in1: u32, in2: u32, ena: u32) -> Self;

  fn forward(&mut self) -> Result<(), io::Error>;

  fn backward(&mut self) -> Result<(), io::Error>;

  fn stop(&mut self) -> Result<(), io::Error>;
  ```

  #### Usage

  ```rust
  use raslib::L298n;
  ```

  ```rust
  let mut motor_left = L298n::new(18, 15, 14);
  let mut motor_right = L298n::new(9, 7, 25);

  motor_left.forward()?;
  motor_right.forward()?;
  ```

  See its [example](examples/motors.rs).

- ## Utils
  The library provides a simple `sleep` function that makes the current thread  wait for a duration in milliseconds.
  
  ```rust
  fn sleep(milliseconds: u64);
  ```
  
  #### Usage
  
  ```rust
  raslib::sleep(1000); // waits 1 second.
  ```

  To make writing values prettier, it also provides two constants:
  
  ```rust
  const HIGH: bool = true;
  const LOW: bool = false;
  ```
  
  #### Usage
  
  ```rust
  gpio.write(raslib::HIGH);
  gpio.write(true); // same as above
  ```
