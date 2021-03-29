// *****************************************************************************
// utils.cpp
// *****************************************************************************
// PROJECT : raslib
// LICENSE : MIT
// AUTHOR  : Antonin Hérault
// *****************************************************************************

#include <uti/utils.hpp>

namespace Ras
{
    void sleep(u_int time)
    {
        std::this_thread::sleep_for( 
            std::chrono::milliseconds(time) 
        );
    }
}