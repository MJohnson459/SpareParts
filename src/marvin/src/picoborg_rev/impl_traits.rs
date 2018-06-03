#![allow(dead_code)]

use picoborg_rev::PicoBorgRev;
use robot_traits::{Led, Robot};

impl Robot for PicoBorgRev {
    fn backward(&mut self, speed: f32) {
        let _ = self.set_motors(-speed);
    }

    fn forward(&mut self, speed: f32) {
        let _ = self.set_motors(speed);
    }

    fn left(&mut self, speed: f32) {
        let _ = self.set_motor_1(speed);
        let _ = self.set_motor_2(-speed);
    }

    fn right(&mut self, speed: f32) {
        let _ = self.set_motor_1(-speed);
        let _ = self.set_motor_2(speed);
    }

    fn reverse(&mut self) {}

    fn stop(&mut self) {
        let _ = self.set_motors(0.0);
    }
}

impl Led for PicoBorgRev {
    fn led_on(&mut self) {
        let _ = self.led_on();
    }

    fn led_off(&mut self) {
        let _ = self.led_off();
    }
}

