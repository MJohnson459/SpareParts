import smbus

bus = smbus.SMBus(1)
bus.write_quick()
