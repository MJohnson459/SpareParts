use std::io::Cursor;
use std::rc::Rc;
use tiny_http::{Response, Server};

use robot_traits::{Led, Robot};
use SpareParts;

pub struct RobotServer {}

impl RobotServer {
    pub fn run<T: Robot, U: Led>(spare_parts: &mut SpareParts<T, U>) {
        let server = Server::http("0.0.0.0:8000").unwrap();

        for request in server.incoming_requests() {
            println!(
                "received request! method: {:?}, url: {:?}",
                request.method(),
                request.url(),
            );

            let response = {
                let path: Vec<&str> = request.url().split('/').collect();

                // path[0] should always be "" as min path is "/"
                assert_eq!(path[0], "");
                match path[1] {
                    "led" => handle_led(&mut spare_parts.led, &path.as_slice()[1..]),
                    "robot" => handle_robot(&mut spare_parts.robot, &path.as_slice()[1..]),
                    _ => Response::from_string(format!("Request not recognised: {:?}", path)),
                }
            };

            request.respond(response).unwrap();
        }
    }
}

fn handle_led<T: Led>(led: &mut Option<Rc<T>>, request: &[&str]) -> Response<Cursor<Vec<u8>>> {
    let not_found = Response::from_string(format!("[led] Request not recognised: {:?}", request));
    match led {
        Some(ref mut led) => if request.len() > 1 {
            let mled = Rc::get_mut(led).unwrap();
            match request[1] {
                "led_on" => {
                    mled.led_on();
                    Response::from_string(format!("[led] turning LED on"))
                }
                "led_off" => {
                    mled.led_off();
                    Response::from_string(format!("[led] turning LED off"))
                }
                _ => not_found,
            }
        } else {
            not_found
        },
        None => Response::from_string("[led] LED not available"),
    }
}

fn handle_robot<T: Robot>(
    robot: &mut Option<Rc<T>>,
    request: &[&str],
) -> Response<Cursor<Vec<u8>>> {
    // Check robot is available
    let not_found = Response::from_string(format!("[robot] Request not recognised: {:?}", request));
    match robot {
        Some(ref mut robot) => if request.len() > 1 {
            let mrobot = Rc::get_mut(robot).unwrap();
            match request[1] {
                "forward" => {
                    mrobot.forward(0.5);
                    Response::from_string("[robot] moving forward at 0.5")
                }
                "stop" => {
                    mrobot.stop();
                    Response::from_string("[robot] stopping robot")
                }
                _ => not_found,
            }
        } else {
            not_found
        },
        None => Response::from_string("[robot] Robot not available"),
    }
}
