// This file is part of raslib
// Copyright (c) Antonin Hérault
// Under the MIT license

#include <tcp/server.hpp>

namespace Ras
{
    Server::Server()
    {
        this->change_ip("000.000.0.00");
        this->change_port(0);
        m_type = NORMAL;

        this->sinit();
    }

    Server::Server(string ip)
    {
        this->change_ip(ip);
        this->change_port(0);
        m_type = NORMAL;

        this->sinit();
    }

    Server::Server(string ip, int port)
    {
        this->change_ip(ip);
        this->change_port(port);
        m_type = NORMAL;

        this->sinit();
    }

    void Server::sinit()
    {
        m_socket = socket(AF_INET, SOCK_STREAM, 0);
        if (m_socket == -1) {
            throw Ras::Exception("Can't create socket !");
        }

        m_adserver.sin_addr.s_addr = inet_addr(m_ip.c_str() );
        m_adserver.sin_family = AF_INET;
        m_adserver.sin_port = htons(m_port);
    }

    void Server::screate(int type)
    {
        m_type = type;

        if (m_type == AUTO)
        {
            this->sbind();
            this->slisten();
        }
    }

    void Server::sbind()
    {
        int ret { bind(m_socket, (sockaddr *)&m_adserver, 
                    sizeof(m_adserver)) };
        if (ret == -1) {
            throw Ras::Exception("Can't bind on " + m_ip);
        }
    }

    void Server::slisten()
    {
        if ( listen(m_socket, 1) == -1 ) {
            throw Ras::Exception("Can't listen on port " + std::to_string(m_port) );
        }
    }

    void Server::saccept()
    {
        socklen_t csize { sizeof(m_adclient) };
        m_sockclient = accept(m_socket, (sockaddr *)&m_adclient, &csize);
    }

    int Server::sget()
    {
        int signal {0};
        recv(m_sockclient, &signal, sizeof(signal), 0);
        return signal;
    }

    void Server::shut()
    {
        close(m_sockclient);
    }

    void Server::sclose()
    {
        close(m_socket);
    }

    void Server::change_ip(string ip)
    {
        m_ip = ip;
        if (m_ip.empty() ) {
            throw Ras::Exception("Ip can't be an empty string !");
        }
    }

    void Server::change_port(int port)
    {
        m_port = port;
    }

    string Server::ip() const
    {
        return m_ip;
    }

    int Server::port() const
    {
        return m_port;
    }
}