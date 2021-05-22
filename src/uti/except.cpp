// This file is part of raslib
// Copyright (c) Antonin Hérault
// Under the MIT license

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