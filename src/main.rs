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
    let mut params = Parameters::new();
    params.wband = 1;
    params.constant_speed = 1.0;
    params.gain = 2.0;

    let mut initial_front = InitialFront2d::new();
    let left = 2;
    let top = 3;
    let right = 8;
    let bottom = 7;

    initial_front.vertices[0] = Point2d::<i32>::new(left, top);
    initial_front.vertices[1] = Point2d::<i32>::new(right, bottom);

    let size = Rc::new(SpaceSize2d::new(11, 11));
    let gray = make_input_gray_2d(&size, &initial_front);

    let mut lsm = LevelSetMethod2d::new(params, Rc::clone(&size), Rc::clone(&gray));
    lsm.initialize_along_front(&initial_front);
    lsm.initialize_over_all(&initial_front);
    lsm.calculate_speed_factors();

    let fs = lsm.set_speed_on_front();
    assert!(fs != 0.0);

    let width = size.width;
    let height = size.height;

    let speed = lsm.get_speed();

    for j in 0..height {
        let wj = width * j;
        for i in 0..width {
            let index = (wj + i) as usize;
            if (i == left && j == top) {
                assert!(0.0 != speed.borrow()[index]);
            } else if (left < i && i <= right && j == top) {
                assert!(0.0 != speed.borrow()[index]);
            } else if (i == right && top <= j && j < bottom) {
                assert!(0.0 != speed.borrow()[index]);
            } else if (i == right && j == bottom) {
                assert!(0.0 != speed.borrow()[index]);
            } else if (left <= i && i < right && j == top) {
                assert!(0.0 != speed.borrow()[index]);
            } else if (i == left && top < j && top <= bottom) {
                //assert!(0.0 != speed.borrow()[index]);
                if 0.0 == speed.borrow()[index] {
                    println!("E: {},{}", index, speed.borrow()[index]);
                }
            } else {
                //assert!(0.0 == speed.borrow()[index]);
                if 0.0 != speed.borrow()[index] {
                    println!("F: {},{}", index, speed.borrow()[index]);
                }
            }
        }
    }

    //let s = 1;
    //let e = 3;
    //for i in s..e {
    //    println!("{}", i);
    //}
    //for i in ((s + 1)..(e + 1)).rev() {
    //    println!("{}", i);
    //}
}
//不変参照(&,borrow)と可変参照(&mut,borrow_mut)
//借用=borrow
