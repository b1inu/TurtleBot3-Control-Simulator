import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch_ros.actions import Node

# ROS2 Launch System will look for this function definition #
def generate_launch_description():

    # Get Package Description and Directory #
    package_description = "turtlebot3_realrobot"
    package_directory = get_package_share_directory(package_description)

    # Load RViz Configuration #
    rviz_config_file = "tb3_burger.rviz"
    rviz_config_path = os.path.join(package_directory, "rviz", rviz_config_file)

    # Launch RViz2 with Configuration File #
    rviz_node = Node(
        package="rviz2",
        executable="rviz2",
        name="rviz_node",
        output="screen",
        emulate_tty=True,
        parameters=[{"use_sim_time": False}],
        arguments=["-d", rviz_config_path])

    # create and return launch description object
    return LaunchDescription(
        [
            # declare_use_sim_time,
            rviz_node,
        ]
    )

# End of Code
