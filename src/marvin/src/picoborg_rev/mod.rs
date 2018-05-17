use std::thread;
use std::time::Duration;

use i2cdev::core::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const I2C_ADDRESS: u16 = 0x44;
const I2C_ID: u8 = 0x15;

// This module is designed to communicate with the PicoBorg Reverse
//
// bus                     the smbus object used to talk to the I²C bus
// struct PicoBorgRev {
//     bus_number: u8,   // Check here for Rev 1 vs Rev 2 and select the correct bus
//     i2c_address: u8,  // I²C address, override for a different address
//     device: LinuxI2CDevice,
// }
//
// impl PicoBorgRev {
//     /// bus_number  I²C bus on which the PicoBorg Reverse is attached
//     ///             (Rev 1 is bus 0, Rev 2 is bus 1)
//     /// i2c_address The I²C address of the PicoBorg Reverse chip to control
//     fn init(i2c_path: &Path, bus_number: u8) -> Option<PicoBorgRev> {
//         // self.bus = smbus.SMBus(self.busNumber)
//         let device = LinuxI2CDevice::new(i2c_path, I2C_ADDRESS)?;
//
//         // Check for PicoBorg Reverse
//         let data = device.smbus_read_block_data(COMMAND_GET_ID)?;
//         // i2cRecv = self.bus.read_i2c_block_data(self.i2cAddress, COMMAND_GET_ID, I2C_MAX_LEN)
//         let id = if len(data) == I2C_MAX_LEN {data[1]};
//
//         if id != I2C_ID {
//             println!("Found a device at {}, but it is not a PicoBorg Reverse (ID {} instead of {})", i2c_address, id, I2C_ID);
//             None
//         }
//
//         println!("Found PicoBorg Reverse at {}", I2C_ADDRESS);
//         Some(PicoBorgRev{
//             bus_number: bus_number,
//             i2c_address: I2C_ADDRESS,
//             device: device,
//         })
//     }
// }


// real code should probably not use unwrap()
pub fn toggle_led() -> Result<(), LinuxI2CError> {
    let mut dev = try!(LinuxI2CDevice::new("/dev/i2c-1", I2C_ADDRESS));

    const COMMAND_SET_LED: u8 = 0x01;
    thread::sleep(Duration::from_millis(100));

    let mut is_on = false;

    loop {
        is_on = !is_on;
        let mut buf: [u8; 6] = [0; 6];
        try!(dev.smbus_write_byte_data(0x01, if is_on {0x01} else {0x00})); // On
        thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}