// *****************************************************************************
// except.cpp
// *****************************************************************************
// PROJECT : raslib
// LICENSE : MIT
// AUTHOR  : Antonin Hérault
// *****************************************************************************

#include <uti/except.hpp>

namespace Ras
{
    Exception::Exception(string what)
    {
        m_what = what;
    }

    const char *Exception::what() const throw()
    {
        return m_what.c_str();
    }
}