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

            let response = {
                let path: Vec<&str> = request.url().split('/').collect();

                // path[0] should always be "" as min path is "/"
                assert_eq!(path[0], "");
                match path[1] {
                    "borg" => self.handle_borg(&path.as_slice()[1..]),
                    _ => Response::from_string(format!("Request not recognised: {:?}", path)),
                }
            };

            request.respond(response);
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
