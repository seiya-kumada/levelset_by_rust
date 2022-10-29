use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::types;
use crate::core::types::{Indexer, IntPoint, SpaceSize, Type};
use crate::core::upwind_scheme::UpwindScheme;

pub struct Position2d {
    left: i32,
    right: i32,
    me: i32,
    top: i32,
    bottom: i32,
}

impl Position2d {
    pub fn new(left: i32, right: i32, me: i32, top: i32, bottom: i32) -> Self {
        Self {
            left,
            right,
            me,
            top,
            bottom,
        }
    }

    pub fn make(&self) {}
}

//pub trait PositionTrait {
//    fn make<D: Type>(&self, p: &types::IntPoint<D>);
//}
//
//impl PositionTrait for Position2d {
//    fn make<D: Type>(&self, p: &types::IntPoint<D>) {}
//}
//
//impl PositionTrait for Position3d {
//    fn make<D: Type>(&self, p: &types::IntPoint<D>) {}
//}
//impl PositionTrait for TwoDim {
//    fn create<types::TwoDim>(p: &IntPoint<types::TwoDim>) {}
//}

//impl PositionTrait for ThreeDim {
//    fn create<D: Type>(p: &IntPoint<D>) {}
//}

pub struct Position3d {
    left: i32,
    right: i32,
    me: i32,
    top: i32,
    bottom: i32,
    front: i32,
    back: i32,
}

impl Position3d {
    pub fn new(
        left: i32,
        right: i32,
        me: i32,
        top: i32,
        bottom: i32,
        front: i32,
        back: i32,
    ) -> Self {
        Self {
            left,
            right,
            me,
            top,
            bottom,
            front,
            back,
        }
    }

    pub fn make(&self) {}
}
