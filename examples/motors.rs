// This file is part of "raslib"
// Under the MIT License
// Copyright (c) Antonin Hérault

use raslib::L298n;

// Considering the connections are made this way: 
// See https://camo.githubusercontent.com/f9567bf527fdda17c1262b4878e33dc1883dc21ba09ecadb0fe0c584631719d4/68747470733a2f2f616c63616c796e2e6769746875622e696f2f6173736574732f696d616765732f7270692d6d6f746f72732f726173702d6c3239386e2e706e67
// from : alcalyn.github.io, https://alcalyn.github.io/control-robot-two-engines/

fn main() -> Result<(), std::io::Error> {
    let mut motor_left = L298n::new(18, 15, 14);
    let mut motor_right = L298n::new(9, 7, 25);

    motor_left.forward()?;
    motor_right.forward()?;

    Ok(())
}
