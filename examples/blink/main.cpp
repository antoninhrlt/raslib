// This file is part of raslib
// Copyright (c) Antonin Hérault
// Under the MIT license

#include <raslib/raslib.hpp>
#include <raslib/pin/gpio.hpp>
#include <raslib/uti/utils.hpp>

void blink(Ras::Gpio *led)
{
    while (true)
    {
        led->write(Ras::HIGH);
        Ras::sleep(1000);

        led->write(Ras::LOW);
        Ras::sleep(1000);
    }
}

int main(int argc, char **argv)
{
    Ras::Gpio led_21 {21};
    blink(&led_21);

    return 0;
}