extern crate tiny_http;
extern crate i2cdev;

mod picoborg_rev;

use tiny_http::{Server, Response};
#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::LinuxI2CError;
use std::error::Error;
use std::path::Path;


use picoborg_rev::PicoBorgRev;

fn main() {
    println!("Hello, world!");

    let server = Server::http("0.0.0.0:8000").unwrap();

    let mut borg = PicoBorgRev::new(Path::new("/dev/i2c-1")).unwrap();

    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let response = match borg.toggle_led() {
            Ok(_) => Response::from_string("hello world"),
            Err(what) => Response::from_string(what.description()),
        };
        request.respond(response);
    }
}
