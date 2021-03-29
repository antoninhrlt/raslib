// *****************************************************************************
// except.hpp
// *****************************************************************************
// PROJECT : raslib
// LICENSE : MIT
// AUTHOR  : Antonin Hérault
// *****************************************************************************

#if !defined(__RASLIB_EXCEPT__)
#define __RASLIB_EXCEPT__

#include <raslib.hpp>
#include <stdexcept>
#include <string>

namespace Ras
{
    class Exception : public std::exception
    {
      public:
        Exception(string what);
        const char *what() const throw();

      private:
        string m_what;
    };

	using exception = Exception;
}

#endif
// __RASLIB_EXCEPT__