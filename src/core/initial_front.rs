use crate::core::point::{Point2d, Point3d};
pub struct InitialFront2d {
    pub vertices: [Point2d<i32>; 2],
}

impl InitialFront2d {
    pub fn new() -> Self {
        Self {
            vertices: [Point2d::<i32>::new(0, 0), Point2d::<i32>::new(0, 0)],
        }
    }
}
pub struct InitialFront3d {
    pub vertices: [Point3d<i32>; 2],
}

impl InitialFront3d {
    pub fn new() -> Self {
        Self {
            vertices: [Point3d::<i32>::new(0, 0, 0), Point3d::<i32>::new(0, 0, 0)],
        }
    }
}
