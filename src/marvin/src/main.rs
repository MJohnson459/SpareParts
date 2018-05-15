extern crate picoborg_rev;
extern crate tiny_http;

use tiny_http::{Server, Response};
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

        toggle_led();

        let response = Response::from_string("hello world");
        request.respond(response);
    }
}
