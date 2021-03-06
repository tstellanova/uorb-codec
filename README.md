# uorb-codec

[![Build Status](https://badge.buildkite.com/2fb7b3d2f3ae1695edab4d2cd18b9b7da282c2d6c3d2144764.svg)](https://buildkite.com/droneflow/uorb-codec-ci)

## What is uORB
[uORB is a publish/subscribe message bus](https://dev.px4.io/en/middleware/uorb.html)
used on the PX4 robotics platform.

## Project Purpose
This project provides rust tools for encoding and decoding uORB messages directly. 
Traditionally uORB messages are only created using C/C++ in the [PX4/Firmware code](https://github.com/PX4/Firmware),
and additional middleware such as 
[ROS message mapping](https://github.com/PX4/px4_msgs)
and an onboard [microRTPS bridge](https://dev.px4.io/zh/middleware/micrortps.html)
are needed to communicate between the native onboard message bus and, say, an external controller or simulator. 

Note that this project does not provide a way to inject uORB messages into the onboard message bus. 

