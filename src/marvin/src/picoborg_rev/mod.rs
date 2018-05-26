// use std::thread;
// use std::time::Duration;
use std::path::Path;

use i2cdev::core::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const I2C_ADDRESS: u16 = 0x44;

// Constant values
const PWM_MAX: u8                 = 255;
const I2C_MAX_LEN: u8             = 4;

const I2C_ID_PICOBORG_REV: u8     = 0x15;

#[repr(u8)]
enum Command {
    SET_LED         = 1,     // Set the LED status
    GET_LED         = 2,     // Get the LED status
    SET_A_FWD       = 3,     // Set motor 2 PWM rate in a forwards direction
    SET_A_REV       = 4,     // Set motor 2 PWM rate in a reverse direction
    GET_A           = 5,     // Get motor 2 direction and PWM rate
    SET_B_FWD       = 6,     // Set motor 1 PWM rate in a forwards direction
    SET_B_REV       = 7,     // Set motor 1 PWM rate in a reverse direction
    GET_B           = 8,     // Get motor 1 direction and PWM rate
    ALL_OFF         = 9,     // Switch everything off
    RESET_EPO       = 10,    // Resets the EPO flag, use after EPO has been tripped and switch is now clear
    GET_EPO         = 11,    // Get the EPO latched flag
    SET_EPO_IGNORE  = 12,    // Set the EPO ignored flag, allows the system to run without an EPO
    GET_EPO_IGNORE  = 13,    // Get the EPO ignored flag
    GET_DRIVE_FAULT = 14,    // Get the drive fault flag, indicates faults such as short-circuits and under voltage
    SET_ALL_FWD     = 15,    // Set all motors PWM rate in a forwards direction
    SET_ALL_REV     = 16,    // Set all motors PWM rate in a reverse direction
    SET_FAILSAFE    = 17,    // Set the failsafe flag, turns the motors off if communication is interrupted
    GET_FAILSAFE    = 18,    // Get the failsafe flag
    SET_ENC_MODE    = 19,    // Set the board into encoder or speed mode
    GET_ENC_MODE    = 20,    // Get the boards current mode, encoder or speed
    MOVE_A_FWD      = 21,    // Move motor 2 forward by n encoder ticks
    MOVE_A_REV      = 22,    // Move motor 2 reverse by n encoder ticks
    MOVE_B_FWD      = 23,    // Move motor 1 forward by n encoder ticks
    MOVE_B_REV      = 24,    // Move motor 1 reverse by n encoder ticks
    MOVE_ALL_FWD    = 25,    // Move all motors forward by n encoder ticks
    MOVE_ALL_REV    = 26,    // Move all motors reverse by n encoder ticks
    GET_ENC_MOVING  = 27,    // Get the status of encoders moving
    SET_ENC_SPEED   = 28,    // Set the maximum PWM rate in encoder mode
    GET_ENC_SPEED   = 29,    // Get the maximum PWM rate in encoder mode
    GET_ID          = 0x99,  // Get the board identifier
    SET_I2C_ADD     = 0xAA,  // Set a new I2C address
}

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
        let data = device.smbus_read_word_data(Command::GET_ID)?;
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
