// *****************************************************************************
// raslib.hpp
// *****************************************************************************
// PROJECT : raslib
// LICENSE : MIT
// AUTHOR  : Antonin Hérault
// *****************************************************************************

#if !defined(__RASLIB__)
#define __RASLIB__

#define to static_cast // example : to<int>(5.3)

#include <string>
    using string = std::string;

typedef   unsigned int     u_int   ;

namespace Ras 
{
    bool const HIGH {true };
    bool const LOW  {false};

    u_int const FORWARD  {2};
    u_int const BACKWARD {3};
    u_int const STOP     {0};

    int const null {0};
}

namespace ras = Ras;

#endif
// __RASLIB__