#include <raslib/raslib.hpp>
#include <raslib/uti/utils.hpp>
#include <raslib/pin/l298n.hpp>

int main(int argc, char **argv)
{
    Ras::L298n motor_l {14, 18, 15};
    Ras::L298n motor_r {};

    motor_r.define(25, 8, 7);

    while (true)
    {
        motor_l.write(Ras::FORWARD);
        motor_r.write(Ras::FORWARD);
        Ras::sleep(5000); // 5 seconds

        motor_l.write(Ras::STOP);
        motor_r.write(Ras::STOP);
        Ras::sleep(5000);
    }

    return 0;
}