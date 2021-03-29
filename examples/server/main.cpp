#include <raslib/raslib.hpp>
#include <raslib/uti/utils.hpp>
#include <raslib/tcp/server.hpp>
#include <raslib/pin/gpio.hpp>

#include <thread>
#include <iostream>

void server()
{
    // CHOOSE YOUR OWN IP
    Ras::Server server {"192.168.1.55", 9999};
    server.screate(ras::AUTO);

    while (true)
    {
        server.saccept();
        int signal {-1};

        while (signal != 0) // signal sent when connection is closed
        {
            signal = server.sget();
            std::cout << "Received : " << signal << std::endl;
        }
        server.shut();
    }
}

void blink(Ras::Gpio *led)
{
    while (true)
    {
        led->write(Ras::HIGH);
        Ras::sleep(1000); // 1 second

        led->write(Ras::LOW);
        Ras::sleep(1000);
    }
}

int main(int argc, char **argv)
{
    std::thread server_th {server};

    Ras::Gpio led_21 {21};
    std::thread blink_th {blink, &led_21};

    server_th.join();
    blink_th.join();

    return 0;
}