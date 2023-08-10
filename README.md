<div align="center">

<img src="assets/raslib.png" align="center" width="15%" alt="raslib logo">

# raslib
[![raslib's crate badge](https://img.shields.io/crates/v/raslib.svg)](https://crates.io/crates/raslib)
[![license badge](https://img.shields.io/github/license/antoninhrlt/raslib)](LICENSE)
[![documentation badge](https://img.shields.io/badge/documentation-docs.rs-blue)](https://docs.rs/raslib/latest/raslib/)

**raslib** is a library to manage **Raspberry PI** devices, written in [Rust](https://rust-lang.org).

It provides GPIO[¹](https://en.wikipedia.org/wiki/General-purpose_input/output) ports acess in order to manage [leds](#blink-a-led) or motors (direct support for [L298N](#l298n-motor) circuit motors).

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
  use raslib::gpio::Gpio;
  ```
  ```rust
  let gpio = Gpio::new(16)?;
  gpio.write(raslib::HIGH)?;

  let pin: u32 = gpio.pin();
  ```

  The Raspberry PI has different GPIO pins following the version. Make sure to  connect to the right numbers.

  > This example is tested on Raspberry PI 4 Model B and the port 16 is not a power (PWR) nor a ground (GND) pin !

  See its [example](#blink-a-led).

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
  use raslib::l298n::L298n;
  ```

  ```rust
  let mut motor_left = L298n::new(18, 15, 14);
  let mut motor_right = L298n::new(9, 7, 25);

  motor_left.forward()?;
  motor_right.forward()?;
  ```

  See its [example](#l298n-motor).

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

## Examples
- ## Blink a led 
    ```rust
    use raslib::gpio::Gpio;

    fn blink_led() -> Result<(), std::io::Error> {
        let led = Gpio::new(16)?;
        loop {
            led.write(raslib::HIGH)?;
            raslib::sleep(1000);
            
            led.write(raslib::LOW)?;
            raslib::sleep(1000);
        }
    }
    ```
- ## L298N motor
    ```rust
    use raslib::l298n::L298n;

    fn forward_forever() -> Result<(), std::io::Error> {
        let mut motor_left = L298n::new(18, 15, 14);
        let mut motor_right = L298n::new(9, 7, 25);

        motor_left.forward()?;
        motor_right.forward()?;

        Ok(())
    }
    ```
    Considering your connection are made this way: (from : [alcalyn.github.io](https://alcalyn.github.io/control-robot-two-engines/))
    ![connection scheme](https://camo.githubusercontent.com/f9567bf527fdda17c1262b4878e33dc1883dc21ba09ecadb0fe0c584631719d4/68747470733a2f2f616c63616c796e2e6769746875622e696f2f6173736574732f696d616765732f7270692d6d6f746f72732f726173702d6c3239386e2e706e67)

## Server socket
Modified from the [Rust docs example](https://doc.rust-lang.org/book/ch20-01-single-threaded.html):
```rust
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn server() {
    let server = TcpListener::bind("<raspi ip>:9000").unwrap();

    for stream in server.incoming() {
        let mut stream: TcpStream = stream.unwrap();

        let mut signal = [0; 1];
        loop {
            stream.read(&mut signal); 
            if signal[0] == 0 {
                break;
            }

            // does things with the raspi
        }
    }
}
```
