//use crate::core::dim;
use crate::core::point;
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use num_traits::Num;
pub trait New<T> {
    fn new(t: &T) -> Self;
}
pub struct Indexer2d {
    width: i32,
}

impl Indexer2d {
    //pub fn new(size: &SpaceSize2d) -> Self {
    //    Self { width: size.width }
    //}
    pub fn get(&self, p: &Point2d<i32>) -> i32 {
        p.x + self.width * p.y
    }
}

impl New<SpaceSize2d> for Indexer2d {
    fn new(size: &SpaceSize2d) -> Self {
        Self { width: size.width }
    }
}
pub struct Indexer3d {
    width: i32,
    area: i32,
}

impl Indexer3d {
    //pub fn new(size: &SpaceSize3d) -> Self {
    //    Self {
    //        width: size.width,
    //        area: size.width * size.height,
    //    }
    //}

    pub fn get(&self, p: &Point3d<i32>) -> i32 {
        p.x + self.width * p.y + self.area * p.z
    }
}

impl New<SpaceSize3d> for Indexer3d {
    fn new(size: &SpaceSize3d) -> Self {
        Self {
            width: size.width,
            area: size.width * size.height,
        }
    }
}
