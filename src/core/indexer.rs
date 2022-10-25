use crate::core::point::IntPoint;
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
//
pub struct Indexer2d {
    width: i32,
}
//
impl Indexer2d {
    pub fn new(size: &SpaceSize2d) -> Self {
        Self { width: size.width }
    }
    pub fn get(&self, p: &IntPoint<2>) -> i32 {
        p[0] + self.width * p[1]
    }
}
//
pub struct Indexer3d {
    width: i32,
    area: i32,
}

impl Indexer3d {
    pub fn new(size: &SpaceSize3d) -> Self {
        Self {
            width: size.width,
            area: size.width * size.height,
        }
    }

    pub fn get(&self, p: &IntPoint<3>) -> i32 {
        p[0] + self.width * p[1] + self.area * p[2]
    }
}
