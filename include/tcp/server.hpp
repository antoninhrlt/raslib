// This file is part of raslib
// Copyright (c) Antonin Hérault
// Under the MIT license

#ifndef __RASLIB_SERVER__
#define __RASLIB_SERVER__

#include <raslib.hpp>
#include <uti/except.hpp>

#include <netinet/in.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>

#include <string>

namespace Ras
{
    int const AUTO   {0};
    int const NORMAL {1};

    class Server
    {
        public:
            Server();
            Server(string ip);
            Server(string ip, int port);

            void sinit();

            void screate(int type);

            void sbind();
            void slisten();
            void saccept();
            int  sget();
            void shut();

            void sclose();

            void change_ip(string ip);
            void change_port(int port);

            string ip() const;
            int port() const;
 
        private:
            string m_ip;
            int m_port;
            int m_type;

            struct sockaddr_in m_adserver;
            struct sockaddr_in m_adclient;
            int m_socket;
            int m_sockclient;
    };
    using server = Server;
}

#endif // __RASLIB_SERVER__