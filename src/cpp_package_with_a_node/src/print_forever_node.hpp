#pragma once

#include <memory>
#include <rclcpp/rclcpp.hpp>

/**
 * @brief A ROS2 Node that prints to the console periodically, but in C++.
 */
class PrintForeverNode: public rclcpp::Node
{
private:
    double timer_period_;
    int print_count_;
    //also equivalent to rclcpp::TimerBase::SharedPtr
    std::shared_ptr<rclcpp::TimerBase> timer_;

    void _timer_callback();
public:
    PrintForeverNode();

};