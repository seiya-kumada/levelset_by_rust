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
    let mut params = Parameters::new();
    params.wband = 3;
    params.constant_speed = 1.0;
    params.gain = 2.0;
    params.wreset = 1;
    params.time_step = 1.0;

    let size = Rc::new(SpaceSize3d::new(3, 3, 3));
    let gray = Rc::new(RefCell::new(vec![0u8]));
    let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray));

    let phi = lsm.get_phi();
    let speed = lsm.get_speed();
    let narrow_band = lsm.get_narrow_bands();

    let p = Point3d::<i32>::new(1, 1, 1);
    narrow_band.push(p);
    speed.borrow_mut()[13] = 3.0; // positive

    let sphi = vec![
        0.0, 0.0, 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 4.0, 2.0, 6.0, 0.0, 5.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0,
    ];

    for i in 0..sphi.len() {
        phi.borrow_mut()[i] = sphi[i];
    }

    assert!(phi.borrow()[13] == 2.0);
    speed.borrow_mut()[13] = -3.0;
    lsm.propagate_front();
    println!("{}", phi.borrow()[13]);
    println!("{}", 2.0 + 3.0 * 91.0_f64.sqrt());
}
//不変参照(&,borrow)と可変参照(&mut,borrow_mut)
//借用=borrow
