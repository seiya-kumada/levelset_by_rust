#![allow(unused)]
pub mod core;
pub mod interface;
pub mod test;
use crate::core::grid::Grid3d;
use crate::core::indexer::IndexerMethod;
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::initial_front::{InitialFront2d, InitialFront3d};
use crate::core::inside_estimator::{InsideEstimator3d, InsideEstimatorMethod};
use crate::core::level_set_method::{LevelSetMethod2d, LevelSetMethod3d};
use crate::core::parameters::Parameters;
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::speed_factor::{SpeedFactor3d, SpeedFactorMethod};
use crate::core::status::Status;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn make_input_gray_2d(size: &SpaceSize2d, front: &InitialFront2d) -> Rc<RefCell<Vec<u8>>> {
    let mut gray = vec![1u8; size.total as usize];
    let left = front.vertices[0].x;
    let top = front.vertices[0].y;
    let right = front.vertices[1].x;
    let bottom = front.vertices[1].y;

    let indexer = Indexer2d::new(&size);

    for i in left..(right + 1) {
        let p = Point2d::<i32>::new(i, top);
        let q = Point2d::<i32>::new(i, bottom);
        let p_index = indexer.get(&p) as usize;
        let q_index = indexer.get(&q) as usize;
        gray[p_index] = 0u8;
        gray[q_index] = 0u8;
    }

    for j in top..(bottom + 1) {
        let p = Point2d::<i32>::new(left, j);
        let q = Point2d::<i32>::new(right, j);
        let p_index = indexer.get(&p) as usize;
        let q_index = indexer.get(&q) as usize;
        gray[p_index] = 0u8;
        gray[q_index] = 0u8;
    }

    Rc::new(RefCell::new(gray))
}

pub fn main() {
    let size = Rc::new(SpaceSize3d::new(3, 3, 3));
    let indexer = Rc::new(Indexer3d::new(&size));
    let gray: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(vec![
        0, 100, 0, 100, 0, 100, 0, 100, 0, 0, 100, 0, 100, 0, 100, 0, 100, 0, 0, 100, 0, 100, 100,
        100, 0, 100, 0,
    ]));
    let mut factor = SpeedFactor3d::new(Rc::clone(&indexer), Rc::clone(&gray));
    factor.calculate_all(&size);
    let answer = 1.0 / (1.0 + 12.5);
    let p = Point3d::<i32>::new(1, 1, 1);
    let r = factor.get_value(&p) as f32;
    assert_eq!(answer, r);
}

//不変参照(&,borrow)と可変参照(&mut,borrow_mut)
//借用=borrow
