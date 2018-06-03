#![allow(dead_code)]

mod impl_traits;

use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};

use i2cdev::core::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const I2C_ADDRESS: u16 = 0x44;

// Constant values
const PWM_MAX: f32 = 255.0;
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

    /// Sets the drive level for motor 1, from +1 to -1.
    pub fn set_motor_1(&mut self, power: f32) -> Result<(), LinuxI2CError> {
        if power < 0.0 {
            self.device.smbus_write_byte_data(COMMAND_SET_A_REV, (PWM_MAX * -power) as u8)
        } else {
            self.device.smbus_write_byte_data(COMMAND_SET_A_FWD, (PWM_MAX * power) as u8)
        }
    }

    /// Sets the drive level for motor 2, from +1 to -1
    pub fn set_motor_2(&mut self, power: f32) -> Result<(), LinuxI2CError> {
        if power < 0.0 {
            self.device.smbus_write_byte_data(COMMAND_SET_B_REV, (PWM_MAX * -power) as u8)
        } else {
            self.device.smbus_write_byte_data(COMMAND_SET_B_FWD, (PWM_MAX * power) as u8)
        }
    }

    /// Sets the drive level for all motors, from +1 to -1.
    pub fn set_motors(&mut self, power: f32) -> Result<(), LinuxI2CError> {
        if power < 0.0 {
            self.device.smbus_write_byte_data(COMMAND_SET_ALL_REV, (PWM_MAX * -power) as u8)
        } else {
            self.device.smbus_write_byte_data(COMMAND_SET_ALL_FWD, (PWM_MAX * power) as u8)
        }
    }

    /// Sets all motors to stopped, useful when ending a program
    pub fn motors_off(&mut self) -> Result<(), LinuxI2CError> {
        try!(self.device.smbus_write_byte_data(COMMAND_ALL_OFF, 0));
        Ok(())
    }

    /// Gets the drive level for motor 1, from +1 to -1.
    pub fn get_motor_1(&mut self) -> Result<f32, LinuxI2CError> {
        let data = self.device.smbus_read_block_data(COMMAND_GET_A)?;
        let power = data[2] as f32 / PWM_MAX;
        match data[1] {
            COMMAND_VALUE_FWD => Ok(power),
            COMMAND_VALUE_REV => Ok(-power),
            _ => Ok(0.0),
        }
    }

    /// Gets the drive level for motor 1, from +1 to -1.
    pub fn get_motor_2(&mut self) -> Result<f32, LinuxI2CError> {
        let data = self.device.smbus_read_block_data(COMMAND_GET_B)?;
        let power = data[2] as f32 / PWM_MAX;
        match data[1] {
            COMMAND_VALUE_FWD => Ok(power),
            COMMAND_VALUE_REV => Ok(-power),
            _ => Ok(0.0),
        }
    }

    /// Sets the current state of the LED, False for off, True for on
    pub fn set_led(&mut self, state: bool) -> Result<(), LinuxI2CError> {
        if state {
            self.device.smbus_write_byte_data(COMMAND_SET_LED, COMMAND_VALUE_ON)
        } else {
            self.device.smbus_write_byte_data(COMMAND_SET_LED, COMMAND_VALUE_OFF)
        }
    }

    /// Reads the current state of the LED, False for off, True for on
    pub fn get_led(&mut self) -> Result<bool, LinuxI2CError> {
        let recv = self.device.smbus_read_block_data(COMMAND_GET_LED)?;
        Ok(recv[1] == COMMAND_VALUE_ON)
    }

    /// Resets the EPO latch state, use to allow movement again after the EPO has been tripped
    pub fn reset_epo(&mut self) -> Result<(), LinuxI2CError> {
        self.device.smbus_write_byte_data(COMMAND_RESET_EPO, 0)
    }

    /// Reads the system EPO latch state.
    /// If False the EPO has not been tripped, and movement is allowed.
    /// If True the EPO has been tripped, movement is disabled if the EPO is not ignored (see SetEpoIgnore)
    ///     Movement can be re-enabled by calling ResetEpo.
    pub fn get_epo(&mut self) -> Result<bool, LinuxI2CError> {
        let data = self.device.smbus_read_block_data(COMMAND_GET_EPO)?;
        Ok(data[1] == COMMAND_VALUE_ON)
    }

    /// Sets the system to ignore or use the EPO latch, set to False if you have an EPO switch, True if you do not
    pub fn set_epo_ignore(&mut self, state: bool) -> Result<(), LinuxI2CError> {
        self.device.smbus_write_byte_data(COMMAND_SET_EPO_IGNORE, if state {COMMAND_VALUE_ON} else {COMMAND_VALUE_OFF})
    }

    /// Reads the system EPO ignore state, False for using the EPO latch, True for ignoring the EPO latch
    pub fn get_epo_ignore(&mut self) -> Result<bool, LinuxI2CError> {
        let data = self.device.smbus_read_block_data(COMMAND_GET_EPO_IGNORE)?;
        Ok(data[1] == COMMAND_VALUE_ON)
    }

    /// Sets the system to enable or disable the communications failsafe
    /// The failsafe will turn the motors off unless it is commanded at least once every 1/4 of a second
    /// Set to True to enable this failsafe, set to False to disable this failsafe
    /// The failsafe is disabled at power on
    pub fn set_comms_failsafe(&mut self, state: bool) -> Result<(), LinuxI2CError> {
        self.device.smbus_write_byte_data(COMMAND_SET_FAILSAFE, if state {COMMAND_VALUE_ON} else {COMMAND_VALUE_OFF})
    }

    /// Read the current system state of the communications failsafe, True for enabled, False for disabled
    /// The failsafe will turn the motors off unless it is commanded at least once every 1/4 of a second
    pub fn get_comms_failsafe(&mut self) -> Result<bool, LinuxI2CError> {
        let data = self.device.smbus_read_block_data(COMMAND_GET_FAILSAFE)?;
        Ok(data[1] == COMMAND_VALUE_ON)
    }

    /// Reads the system drive fault state, False for no problems, True for a fault has been detected
    /// Faults may indicate power problems, such as under-voltage (not enough power), and may be cleared by setting a lower drive power
    /// If a fault is persistent, it repeatably occurs when trying to control the board, this may indicate a wiring problem such as:
    ///     * The supply is not powerful enough for the motors
    ///         The board has a bare minimum requirement of 6V to operate correctly
    ///         A recommended minimum supply of 7.2V should be sufficient for smaller motors
    ///     * The + and - connections for either motor are connected to each other
    ///     * Either + or - is connected to ground (GND, also known as 0V or earth)
    ///     * Either + or - is connected to the power supply (V+, directly to the battery or power pack)
    ///     * One of the motors may be damaged
    /// Faults will self-clear, they do not need to be reset, however some faults require both motors to be moving at less than 100% to clear
    /// The easiest way to check is to put both motors at a low power setting which is high enough for them to rotate easily, such as 30%
    /// Note that the fault state may be true at power up, this is normal and should clear when both motors have been driven
    /// If there are no faults but you cannot make your motors move check GetEpo to see if the safety switch has been tripped
    /// For more details check the website at www.piborg.org/picoborgrev and double check the wiring instructions
    pub fn get_drive_fault(&mut self) -> Result<bool, LinuxI2CError> {
        let data = self.device.smbus_read_block_data(COMMAND_GET_DRIVE_FAULT)?;
        Ok(data[1] == COMMAND_VALUE_ON)
    }

    /// Sets the system to enable or disable the encoder based move mode
    /// In encoder move mode (enabled) the EncoderMoveMotor* commands are available to move fixed distances
    /// In non-encoder move mode (disabled) the SetMotor* commands should be used to set drive levels
    /// The encoder move mode requires that the encoder feedback is attached to an encoder signal, see the website at www.piborg.org/picoborgrev for wiring instructions
    /// The encoder based move mode is disabled at power on
    pub fn set_encoder_move_mode(&mut self, state: bool) -> Result<(), LinuxI2CError> {
        self.device.smbus_write_byte_data(COMMAND_SET_ENC_MODE, if state {COMMAND_VALUE_ON} else {COMMAND_VALUE_OFF})
    }

    /// Read the current system state of the encoder based move mode, True for enabled (encoder moves), False for disabled (power level moves)
    pub fn get_encoder_move_mode(&mut self) -> Result<bool, LinuxI2CError> {
        let data = self.device.smbus_read_block_data(COMMAND_GET_ENC_MODE)?;
        Ok(data[1] == COMMAND_VALUE_ON)
    }

    /// Moves motor 1 until it has seen a number of encoder counts, up to 32767
    /// Use negative values to move in reverse
    pub fn encoder_move_motor_1(&mut self, counts: i16) -> Result<(), LinuxI2CError> {
        if counts < 0 {
            self.device.smbus_write_block_data(COMMAND_MOVE_A_REV, &u16_to_vu8(-counts as u16))
        } else {
            self.device.smbus_write_block_data(COMMAND_MOVE_A_FWD, &u16_to_vu8(counts as u16))
        }
    }

    /// Moves motor 2 until it has seen a number of encoder counts, up to 32767
    /// Use negative values to move in reverse
    pub fn encoder_move_motor_2(&mut self, counts: i16) -> Result<(), LinuxI2CError> {
        if counts < 0 {
            self.device.smbus_write_block_data(COMMAND_MOVE_B_REV, &u16_to_vu8(-counts as u16))
        } else {
            self.device.smbus_write_block_data(COMMAND_MOVE_B_FWD, &u16_to_vu8(counts as u16))
        }
    }

    /// Moves all motors until it has seen a number of encoder counts, up to 32767
    /// Use negative values to move in reverse
    pub fn encoder_move_motors(&mut self, counts: i16) -> Result<(), LinuxI2CError> {
        if counts < 0 {
            self.device.smbus_write_block_data(COMMAND_MOVE_ALL_REV, &u16_to_vu8(-counts as u16))
        } else {
            self.device.smbus_write_block_data(COMMAND_MOVE_ALL_FWD, &u16_to_vu8(counts as u16))
        }
    }

    /// Reads the current state of the encoder motion, False for all motors have finished, True for any motor is still moving
    pub fn is_encoder_moving(&mut self) -> Result<bool, LinuxI2CError> {
        let data = self.device.smbus_read_block_data(COMMAND_GET_ENC_MOVING)?;
        Ok(data[1] == COMMAND_VALUE_ON)
    }

    /// Waits until all motors have finished performing encoder based moves
    /// If the motors stop moving the function will return True
    /// If a timeout is provided the function will return False after timeout seconds if the motors are still in motion
    pub fn wait_while_encoder_moving(&mut self, timeout: Duration) -> bool {

        let now = Instant::now();
        while self.is_encoder_moving().unwrap() {
            sleep(Duration::from_millis(100));
            if now.elapsed() > timeout {
                return false;
            }
        }
        true
    }

    /// Sets the drive limit for encoder based moves, from 0 to 1.
    pub fn set_encoder_speed(&mut self, power: f32) -> Result<(), LinuxI2CError> {
        let pwm: u8 = (PWM_MAX * power) as u8;
        self.device.smbus_write_byte_data(COMMAND_SET_ENC_SPEED, pwm)
    }

    /// Gets the drive limit for encoder based moves, from 0 to 1.
    pub fn get_encoder_speed(&mut self) -> Result<f32, LinuxI2CError> {
        let data = self.device.smbus_read_block_data(COMMAND_GET_ENC_MODE)?;
        Ok(data[1] as f32 / PWM_MAX)
    }
}

impl Drop for PicoBorgRev {
    fn drop(&mut self) {
        let _ = self.motors_off();
    }
}

fn u16_to_vu8(input: u16) -> [u8; 2] {
    let one: u8 = ((input >> 8) & 0xFF) as u8;
    let two: u8 = (input & 0xFF) as u8;
    [one, two]
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
