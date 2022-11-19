//use crate::core::dim;
use crate::core::point;
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use num_traits::Num;

//
pub struct Indexer2d {
    width: i32,
}
//
impl Indexer2d {
    pub fn new(size: &SpaceSize2d) -> Self {
        Self { width: size.width }
    }
    pub fn get(&self, p: &Point2d<i32>) -> i32 {
        p.x + self.width * p.y
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

    pub fn get(&self, p: &Point3d<i32>) -> i32 {
        p.x + self.width * p.y + self.area * p.z
    }
}

//pub trait IndexerT {
//    type Type;
//    type Point;
//
//    fn get(&self, p: &Self::Point) -> i32;
//}

//impl IndexerT for dim::Two {
//    type Type = Indexer2d;
//    type Point = point::Point2d<u8>;
//    fn get(&self, p: &Self::Point) -> i32 {
//        1
//    }
//}
//
//impl IndexerT for dim::Three {
//    type Type = Indexer3d;
//    fn get<T: Num, D: point::PointT<T>>(&self, p: &point::Point<D, T>) -> i32 {
//        1
//    }
//}
//
//pub type Indexer<D> = <D as IndexerT>::Type;
