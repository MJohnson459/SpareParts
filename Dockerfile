FROM osrf/ros:kinetic-robot

RUN apt-get -q update && apt-get install -y \
	# build-essential \
	apt-utils \
	kmod \
	# module-init-tools \
	# net-tools \
	# ifupdown \
	# iputils-ping \
	i2c-tools \
	# usbutils \
	&& apt-get clean && rm -rf /var/lib/apt/lists/*

COPY ./catkin_ws /catkin_ws
WORKDIR /catkin_ws

RUN /ros_entrypoint.sh catkin_make install -DCMAKE_INSTALL_PREFIX=/opt/ros/kinetic/

CMD ["roslaunch", "spare_parts", "spare_parts.launch"]
