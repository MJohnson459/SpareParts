extern crate tiny_http;
extern crate i2cdev;

mod picoborg_rev;

use tiny_http::{Server, Response};
#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::LinuxI2CError;
use std::error::Error;


use picoborg_rev::toggle_led;

fn main() {
    println!("Hello, world!");

    let server = Server::http("0.0.0.0:8000").unwrap();

    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );


        let response = match toggle_led() {
            Ok(_) => Response::from_string("hello world"),
            Err(what) => Response::from_string(what.description()),
        };
        request.respond(response);
    }
}
