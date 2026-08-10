#include "print_forever_node.hpp"

/**
 * @brief PrintForeverNode::PrintForeverNode Default constructor.
 */
PrintForeverNode::PrintForeverNode():
    rclcpp::Node("print_forever_cpp"),
    timer_period_(0.5),
    print_count_(0)
{
    //(Smart) pointers at the one thing that it doesn't matter much if they are not initialized in the member initializer list
    //and this is a bit more readable.
    timer_ = create_wall_timer(
                std::chrono::milliseconds(long(timer_period_*1e3)),
                std::bind(&PrintForeverNode::_timer_callback, this) //Note here the use of std::bind to build a single argument
                );
}

/**
 * @brief PrintForeverNode::_timer_callback periodically prints class info using RCLCPP_INFO.
 */
void PrintForeverNode::_timer_callback()
{
    RCLCPP_INFO_STREAM(get_logger(),
                       std::string("Printed ") +
                       std::to_string(print_count_) +
                       std::string(" times.")
                       );
    print_count_++;
}