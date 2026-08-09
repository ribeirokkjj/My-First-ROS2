import random
import string

import rclpy
from rclpy.node import Node
from python_package_with_a_library.sample_python_library import SampleClass, sample_function_for_square_of_sum


class NodeThatUsesTheLibrary(Node):
    """A ROS2 Node that prints to the console periodically."""

    def __init__(self):
        super().__init__('node_that_uses_the_library')
        timer_period: float = 0.5
        self.timer = self.create_timer(timer_period, self.timer_callback)

    def timer_callback(self):
        """
        Method that is periodically called by the timer.
        Prints out the result of sample_function_for_square_of_sum of two random numbers,
        followed by the result of SampleClass.get_name() for an instance created with
        a ten-character-long ascii string of random characters.
        """
        a: float = random.uniform(0, 1)
        b: float = random.uniform(1, 2)
        c: float = sample_function_for_square_of_sum(a, b)
        self.get_logger().info(f'sample_function_for_square_of_sum({a},{b}) returned {c}.')

        random_name_ascii: str = ''.join(random.choice(string.ascii_letters) for _ in range(10))
        sample_class_with_random_name = SampleClass(name=random_name_ascii)
        self.get_logger().info(f'sample_class_with_random_name.get_name() '
                               f'returned {sample_class_with_random_name.get_name()}.')


def main(args=None):
    """
    The main function.
    :param args: Not used directly by the user, but used by ROS2 to configure
    certain aspects of the Node.
    """
    try:
        rclpy.init(args=args)

        node_that_uses_the_library = NodeThatUsesTheLibrary()

        rclpy.spin(node_that_uses_the_library)
    except KeyboardInterrupt:
        pass
    except Exception as e:
        print(e)


if __name__ == '__main__':
    main()