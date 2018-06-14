extern crate linux_embedded_hal;
extern crate picoborgrev;
extern crate robot_traits;
extern crate tiny_http;

pub mod robot_server;

use linux_embedded_hal::I2cdev;
use picoborgrev::PicoBorgRev;
use robot_traits::{Led, Robot};
use std::path::Path;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use robot_server::RobotServer;

pub struct SpareParts<T: Robot, U: Led> {
    pub robot: Option<Rc<T>>,
    pub led: Option<Rc<U>>,
}

fn main() {
    println!("Hello, world!");

    let device = I2cdev::new(Path::new("/dev/i2c-1")).expect("Unable to create i2c device");
    let mut borg = PicoBorgRev::new(device).expect("Unable to create PicoBorgRev");
    for _ in 0..10 {
        borg.set_led(true).unwrap();
        sleep(Duration::from_millis(599));
        borg.set_led(false).unwrap();
        sleep(Duration::from_millis(500));
    }

    let rb = Rc::new(borg);
    let mut borg = SpareParts {
        robot: Some(Rc::clone(&rb)),
        led: Some(Rc::clone(&rb)), // Some(borg),
    };

    RobotServer::run(&mut borg);
}
