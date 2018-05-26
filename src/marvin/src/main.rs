extern crate tiny_http;
extern crate i2cdev;

mod picoborg_rev;

use tiny_http::{Server, Response};
use std::io::Cursor;
use std::path::Path;

use picoborg_rev::PicoBorgRev;

struct SpareParts {
    borg: Option<PicoBorgRev>,
}

impl SpareParts {
    fn new() -> SpareParts {
        SpareParts {
            borg: PicoBorgRev::new(Path::new("/dev/i2c-1")).ok(),
        }
    }

    fn run(&mut self) {
        let server = Server::http("0.0.0.0:8000").unwrap();


        for request in server.incoming_requests() {
            println!("received request! method: {:?}, url: {:?}",
                request.method(),
                request.url(),
            );

            let path: Vec<&str> = request.url().split('/').collect();

            let response = match path[0] {
                "borg" => self.handle_borg(&path.as_slice()[1..]),
                _ => Response::from_string("Request not recognised"),
            };

            request.respond(response);
        }

    }

    fn handle_borg(&mut self, request: &[&str]) -> Response<Cursor<Vec<u8>>> {
        // Check borg is available
        match self.borg {
            Some(borg) => match request[0] {
                "toggle_led" => {
                    let led_on = borg.toggle_led();
                    Response::from_string(format!("led_on: {}", led_on))

                },
                _ => Response::from_string("Request not recognised"),
                },
            None => Response::from_string("PicoBorgRev not available"),
        }
    }
}


fn main() {
    println!("Hello, world!");

    let robot = SpareParts::new();
    robot.run();
}
