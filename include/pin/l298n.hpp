// *****************************************************************************
// l298n.hpp
// *****************************************************************************
// PROJECT : raslib
// LICENSE : MIT
// AUTHOR  : Antonin Hérault
// *****************************************************************************

#if !defined(__RASLIB_L298N__)
#define __RASLIB_L298N__

#include <raslib.hpp>
#include <pin/gpio.hpp>
#include <vector>

namespace Ras
{
    /**
     * Control a L298N motor thanks to Ras::Gpio class
     * Class can be called in two ways:
     *      ras::l298n(..)    or    Ras::L298n(..)
     * @attr L298n::m_pins
     * @attr L298n::m_gpios
    */
    class L298n
    {
      public:
        /**
         * Init 3 GPIOs for the 3 pins
         * @param ... - GPIO pins to init
        */
        L298n(u_int en, u_int a, u_int b);

        /**
         * Use Ras::Gpio to write new value for the motors
         *      Ras::FORWARD ->
                    en = Ras::HIGH
                    a  = Ras::HIGH
                    b  = Ras::LOW
                Ras::STOP ->
                    en = Ras::LOW
         * @param value - value to write to GPIO file
        */
        void write(u_int value);

        /**
         * @return m_pins
        */
        std::vector<int> pins() const;

        /**
         * @return m_gpios
        */
        std::vector<Ras::Gpio> gpios() const;

      private:
        std::vector<int>       m_pins;
        std::vector<Ras::Gpio> m_gpios;
    };

    using ln298n = L298n;
}

#endif
// __RASLIB_L298N__