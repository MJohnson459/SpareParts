# SpareParts

[![Snap Status](https://build.snapcraft.io/badge/MJohnson459/SpareParts.svg)](https://build.snapcraft.io/user/MJohnson459/SpareParts)

This repository contains the source code for a custom robot
built with a Raspberry Pi 3 (with camera) and a PiBorg Reverse motor
controller.

The intention is to integrate this robot with ROS and use monocular
visual odometry to to estimate the position (probably in conjunction
with the ros [robot_localization] package.

This package will contain multiple components which may be split up
in the future:

1. A ROS node to convert the `cmd_vel` topic to PiBorg Reverse commands.
2. A node which estimates position based on `cmd_vel` topic.
3. A launch file which starts up the nodes


This robot will rely on the following packages:
[viso2_ros]



[robot_localization]: http://wiki.ros.org/robot_localization
[viso2_ros]: http://wiki.ros.org/viso2_ros
