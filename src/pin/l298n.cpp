// This file is part of raslib
// Copyright (c) Antonin Hérault
// Under the MIT license

#include <pin/l298n.hpp>

namespace Ras
{
    L298n::L298n(u_int en, u_int a, u_int b)
    {
        m_pins[0] = en;
        m_pins[1] = a ;
        m_pins[2] = b ;

        for (size_t i {0}; i < m_pins.size(); i++) {
            m_gpios[i] = Ras::Gpio( m_pins[i] );
        }
    }

    void L298n::write(u_int value)
    {
        if (value == Ras::FORWARD)
        {
            m_gpios[0].write(Ras::HIGH);
            m_gpios[1].write(Ras::HIGH);
            m_gpios[2].write(Ras::LOW);
        }
        else if (value == Ras::STOP) {
            m_gpios[0].write(Ras::LOW);
        }
        else if (value == Ras::BACKWARD)
        {
            throw Ras::Exception(
            "SORRY, raslib don't support this action. Please wait for updates"
            );
        }
        else {
            throw Ras::Exception("Value must be FORWARD, BACKWARD or STOP");
        }
    }

    std::vector<int> L298n::pins() const
    {
        return m_pins;
    }

    std::vector<Ras::Gpio> L298n::gpios() const
    {
        return m_gpios;
    }
}