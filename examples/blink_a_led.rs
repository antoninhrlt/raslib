use raslib::gpio::Gpio;

fn main() -> Result<(), std::io::Error> {
    let led = Gpio::new(16)?;
    
    loop {
        led.write(raslib::HIGH)?;
        raslib::sleep(1000);
        
        led.write(raslib::LOW)?;
        raslib::sleep(1000);
    }
}
