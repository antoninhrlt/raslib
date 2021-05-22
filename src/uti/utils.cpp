// This file is part of raslib
// Copyright (c) Antonin Hérault
// Under the MIT license

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