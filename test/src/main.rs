use raslib;
use raslib::gpio::Gpio;
use raslib::l298n::L298n;

use std::thread;

fn blink_led() -> Result<(), std::io::Error> {
    let led = Gpio::new(16)?;
    loop {
        led.write(raslib::HIGH)?;
        raslib::sleep(1000);
        
        led.write(raslib::LOW)?;
        raslib::sleep(1000);
    }
}

fn motor_test() -> Result<(), std::io::Error> {
    let mut motor_left = L298n::new(18, 15, 14);
    let mut motor_right = L298n::new(9, 7, 25);

    motor_left.forward()?;
    motor_right.forward()?;

    Ok(())
}

fn main() {
    thread::spawn(|| blink_led().unwrap());
    thread::spawn(|| motor_test().unwrap());
}
