use crate::core::distance_map_generator::{
    DistanceMapGenerator2d, DistanceMapGenerator3d, DistanceMapGeneratorMethod,
};
use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::status::Status;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::*;

    struct CheckerSelector2d;
    impl CheckerSelector2d {
        const COUNT: usize = 37;
    }

    struct CheckerSelector3d;
    impl CheckerSelector3d {
        const COUNT: usize = 19;
    }

    #[test]
    fn initialize_distance_map_2d() {
        let size = SpaceSize2d::new(3, 3);
        let statuses = RefCell::new(Vec::<Status>::new());
        let wband = 3;
        let indexer = Rc::new(Indexer2d::new(&size));
        let mut generator =
            DistanceMapGenerator2d::new(wband, Rc::clone(&indexer), RefCell::clone(&statuses));
        generator.create_distance_map();
        let map = generator.get_distance_map();
        let mut c = 0;
        for key in map.keys() {
            c += map.get_vec(key).unwrap().len();
        }
        assert_eq!(CheckerSelector2d::COUNT, c);
    }
}
