#!/usr/bin/env python

import PicoBorgRev
import socket
import struct
import rospy

from geometry_msgs.msg import Twist


class RosBorg():

	def __init__(self):

		self.PBR = PicoBorgRev.PicoBorgRev()
		self.PBR.Init()
		self.PBR.ResetEpo()
		self.PBR.SetCommsFailsafe(True)

		rospy.init_node('controller')

		rospy.Subscriber("cmd_vel", Twist, self.handleCmdVel)

	def handleCmdVel(self, data):

		print('Received data callback:', data)

		speed1 = data.linear.x + data.angular.y
		speed2 = data.linear.x - data.angular.y

		self.PBR.SetMotor1(speed1)
		self.PBR.SetMotor2(speed2)



def main():
	rosBorg = RosBorg()
	rospy.spin()

if __name__ == "__main__":
	main()
