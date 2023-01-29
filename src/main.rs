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
use btreemultimap::BTreeMultiMap;
use multimap::MultiMap;
use ordered_multimap;
use std::cell::RefCell;
use std::collections::BTreeMap;
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
    params.wband = 3;
    params.constant_speed = 1.0;
    params.gain = 2.0;
    params.wreset = 1;

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
    lsm.initialize_distance_map();
    lsm.initialize_along_front(&initial_front);
    lsm.initialize_over_all(&initial_front);
    lsm.calculate_speed_factors();

    let resets = true;

    lsm.clear_speed_within_narrow_band(resets);
    lsm.set_speed_on_front();

    lsm.copy_nearest_speed_to_narrow_band(resets);

    let squared_phi_answers: Vec<f64> = vec![
        9.0, 10.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 10.0, 9.0, 8.0, 5.0, 4.0, 4.0, 4.0, 4.0, 4.0,
        4.0, 4.0, 5.0, 8.0, 5.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 5.0, 4.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 4.0, 4.0, 1.0, 0.0, -1.0, -1.0, -1.0, -1.0, -1.0, 0.0,
        1.0, 4.0, 4.0, 1.0, 0.0, -1.0, -4.0, -4.0, -4.0, -1.0, 0.0, 1.0, 4.0, 4.0, 1.0, 0.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, 0.0, 1.0, 4.0, 4.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        4.0, 5.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 5.0, 8.0, 5.0, 4.0, 4.0, 4.0, 4.0,
        4.0, 4.0, 4.0, 5.0, 8.0, 9.0, 10.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 10.0, 9.0,
    ];

    let phi = lsm.get_phi();
    for (phi, ans) in phi.borrow().iter().zip(&squared_phi_answers) {
        let mut a = ans.abs().sqrt();
        if *ans < 0.0 {
            a = -a;
        }
        //assert!((phi - a).abs() < 1.0e-03);
        //if (phi - a).abs() < 1.0e-03 {
        //} else {
        //    println!("phi:{}", phi);
        //    println!("a:{}", a);
        //}
    }

    // btreemultimapを使うようにする。
    // ロジックが足りていない。advanceのあたり。
    let mut ts = BTreeMultiMap::<usize, String>::new();
    //let mut ts = BTreeMap::<usize, String>::new();
    ts.insert(1, String::from("foo-1"));
    ts.insert(1, String::from("foo-2"));
    ts.insert(3, String::from("bar"));
    ts.insert(2, String::from("bar-1"));
    ts.insert(2, String::from("bar-2"));
    //for (k, v) in ts.iter().rev() {
    //    println!("{k}: {v}");
    //}

    let mut iter = ts.iter().rev();
    //let end = iter.last();
    //while iter != end {

    //}
    let (k, v) = iter.next().unwrap();

    println!("{},{}", k, v);
    let (k, v) = iter.next().unwrap();
    println!("{},{}", k, v);
}

//不変参照(&,borrow)と可変参照(&mut,borrow_mut)
//借用=borrow
