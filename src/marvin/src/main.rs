extern crate tiny_http;
extern crate i2cdev;
extern crate sysfs_gpio;

mod picoborg_rev;
mod blinkt;
mod robot_traits;

use std::io::Cursor;
use std::path::Path;
use std::rc::Rc;
use tiny_http::{Server, Response};

use picoborg_rev::PicoBorgRev;
use blinkt::Blinkt;
use robot_traits::{Robot, Led};

struct SpareParts<T: Robot, U: Led> {
    robot: Option<Rc<T>>,
    led: Option<Rc<U>>,
}

impl SpareParts<PicoBorgRev, PicoBorgRev> {
    fn new_borg() -> Option<Self> {
        match PicoBorgRev::new(Path::new("/dev/i2c-1")) {
            Ok(borg) => {
                let rb = Rc::new(borg);
                Some(SpareParts {
                    robot: Some(Rc::clone(&rb)),
                    led: Some(Rc::clone(&rb)), // Some(borg),
                })
            },
            Err(error) => {
                println!("Failed to create PicoBorgRev interface: {:?}", error);
                None
            }
        }
    }
}

impl SpareParts<PicoBorgRev, Blinkt> {
    fn new_blinkt() -> Option<Self> {
        match Blinkt::new() {
            Ok(blinkt) => {
                let rb = Rc::new(blinkt);
                Some(SpareParts {
                    robot: None,
                    led: Some(rb),
                })
            },
            Err(error) => {
                println!("Failed to create Blinkt interface: {:?}", error);
                None
            }
        }
    }
}

impl<T: Robot, U: Led> SpareParts<T, U> {
    fn run(&mut self) {
        let server = Server::http("0.0.0.0:8000").unwrap();

        for request in server.incoming_requests() {
            println!("received request! method: {:?}, url: {:?}",
                request.method(),
                request.url(),
            );

            let response = {
                let path: Vec<&str> = request.url().split('/').collect();

                // path[0] should always be "" as min path is "/"
                assert_eq!(path[0], "");
                match path[1] {
                    "led" => self.handle_led(&path.as_slice()[1..]),
                    "robot" => self.handle_robot(&path.as_slice()[1..]),
                    _ => Response::from_string(format!("Request not recognised: {:?}", path)),
                }
            };

            request.respond(response);
        }

    }


    fn handle_led(&mut self, request: &[&str]) -> Response<Cursor<Vec<u8>>> {
        let not_found = Response::from_string(format!("[led] Request not recognised: {:?}", request));
        match self.led {
            Some(ref mut led) => if request.len() > 1 {
                let mled = Rc::get_mut(led).unwrap();
                match request[1] {
                    "led_on" => {
                        mled.led_on();
                        Response::from_string(format!("[led] turning LED on"))
                    },
                    "led_off" => {
                        mled.led_off();
                        Response::from_string(format!("[led] turning LED on"))
                    },
                    _ => not_found,
                }
            } else {
                not_found
            },
            None => Response::from_string("[led] LED not available"),
        }
    }

    fn handle_robot(&mut self, request: &[&str]) -> Response<Cursor<Vec<u8>>> {
        // Check robot is available
        let not_found = Response::from_string(format!("[robot] Request not recognised: {:?}", request));
        match self.robot {
            Some(ref mut robot) => if request.len() > 1 {
                let mrobot = Rc::get_mut(robot).unwrap();
                match request[1] {
                    "forward" => {
                        mrobot.forward(0.5);
                        Response::from_string("[robot] moving forward at 0.5")
                    },
                    "stop" => {
                        mrobot.stop();
                        Response::from_string("[robot] stopping robot")
                    },
                    _ => not_found,
                }
            } else {
                not_found
            },
            None => Response::from_string("[robot] Robot not available"),
        }
    }
}


fn main() {
    println!("Hello, world!");

    let mut robot = SpareParts::new_blinkt().expect("Missing PicoBorgRev");
    robot.run();
}
