#![allow(unused)]
pub mod core;
pub mod interface;
pub mod test;
use crate::core::grid::Grid3d;
use crate::core::indexer::IndexerMethod;
use crate::core::initial_front::InitialFront3d;
use crate::core::inside_estimator::{InsideEstimator3d, InsideEstimatorMethod};
use crate::core::level_set_method::LevelSetMethod3d;
use crate::core::parameters::Parameters;
use crate::core::point::Point3d;
use crate::core::space_size::SpaceSize3d;
use crate::core::status::Status;
use std::cell::RefCell;
use std::rc::Rc;
pub fn main() {
    let mut vs = Rc::new(RefCell::new(vec![1, 2, 3]));
    //let a = vs.borrow();
    let b = vs.borrow_mut();
}
//不変参照(&,borrow)と可変参照(&mut,borrow_mut)
//借用=borrow
