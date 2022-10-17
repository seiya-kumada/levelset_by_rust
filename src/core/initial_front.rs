use crate::core::point::Point2D;

pub struct InitialFront2D<T> {
    pub vertices: [Point2D<T>; 2],
}

impl<T> InitialFront2D<T> {
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
        let a = Point2D::<T> { x: left, y: top };
        let b = Point2D::<T> {
            x: right,
            y: bottom,
        };
        Self { vertices: [a, b] }
    }
}

pub type IntInitialFront2D = InitialFront2D<i32>;
