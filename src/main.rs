#![allow(unused)]
pub mod core;
pub mod interface;
pub mod test;
use crate::core::grid::Grid3d;
use crate::core::initial_front::InitialFront3d;
use crate::core::level_set_method::LevelSetMethod3d;
use crate::core::parameters::Parameters;
use crate::core::point::Point3d;
use crate::core::space_size::SpaceSize3d;
use std::cell::RefCell;
use std::rc::Rc;
pub fn main() {
    let mut params = Parameters::new();
    params.wband = 3;
    let mut initial_front = InitialFront3d::new();
    initial_front.vertices[0] = Point3d::<i32>::new(10, 15, 32);
    initial_front.vertices[1] = Point3d::<i32>::new(82, 74, 61);

    let size = Rc::new(SpaceSize3d::new(101, 143, 131));
    let gray = Rc::new(RefCell::new(vec![0u8]));
    let grid = Grid3d::new();
    let mut lsm = LevelSetMethod3d::new(params.clone(), Rc::clone(&size), Rc::clone(&gray), grid);
    lsm.initialize_along_front(&initial_front);
    lsm.initailze_over_all(&initial_front);
}
//不変参照(&,borrow)と可変参照(&mut,borrow_mut)
//借用=borrow
//mod core;
//use crate::core::distance_map_generator::{
//    DistanceMapGenerator2d, DistanceMapGenerator3d, DistanceMapGeneratorMethod,
//};
//use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
//use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
//use crate::core::status::Status;
//use std::cell::RefCell;
//use std::rc::Rc;
//
//pub fn main() {
//    let size = SpaceSize3d::new(3, 3, 3);
//    let statuses = RefCell::new(Vec::<Status>::new());
//    let wband = 1;
//    let indexer = Rc::new(Indexer3d::new(&size));
//    let mut generator =
//        DistanceMapGenerator3d::new(wband, Rc::clone(&indexer), RefCell::clone(&statuses));
//    generator.create_distance_map();
//    let map = generator.get_distance_map();
//    let mut c = 0;
//    for k in map.keys() {
//        let v = map.get_vec(k).unwrap();
//        c += v.len();
//    }
//    println!("_/_/_/{}", c);
//}
//
