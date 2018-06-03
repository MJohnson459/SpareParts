#![allow(dead_code)]

mod impl_traits;

use std::path::Path;

use i2cdev::core::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const I2C_ADDRESS: u16 = 0x44;

// Constant values
const PWM_MAX: u8 = 255;
const I2C_MAX_LEN: u8 = 4;

const I2C_ID_PICOBORG_REV: u8 = 0x15;

const COMMAND_SET_LED: u8 = 1; // Set the LED status
const COMMAND_GET_LED: u8 = 2; // Get the LED status
const COMMAND_SET_A_FWD: u8 = 3; // Set motor 2 PWM rate in a forwards direction
const COMMAND_SET_A_REV: u8 = 4; // Set motor 2 PWM rate in a reverse direction
const COMMAND_GET_A: u8 = 5; // Get motor 2 direction and PWM rate
const COMMAND_SET_B_FWD: u8 = 6; // Set motor 1 PWM rate in a forwards direction
const COMMAND_SET_B_REV: u8 = 7; // Set motor 1 PWM rate in a reverse direction
const COMMAND_GET_B: u8 = 8; // Get motor 1 direction and PWM rate
const COMMAND_ALL_OFF: u8 = 9; // Switch everything off
const COMMAND_RESET_EPO: u8 = 10; // Resets the EPO flag, use after EPO has been tripped and switch is now clear
const COMMAND_GET_EPO: u8 = 11; // Get the EPO latched flag
const COMMAND_SET_EPO_IGNORE: u8 = 12; // Set the EPO ignored flag, allows the system to run without an EPO
const COMMAND_GET_EPO_IGNORE: u8 = 13; // Get the EPO ignored flag
const COMMAND_GET_DRIVE_FAULT: u8 = 14; // Get the drive fault flag, indicates faults such as short-circuits and under voltage
const COMMAND_SET_ALL_FWD: u8 = 15; // Set all motors PWM rate in a forwards direction
const COMMAND_SET_ALL_REV: u8 = 16; // Set all motors PWM rate in a reverse direction
const COMMAND_SET_FAILSAFE: u8 = 17; // Set the failsafe flag, turns the motors off if communication is interrupted
const COMMAND_GET_FAILSAFE: u8 = 18; // Get the failsafe flag
const COMMAND_SET_ENC_MODE: u8 = 19; // Set the board into encoder or speed mode
const COMMAND_GET_ENC_MODE: u8 = 20; // Get the boards current mode, encoder or speed
const COMMAND_MOVE_A_FWD: u8 = 21; // Move motor 2 forward by n encoder ticks
const COMMAND_MOVE_A_REV: u8 = 22; // Move motor 2 reverse by n encoder ticks
const COMMAND_MOVE_B_FWD: u8 = 23; // Move motor 1 forward by n encoder ticks
const COMMAND_MOVE_B_REV: u8 = 24; // Move motor 1 reverse by n encoder ticks
const COMMAND_MOVE_ALL_FWD: u8 = 25; // Move all motors forward by n encoder ticks
const COMMAND_MOVE_ALL_REV: u8 = 26; // Move all motors reverse by n encoder ticks
const COMMAND_GET_ENC_MOVING: u8 = 27; // Get the status of encoders moving
const COMMAND_SET_ENC_SPEED: u8 = 28; // Set the maximum PWM rate in encoder mode
const COMMAND_GET_ENC_SPEED: u8 = 29; // Get the maximum PWM rate in encoder mode
const COMMAND_GET_ID: u8 = 0x99; // Get the board identifier
const COMMAND_SET_I2C_ADD: u8 = 0xAA; // Set a new I2C address

const COMMAND_VALUE_FWD: u8 = 1; // I2C value representing forward
const COMMAND_VALUE_REV: u8 = 2; // I2C value representing reverse

const COMMAND_VALUE_ON: u8 = 1; // I2C value representing on
const COMMAND_VALUE_OFF: u8 = 0; // I2C value representing off

pub struct PicoBorgRev {
    device: LinuxI2CDevice,
}

impl PicoBorgRev {
    pub fn new(i2c_path: &Path) -> Result<PicoBorgRev, LinuxI2CError> {
        let mut device = LinuxI2CDevice::new(i2c_path, I2C_ADDRESS)?;

        // Check for PicoBorg Reverse
        let data = device.smbus_read_word_data(COMMAND_GET_ID)?;
        let id = (data >> 8) as u8;

        if id != I2C_ID_PICOBORG_REV {
            println!(
                "Found a device at {}, but it is not a PicoBorg Reverse (ID {} instead of {})",
                I2C_ADDRESS, id, I2C_ID_PICOBORG_REV
            );
        } else {
            println!("Found PicoBorg Reverse at {}", I2C_ADDRESS);
        }

        Ok(PicoBorgRev {device})
    }

    /// Set motor 1 power.
    /// Range power [-1.0, 1.0]
    pub fn set_motor_1(&mut self, power: f32) -> Result<f32, LinuxI2CError> {
        let command: u8;
        let mut pwm: u8;
        if power < 0.0 {
            command = COMMAND_SET_A_REV;
            pwm = (PWM_MAX as f32 * -power) as u8;
        } else {
            command = COMMAND_SET_A_FWD;
            pwm = (PWM_MAX as f32 * power) as u8;
        }

        if pwm > PWM_MAX {
            pwm = PWM_MAX;
        }

        try!(self.device.smbus_write_byte_data(command, pwm));

        println!("Setting motor 1 power: {}  pwm: {}", power, pwm);
        Ok(power)
    }

    /// Set motor 2 power.
    /// Range power [-1.0, 1.0]
    pub fn set_motor_2(&mut self, power: f32) -> Result<f32, LinuxI2CError> {
        let command: u8;
        let mut pwm: u8;
        if power < 0.0 {
            command = COMMAND_SET_B_REV;
            pwm = (PWM_MAX as f32 * -power) as u8;
        } else {
            command = COMMAND_SET_B_FWD;
            pwm = (PWM_MAX as f32 * power) as u8;
        }

        if pwm > PWM_MAX {
            pwm = PWM_MAX;
        }

        try!(self.device.smbus_write_byte_data(command, pwm));

        println!("Setting motor 1 power: {}  pwm: {}", power, pwm);
        Ok(power)
    }

    /// Sets the drive level for all motors, from +1 to -1.
    pub fn set_motors(&mut self, power: f32) -> Result<f32, LinuxI2CError> {
        let command: u8;
        let mut pwm: u8;
        if power < 0.0 {
            command = COMMAND_SET_ALL_REV;
            pwm = (PWM_MAX as f32 * -power) as u8;
        } else {
            command = COMMAND_SET_ALL_FWD;
            pwm = (PWM_MAX as f32 * power) as u8;
        }

        if pwm > PWM_MAX {
            pwm = PWM_MAX;
        }

        try!(self.device.smbus_write_byte_data(command, pwm));

        println!("Setting motor 1 power: {}  pwm: {}", power, pwm);
        Ok(power)
    }

    /// Sets all motors to stopped, useful when ending a program
    pub fn motors_off(&mut self) -> Result<(), LinuxI2CError> {
        try!(self.device.smbus_write_byte_data(COMMAND_ALL_OFF, 0));
        Ok(())
    }

    /// Gets the drive level for motor 1, from +1 to -1.
    pub fn get_motor_1(&mut self) -> Result<f32, LinuxI2CError> {
        let data = self.device.smbus_read_block_data(COMMAND_GET_A)?;
        let power = data[2] as f32 / PWM_MAX as f32;
        match data[1] {
            COMMAND_VALUE_FWD => Ok(power),
            COMMAND_VALUE_REV => Ok(-power),
            _ => Ok(0.0),
        }
    }

    /// Gets the drive level for motor 1, from +1 to -1.
    pub fn get_motor_2(&mut self) -> Result<f32, LinuxI2CError> {
        let data = self.device.smbus_read_block_data(COMMAND_GET_B)?;
        let power = data[2] as f32 / PWM_MAX as f32;
        match data[1] {
            COMMAND_VALUE_FWD => Ok(power),
            COMMAND_VALUE_REV => Ok(-power),
            _ => Ok(0.0),
        }
    }

    /// Resets the EPO latch state, use to allow movement again after the EPO has been tripped
    pub fn reset_epo(&mut self) -> Result<(), LinuxI2CError> {
        try!(self.device.smbus_write_byte_data(COMMAND_RESET_EPO, 0));

        Ok(())
    }

    /// state = GetEpo()
    ///
    /// Reads the system EPO latch state.
    /// If False the EPO has not been tripped, and movement is allowed.
    /// If True the EPO has been tripped, movement is disabled if the EPO is not ignored (see SetEpoIgnore)
    ///     Movement can be re-enabled by calling ResetEpo.
    pub fn get_epo(&mut self) -> Result<bool, LinuxI2CError> {
        let data = self.device.smbus_read_word_data(COMMAND_GET_EPO)?;
        let status = (data >> 8) as u8;

        Ok(status == COMMAND_VALUE_ON)
    }

    pub fn led_on(&mut self) -> Result<(), LinuxI2CError> {
        self.device.smbus_write_byte_data(COMMAND_SET_LED, COMMAND_VALUE_ON)
    }

    pub fn led_off(&mut self) -> Result<(), LinuxI2CError> {
        self.device.smbus_write_byte_data(COMMAND_SET_LED, COMMAND_VALUE_OFF)
    }
}

impl Drop for PicoBorgRev {
    fn drop(&mut self) {
        let _ = self.motors_off();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
