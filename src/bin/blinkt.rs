extern crate i2cdev;
extern crate sysfs_gpio;
extern crate tiny_http;
extern crate blinkt;

extern crate marvin;

use std::rc::Rc;
use blinkt::Blinkt;

use marvin::picoborg_rev::PicoBorgRev;
use marvin::robot_server::RobotServer;
use marvin::spare_parts::SpareParts;

fn new_blinkt() -> Option<SpareParts<PicoBorgRev, Blinkt>> {
    match Blinkt::new() {
        Ok(blinkt) => {
            let rb = Rc::new(blinkt);
            Some(SpareParts {
                robot: None,
                led: Some(rb),
            })
        }
        Err(error) => {
            println!("Failed to create Blinkt interface: {:?}", error);
            None
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut blinkt = new_blinkt().expect("Failed to find Blinkt");
    RobotServer::run(&mut blinkt);
}
