use crate::core::point::Point2d;

pub struct InitialFront2d<T> {
    pub vertices: [Point2d<T>; 2],
}

impl<T> InitialFront2d<T> {
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
        let a = Point2d { x: left, y: top };
        let b = Point2d {
            x: right,
            y: bottom,
        };
        Self { vertices: [a, b] }
    }
}
