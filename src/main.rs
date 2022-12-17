#![allow(unused)]
use clap::Parser;
use interface::commandline_interface as cli;
pub mod core;
pub mod interface;
pub mod test;
pub fn main() {
    let args = cli::CommandlineArguments::parse();
    cli::execute_level_set_method(&args);
}

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
