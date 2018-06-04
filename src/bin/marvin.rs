extern crate i2cdev;
extern crate sysfs_gpio;
extern crate tiny_http;

extern crate marvin;

use std::path::Path;
use std::rc::Rc;

use marvin::picoborg_rev::PicoBorgRev;
use marvin::robot_server::RobotServer;
use marvin::spare_parts::SpareParts;

fn new_borg() -> Option<SpareParts<PicoBorgRev, PicoBorgRev>> {
    match PicoBorgRev::new(Path::new("/dev/i2c-1")) {
        Ok(borg) => {
            let rb = Rc::new(borg);
            Some(SpareParts {
                robot: Some(Rc::clone(&rb)),
                led: Some(Rc::clone(&rb)), // Some(borg),
            })
        }
        Err(error) => {
            println!("Failed to create PicoBorgRev interface: {:?}", error);
            None
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut borg = new_borg().expect("Failed to find PicoBorgRev");
    RobotServer::run(&mut borg);
}
