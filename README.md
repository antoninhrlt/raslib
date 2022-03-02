# raslib
Raspberry PI library for Rust. GPIO controller, L298N motors, sockets and 
"i2clib" integrated 

<p align="center">
    <img src="https://github.com/antoninhrlt/raslib-cpp/raw/main/share/png_logo.png" width="40%">
</p>

All tests are made on Raspberry PI 4B+ on Raspbian OS

## Overview
 - GPIO controller
 - L298N motors controller (needs the GPIO controller)
> *NOTE* there is no server socket generator because the Rust standard library
> already contains a really good way to create sockets \
> *SEE* [here](#server-socket) to create a server socket

## Install
In your project `Cargo.toml` file, add the "raslib" dependency :
```toml
[dependencies]
raslib = { git = "https://github.com/antoninhrlt/raslib" }
```
And then, build your project : `cargo build`

## Examples
- ## Blink a led 
    ```rust
    use raslib;
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
    And your connections are: (from : [alcalyn.github.io](https://alcalyn.github.io/control-robot-two-engines/))
    ![connection scheme](https://camo.githubusercontent.com/f9567bf527fdda17c1262b4878e33dc1883dc21ba09ecadb0fe0c584631719d4/68747470733a2f2f616c63616c796e2e6769746875622e696f2f6173736574732f696d616765732f7270692d6d6f746f72732f726173702d6c3239386e2e706e67)

## Server socket
From [doc.rust-lang.org](https://doc.rust-lang.org/book/ch20-01-single-threaded.html)
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

            // does things
        }
    }
}
```
