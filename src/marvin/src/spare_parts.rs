use robot_traits::{Led, Robot};
use std::rc::Rc;

pub struct SpareParts<T: Robot, U: Led> {
    pub robot: Option<Rc<T>>,
    pub led: Option<Rc<U>>,
}
