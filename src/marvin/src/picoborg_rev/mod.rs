use std::thread;
use std::time::Duration;
use std::path::Path;

use i2cdev::core::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const I2C_ADDRESS: u16 = 0x44;

// Constant values
const PWM_MAX: u8                 = 255;
const I2C_MAX_LEN: u8             = 4;

const I2C_ID_PICOBORG_REV: u8     = 0x15;

const COMMAND_SET_LED: u8         = 1;     // Set the LED status
const COMMAND_GET_LED: u8         = 2;     // Get the LED status
const COMMAND_SET_A_FWD: u8       = 3;     // Set motor 2 PWM rate in a forwards direction
const COMMAND_SET_A_REV: u8       = 4;     // Set motor 2 PWM rate in a reverse direction
const COMMAND_GET_A: u8           = 5;     // Get motor 2 direction and PWM rate
const COMMAND_SET_B_FWD: u8       = 6;     // Set motor 1 PWM rate in a forwards direction
const COMMAND_SET_B_REV: u8       = 7;     // Set motor 1 PWM rate in a reverse direction
const COMMAND_GET_B: u8           = 8;     // Get motor 1 direction and PWM rate
const COMMAND_ALL_OFF: u8         = 9;     // Switch everything off
const COMMAND_RESET_EPO: u8       = 10;    // Resets the EPO flag, use after EPO has been tripped and switch is now clear
const COMMAND_GET_EPO: u8         = 11;    // Get the EPO latched flag
const COMMAND_SET_EPO_IGNORE: u8  = 12;    // Set the EPO ignored flag, allows the system to run without an EPO
const COMMAND_GET_EPO_IGNORE: u8  = 13;    // Get the EPO ignored flag
const COMMAND_GET_DRIVE_FAULT: u8 = 14;    // Get the drive fault flag, indicates faults such as short-circuits and under voltage
const COMMAND_SET_ALL_FWD: u8     = 15;    // Set all motors PWM rate in a forwards direction
const COMMAND_SET_ALL_REV: u8     = 16;    // Set all motors PWM rate in a reverse direction
const COMMAND_SET_FAILSAFE: u8    = 17;    // Set the failsafe flag, turns the motors off if communication is interrupted
const COMMAND_GET_FAILSAFE: u8    = 18;    // Get the failsafe flag
const COMMAND_SET_ENC_MODE: u8    = 19;    // Set the board into encoder or speed mode
const COMMAND_GET_ENC_MODE: u8    = 20;    // Get the boards current mode, encoder or speed
const COMMAND_MOVE_A_FWD: u8      = 21;    // Move motor 2 forward by n encoder ticks
const COMMAND_MOVE_A_REV: u8      = 22;    // Move motor 2 reverse by n encoder ticks
const COMMAND_MOVE_B_FWD: u8      = 23;    // Move motor 1 forward by n encoder ticks
const COMMAND_MOVE_B_REV: u8      = 24;    // Move motor 1 reverse by n encoder ticks
const COMMAND_MOVE_ALL_FWD: u8    = 25;    // Move all motors forward by n encoder ticks
const COMMAND_MOVE_ALL_REV: u8    = 26;    // Move all motors reverse by n encoder ticks
const COMMAND_GET_ENC_MOVING: u8  = 27;    // Get the status of encoders moving
const COMMAND_SET_ENC_SPEED: u8   = 28;    // Set the maximum PWM rate in encoder mode
const COMMAND_GET_ENC_SPEED: u8   = 29;    // Get the maximum PWM rate in encoder mode
const COMMAND_GET_ID: u8          = 0x99;  // Get the board identifier
const COMMAND_SET_I2C_ADD: u8     = 0xAA;  // Set a new I2C address

const COMMAND_VALUE_FWD: u8       = 1;     // I2C value representing forward
const COMMAND_VALUE_REV: u8       = 2;     // I2C value representing reverse

const COMMAND_VALUE_ON: u8        = 1;     // I2C value representing on
const COMMAND_VALUE_OFF: u8       = 0;     // I2C value representing off

pub struct PicoBorgRev {
    device: LinuxI2CDevice,
    led_on: bool,
}

impl PicoBorgRev {
    pub fn new(i2c_path: &Path) -> Result<PicoBorgRev, LinuxI2CError> {
        let mut device = LinuxI2CDevice::new(i2c_path, I2C_ADDRESS)?;

        // Check for PicoBorg Reverse
        let data = device.smbus_read_word_data(COMMAND_GET_ID)?;
        let id = (data >> 8) as u8;

        if id != I2C_ID_PICOBORG_REV {
            println!("Found a device at {}, but it is not a PicoBorg Reverse (ID {} instead of {})", I2C_ADDRESS, id, I2C_ID_PICOBORG_REV);
        } else {
            println!("Found PicoBorg Reverse at {}", I2C_ADDRESS);
        }

        Ok(PicoBorgRev { device: device, led_on: false })
    }

    // real code should probably not use unwrap()
    pub fn toggle_led(&mut self) -> Result<bool, LinuxI2CError> {
        self.led_on = !self.led_on;
        println!("Toggling led {}", self.led_on);
        try!(self.device.smbus_write_byte_data(0x01, if self.led_on {COMMAND_VALUE_ON} else {COMMAND_VALUE_OFF}));

        Ok(self.led_on)
    }
}


// real code should probably not use unwrap()
pub fn toggle_led() -> Result<(), LinuxI2CError> {
    let mut dev = try!(LinuxI2CDevice::new("/dev/i2c-1", I2C_ADDRESS));

    thread::sleep(Duration::from_millis(100));

    println!("Toggling led on");
    try!(dev.smbus_write_byte_data(0x01, 0x01));
    thread::sleep(Duration::from_secs(10));
    println!("Toggling led off");
    try!(dev.smbus_write_byte_data(0x01, 0x00)); // On

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
