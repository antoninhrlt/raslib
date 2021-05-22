// This file is part of raslib
// Copyright (c) Antonin Hérault
// Under the MIT license

#include <pin/gpio.hpp>

namespace Ras
{
    Gpio::Gpio(u_int pin)
    {
        m_pin = pin;

        if (m_pin > 40 /* MAX OF GPIO PINS IN RASPBERRY */)
        {
            m_pin = Ras::null;
            throw Ras::Exception("Pin value can't be superior to 40");
        }

        std::ofstream stream {Ras::GPIO_PATH + "export", std::ios::out};
        if (stream.is_open() )
        {
            stream << m_pin;
            stream.close();
        }
        else
        {
            throw Ras::Exception("Can't open export file for GPIO (" 
                + std::to_string(m_pin) + ")");
        }
    }

    void Gpio::write(bool value)
    {
        this->direction_file();

        std::ofstream stream {
            Ras::GPIO_PATH + "gpio" + std::to_string(m_pin) + "/value",
            std::ios::out
        };

        if (stream.is_open() )
        {
            stream << to<int>(value);
            stream.close();
        }
        else
        {
            throw Ras::Exception("Can't open value file for GPIO ("
                + std::to_string(m_pin) + ")");
        }
    }

    bool Gpio::read()
    {
        std::ifstream stream {
            GPIO_PATH + "gpio" + std::to_string(m_pin) + "/value",
            std::ios::out
        };

        if (stream.is_open() )
        {
            short val {0};
            stream >> val;

            return to<bool>(val);
        }
        else
        {
            throw Ras::Exception("Can't open value file for GPIO ("
                + std::to_string(m_pin) + ")");
        }
    }

    u_int Gpio::pin() const
    {
        return m_pin;
    }

    void Gpio::direction_file()
    {
        std::ofstream stream {
            Ras::GPIO_PATH + "gpio" + std::to_string(m_pin) + "/direction",
            std::ios::out
        };

        if (stream.is_open() )
        {
            stream << "out";
            stream.close();
        }
        else
        {
            throw Ras::Exception("Can't open direction file for GPIO ("
                + std::to_string(m_pin) + ")");
        }
    }
}