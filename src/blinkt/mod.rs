use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Error, Pin};

use robot_traits::Led;

const DAT: u64 = 23;
const CLK: u64 = 24;
const NUM_PIXELS: usize = 8;
const BRIGHTNESS: u8 = 7;

pub struct Blinkt {
    data_pin: Pin,
    clock_pin: Pin,
    pixels: Vec<(u8, u8, u8, u8)>,
}

impl Blinkt {
    pub fn new() -> Result<Blinkt, Error> {
        let data_pin = Pin::new(DAT);
        let clock_pin = Pin::new(CLK);
        data_pin.export()?;
        clock_pin.export()?;
        let pixels = vec![(0, 0, 0, BRIGHTNESS); NUM_PIXELS];
        Ok(Blinkt {data_pin, clock_pin, pixels})
    }

    fn write_byte(&self, mut byte: u8) -> Result<(), Error> {
        for _ in 0..8 {
            self.data_pin.set_value(byte & 0b1000000)?;
            self.clock_pin.set_value(1)?;
            sleep(Duration::new(0, 500));
            byte <<= 1;
            self.clock_pin.set_value(0)?;
            sleep(Duration::new(0, 500));
        }
        Ok(())
    }

    // Emit exactly enough clock pulses to latch the small dark die APA102s which are weird
    // for some reason it takes 36 clocks, the other IC takes just 4 (number of pixels/2)
    fn eof(&self) -> Result<(), Error> {
        self.data_pin.set_value(0)?;
        for _ in 0..36 {
            self.clock_pin.set_value(1)?;
            sleep(Duration::new(0, 500));
            self.clock_pin.set_value(0)?;
            sleep(Duration::new(0, 500));
        }
        Ok(())
    }

    fn sof(&self) -> Result<(), Error> {
        self.data_pin.set_value(0)?;
        for _ in 0..32 {
            self.clock_pin.set_value(1)?;
            sleep(Duration::new(0, 500));
            self.clock_pin.set_value(0)?;
            sleep(Duration::new(0, 500));
        }
        Ok(())
    }

    pub fn show(&self) -> Result<(), Error> {
        self.sof()?;
        for &(r, g, b, brightness) in &self.pixels {
            self.write_byte(0b11100000 | brightness)?;
            self.write_byte(b)?;
            self.write_byte(g)?;
            self.write_byte(r)?;
        }

        self.eof()?;
        Ok(())
    }

    ///  Set the RGB value and optionally brightness of all pixels
    ///  If you don't supply a brightness value, the last value set for each pixel be kept.
    ///  :param r: Amount of red: 0 to 255
    ///  :param g: Amount of green: 0 to 255
    ///  :param b: Amount of blue: 0 to 255
    ///  :param brightness: Brightness: 0.0 to 1.0 (default around 0.2)
    pub fn set_all(&mut self, r: u8, g: u8, b: u8) {
        for x in 0..NUM_PIXELS {
            self.set_pixel(x, r, g, b, 0.3)
        }
    }

    // /// Get the RGB and brightness value of a specific pixel"""
    // pub fn get_pixel(&self, x: u8) -> (u8, u8, u8, f32) {
    //
    //     r, g, b, brightness = pixels[x]
    //     brightness /= 31.0
    //
    //     (r, g, b, round(brightness, 3))
    // }

    /// Set the RGB value, and optionally brightness, of a single pixel
    ///
    /// If you don't supply a brightness value, the last value will be kept.
    ///
    /// :param x: The horizontal position of the pixel: 0 to 7
    /// :param r: Amount of red: 0 to 255
    /// :param g: Amount of green: 0 to 255
    /// :param b: Amount of blue: 0 to 255
    /// :param brightness: Brightness: 0.0 to 1.0 (default around 0.2)
    pub fn set_pixel(&mut self, x: usize, r: u8, g: u8, b: u8, _: f32) {
        // if brightness is None {
        // brightness = pixels[x][3]
        // } else {
        //     brightness = int(31.0 * brightness) & 0b11111
        // }

        self.pixels[x] = (r, g, b, BRIGHTNESS)
    }
}

impl Drop for Blinkt {
    fn drop(&mut self) {
        self.data_pin.unexport().unwrap();
        self.data_pin.unexport().unwrap();
    }
}

impl Led for Blinkt {
    fn led_on(&mut self) {
        self.set_all(255, 255, 255);
        match self.show() {
            Ok(()) => {}
            Err(error) => println!("[blinkt] Error turning on led: {:?}", error)
        }
    }

    fn led_off(&mut self) {
        self.set_all(0, 0, 0);
        match self.show() {
            Ok(()) => {}
            Err(error) => println!("[blinkt] Error turning off led: {:?}", error)
        }
    }
}
