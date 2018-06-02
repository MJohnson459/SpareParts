use std::rc::Rc;
use robot_traits::{Led, Robot};

pub struct SpareParts<T: Robot, U: Led> {
    pub robot: Option<Rc<T>>,
    pub led: Option<Rc<U>>,
}
