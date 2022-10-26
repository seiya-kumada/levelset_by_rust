use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::types::{Indexer, IntPoint, SpaceSize, ThreeDim, TwoDim, Type};
use crate::core::upwind_scheme::UpwindScheme;
pub struct Position2d {
    left: i32,
    right: i32,
    me: i32,
    top: i32,
    bottom: i32,
}

trait PositionTrait<D: Type> {
    fn new(p: &IntPoint<D>, indexer: &Indexer<D>);
}

impl<D: Type> PositionTrait<D> for Position2d {
    fn new(p: &IntPoint<D>, indexer: &Indexer<D>) {}
}
impl Position2d {
    pub fn new(p: &IntPoint<TwoDim>, indexer: &Indexer<TwoDim>) {
        let q = indexer.get(&p);
    }
}
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
    pub fn new(p: &IntPoint<ThreeDim>, indexer: &Indexer<ThreeDim>) {
        let q = indexer.get(&p);
    }
}
