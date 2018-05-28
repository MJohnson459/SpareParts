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
        let borg_result = PicoBorgRev::new(Path::new("/dev/i2c-1"));
        let borg = match borg_result {
            Ok(borg) => Some(borg),
            Err(error) => {
                println!("Failed to create PicoBorgRev interface: {:?}", error);
                None
            }
        }

        let blinkt_result = Blinkt::new();
        let blinkt = match blinkt_result {
            Ok(blinkt) => Some(blinkt),
            Err(error) => {
                println!("Failed to create Blinkt interface: {:?}", error);
                None
            }
        }

        SpareParts {
            borg: borg,
            blinkt: blinkt,
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
            None => Response::from_string("[blinkt] Blinkt not available"),
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
                    "set_motors" => {
                        let epo_status = borg.get_epo().unwrap();
                        println!("EPO status: {}", epo_status);
                        let power = borg.set_motors(0.5).unwrap();
                        Response::from_string(format!("[borg] motor1 power at: {}", power))
                    },
                    "clear_motors" => {
                        let epo_status = borg.get_epo().unwrap();
                        println!("EPO status: {}", epo_status);
                        let power = borg.set_motors(0.0).unwrap();
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
