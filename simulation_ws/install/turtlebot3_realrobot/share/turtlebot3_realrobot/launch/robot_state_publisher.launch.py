import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch_ros.actions import Node

# ROS2 Launch System will look for this function definition #
def generate_launch_description():

    # Get Package Description and Directory #
    package_description = "turtlebot3_realrobot"
    package_directory = get_package_share_directory(package_description)
    
    # Load Robot Description File #
    urdf_file_name = "turtlebot3_burger.urdf"
    urdf_file_path = os.path.join(
        get_package_share_directory("turtlebot3_description"),
        "urdf", urdf_file_name)
    with open(urdf_file_path, 'r') as urdf_file:
        robot_description = urdf_file.read()
    
    # Robot State Publisher #
    robot_state_publisher = Node(
        package="robot_state_publisher",
        executable="robot_state_publisher",
        name="robot_state_publisher",
        output="screen",
        parameters=[{"use_sim_time": False,
                     "robot_description": robot_description}]
    )

    # create and return launch description object
    return LaunchDescription(
        [
            robot_state_publisher,
        ]
    )

# End of Code
