extern crate linux_embedded_hal as hal;
extern crate picoborgrev;
extern crate robot_traits;
extern crate tiny_http;
extern crate hcsr04;
extern crate bme280;

use hal::I2cdev;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use hcsr04::measure_time;
use picoborgrev::PicoBorgRev;

fn main() {
    println!("Hello, world!");


    read_temperature();
    read_temperature();

    flash_picoborg_led();

    read_temperature();

    println!("Ping! world!");
    read_distance();
}

fn flash_picoborg_led() {
    let i2c_bus = I2cdev::new(Path::new("/dev/i2c-1")).expect("Unable to create i2c device");
    let mut borg = PicoBorgRev::new(i2c_bus).expect("Unable to create PicoBorgRev");
    for _ in 0..10 {
        borg.set_led(true).unwrap();
        sleep(Duration::from_millis(500));
        borg.set_led(false).unwrap();
        sleep(Duration::from_millis(500));
    }
}

fn read_temperature() {
    use hal::{Delay};
    use bme280::BME280;

    let i2c_bus = I2cdev::new(Path::new("/dev/i2c-1")).expect("Unable to create i2c device");


    // initialize the BME280 using the primary I2C address 0x77
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);

    // initialize the sensor
    bme280.init().unwrap();

    // measure temperature, pressure, and humidity
    let measurements = bme280.measure().unwrap();

    println!("Relative Humidity = {}%", measurements.humidity);
    println!("Temperature = {} deg C", measurements.temperature);
    println!("Pressure = {} pascals", measurements.pressure);
}

fn read_distance() {
    const TRIG: u8 = 6;
    const ECHO: u8 = 19;

    let time_nanosecs = measure_time(TRIG, ECHO).unwrap();
    let time_secs = time_nanosecs as f64 * 0.000_000_001;
    // Calculate speed metres per second by dividing distance by time.
    println!("{}", 0.1 / time_secs);
}