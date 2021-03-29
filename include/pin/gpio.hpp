// *****************************************************************************
// gpio.hpp
// *****************************************************************************
// PROJECT : raslib
// LICENSE : MIT
// AUTHOR  : Antonin Hérault
// *****************************************************************************

#if !defined(__RASLIB_GPIO__)
#define __RASLIB_GPIO__

#include <raslib.hpp>
#include <uti/except.hpp>
#include <fstream>
#include <string>

namespace Ras
{
    string const GPIO_PATH {"/sys/class/gpio/"};

    /**
     * Control a GPIO pin only with its ID (see https://cutt.ly/dx9edqs)
     * Class can be called in two ways:
     *      ras::gpio(..)    or    Ras::Gpio(..)
     * @attr Gpio::m_pin
    */
    class Gpio
    {
      public:
        /**
         * Create new folder for GPIO pin and new "value" file
         * @param pin - GPIO pin to control
        */
        Gpio(u_int pin);

        /**
         * Open file stream (std::ofstream) to write value in the GPIO file
         * The value is converted from bool to int :
         *      Ras::LOW  : 0
         *      Ras::HIGH : 1 
         * @param value - value to write to GPIO file
        */
        void write(bool value);

        /**
         * Open file stream (std::ifstream) to read value in the GPIO file
         * The value is converted from int to bool :
         *      Ras::LOW  : false
         *      Ras::HIGH : true
         * @return the value read from the GPIO file
        */
        bool read();

        /**
         * @return m_pin
        */
        u_int pin() const;

      private:
        /**
         * Open file stream (std::ofstream) to create new direction file for 
         * GPIO pin, private method
        */
        void direction_file();

        u_int m_pin;
    };

    using gpio = Gpio;
}

#endif
// __RASLIB_GPIO__