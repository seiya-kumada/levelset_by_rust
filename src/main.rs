#![allow(unused)]
pub mod core;
pub mod interface;
pub mod test;
use crate::core::grid::Grid3d;
use crate::core::indexer::Indexer3d;
use crate::core::indexer::IndexerMethod;
use crate::core::initial_front::InitialFront3d;
use crate::core::inside_estimator::{InsideEstimator3d, InsideEstimatorMethod};
use crate::core::level_set_method::LevelSetMethod3d;
use crate::core::parameters::Parameters;
use crate::core::point::Point3d;
use crate::core::space_size::SpaceSize3d;
use crate::core::status::Status;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn make_input_gray(size: &SpaceSize3d, front: &InitialFront3d) -> Rc<RefCell<Vec<u8>>> {
    let mut gray = vec![1u8; size.total as usize];
    let left = front.vertices[0].x;
    let top = front.vertices[0].y;
    let front_ = front.vertices[0].z;
    let right = front.vertices[1].x;
    let bottom = front.vertices[1].y;
    let back = front.vertices[1].z;

    let indexer = Indexer3d::new(&size);

    for j in top..bottom {
        for i in left..right {
            let p = Point3d::<i32>::new(i, j, front_);
            let q = Point3d::<i32>::new(i, j, back);
            let p_index = indexer.get(&p) as usize;
            let q_index = indexer.get(&q) as usize;
            gray[p_index] = 0u8;
            gray[q_index] = 0u8;
        }
    }

    for j in top..bottom {
        for k in front_..back {
            let p = Point3d::<i32>::new(left, j, k);
            let q = Point3d::<i32>::new(right, j, k);
            let p_index = indexer.get(&p) as usize;
            let q_index = indexer.get(&q) as usize;
            gray[p_index] = 0u8;
            gray[q_index] = 0u8;
        }
    }

    for i in left..right {
        for k in front_..back {
            let p = Point3d::<i32>::new(i, top, k);
            let q = Point3d::<i32>::new(i, bottom, k);
            let p_index = indexer.get(&p) as usize;
            let q_index = indexer.get(&q) as usize;
            gray[p_index] = 0u8;
            gray[q_index] = 0u8;
        }
    }
    Rc::new(RefCell::new(gray))
}

pub fn main() {
    let mut params = Parameters::new();
    params.wband = 3;
    params.constant_speed = 1.0;
    params.gain = 2.0;
    params.wreset = 1;

    let mut initial_front = InitialFront3d::new();
    let left = 2;
    let top = 3;
    let right = 8;
    let bottom = 7;
    let front = 3;
    let back = 7;

    initial_front.vertices[0] = Point3d::<i32>::new(left, top, front);
    initial_front.vertices[1] = Point3d::<i32>::new(right, bottom, back);

    let size = Rc::new(SpaceSize3d::new(11, 11, 11));
    let gray = make_input_gray(&size, &initial_front);
    let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray));
    lsm.initialize_distance_map();
    lsm.initialize_along_front(&initial_front);
    lsm.initialize_over_all(&initial_front);
    lsm.calculate_speed_factors();

    let resets = true;

    lsm.clear_speed_within_narrow_band(resets);
    lsm.set_speed_on_front();

    lsm.copy_nearest_speed_to_narrow_band(resets);

    let mut status_map: HashMap<usize, Status> = HashMap::new();
    status_map.insert(0, Status::Farway);
    status_map.insert(1, Status::Band);
    status_map.insert(2, Status::ResetBand);
    status_map.insert(3, Status::Front);

    let status_answers: Vec<usize> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1,
        1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
        2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 2, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1,
        3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1,
        1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 0, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 2, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1,
        1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3,
        1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 0, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1,
        1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3,
        3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 0, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1,
        1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3,
        3, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 0, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3,
        1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3,
        3, 3, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 0, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1,
        1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1,
        1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let statuses = lsm.get_statuses();
    for (i, status) in statuses.borrow().iter().enumerate() {
        let a = status_answers[i];
        let b = status_map[&a];
        let c = *status;
        if b != c {
            println!(">> {}", i)
        }
        //assert_eq!(b, c);
    }
}
//不変参照(&,borrow)と可変参照(&mut,borrow_mut)
//借用=borrow
