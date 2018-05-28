extern crate tiny_http;
extern crate i2cdev;
extern crate sysfs_gpio;

mod picoborg_rev;
mod blinkt;
mod robot_traits;

use std::io::Cursor;
use std::path::Path;
use std::thread;
use std::time::Duration;
use tiny_http::{Server, Response};

use picoborg_rev::PicoBorgRev;
use blinkt::Blinkt;

struct SpareParts {
    borg: Option<PicoBorgRev>,
    blinkt: Option<Blinkt>,
}

impl SpareParts {
    fn new() -> SpareParts {
        SpareParts {
            borg: PicoBorgRev::new(Path::new("/dev/i2c-1")).ok(),
            blinkt: Blinkt::new().ok(),
        }
    }

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
                    "borg" => self.handle_borg(&path.as_slice()[1..]),
                    "blinkt" => self.handle_blinkt(&path.as_slice()[1..]),
                    _ => Response::from_string(format!("Request not recognised: {:?}", path)),
                }
            };

            request.respond(response);
        }

    }


    fn handle_blinkt(&mut self, request: &[&str]) -> Response<Cursor<Vec<u8>>> {
        let not_found = Response::from_string(format!("[blinkt] Request not recognised: {:?}", request));
        match self.blinkt {
            Some(ref mut blinkt) => if request.len() > 1 {
                match request[1] {
                    "strobe_led" => {
                        blinkt.set_all(0, 255, 255);
                        blinkt.show();
                        thread::sleep(Duration::new(2,0));
                        blinkt.set_all(0, 0, 0);
                        blinkt.show();
                        Response::from_string(format!("[blinkt] strobing LEDs"))
                    },
                    _ => not_found,
                }
            } else {
                not_found
            },
            None => Response::from_string("[blinkt] PicoBorgRev not available"),
        }
    }

    fn handle_borg(&mut self, request: &[&str]) -> Response<Cursor<Vec<u8>>> {
        // Check borg is available
        let not_found = Response::from_string(format!("[borg] Request not recognised: {:?}", request));
        match self.borg {
            Some(ref mut borg) => if request.len() > 1 {
                match request[1] {
                    "toggle_led" => {
                        let led_on = borg.toggle_led().unwrap();
                        Response::from_string(format!("[borg] led_on: {}", led_on))
                    },
                    "set_motor1" => {
                        let epo_status = borg.get_epo().unwrap();
                        println!("EPO status: {}", epo_status);
                        let power = borg.set_motor_1(0.5).unwrap();
                        let power = borg.set_motor_2(0.5).unwrap();
                        thread::sleep(Duration::from_secs(10));
                        let power = borg.set_motor_1(0.0).unwrap();
                        let power = borg.set_motor_2(0.0).unwrap();
                        Response::from_string(format!("[borg] motor1 power at: {}", power))
                    },
                    _ => not_found,
                }
            } else {
                not_found
            },
            None => Response::from_string("[borg] PicoBorgRev not available"),
        }
    }
}


fn main() {
    println!("Hello, world!");

    let mut robot = SpareParts::new();
    robot.run();
}
