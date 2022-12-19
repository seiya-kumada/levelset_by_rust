use crate::core::grid::Grid3d;
use crate::core::indexer::{Indexer3d, IndexerMethod};
use crate::core::initial_front::InitialFront3d;
use crate::core::level_set_method::LevelSetMethod3d;
use crate::core::parameters::Parameters;
use crate::core::point::Point3d;
use crate::core::space_size::SpaceSize3d;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use crate::core::{initial_front, parameters};

    use super::*;
    #[test]
    fn front_3d() {
        let mut params = Parameters::new();
        params.wband = 3;
        let mut initial_front = InitialFront3d::new();

        initial_front.vertices[0] = Point3d::<i32>::new(0, 0, 0);
        initial_front.vertices[1] = Point3d::<i32>::new(2, 2, 2);

        let size = Rc::new(SpaceSize3d::new(3, 3, 3));
        let gray = RefCell::new(vec![0u8]);
        let grid = Grid3d::new();
        let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), RefCell::clone(&gray), grid);
        lsm.initialize_along_front(&initial_front);

        let front = lsm.get_front();
        //assert_eq!(front.len(), 26);
    }

    //#[test]
    //fn initialize_along_front_3d() {
    //    let mut params = Parameters::new();
    //    params.wband = 3;
    //    let mut initial_front = InitialFront3d::new();

    //    initial_front.vertices[0] = Point3d::<i32>::new(10, 15, 32);
    //    initial_front.vertices[1] = Point3d::<i32>::new(82, 74, 61);

    //    let size = Rc::new(SpaceSize3d::new(101, 143, 131));
    //    let gray = RefCell::new(vec![0u8]);
    //    let grid = Grid3d::new();
    //    let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), RefCell::clone(&gray), grid);
    //    lsm.initialize_along_front(&initial_front);

    //    let phi = lsm.get_phi();
    //    let statuses = lsm.get_statuses();

    //    let left = initial_front.vertices[0].x;
    //    let top = initial_front.vertices[0].y;
    //    let front_ = initial_front.vertices[0].z;
    //    let right = initial_front.vertices[1].x;
    //    let bottom = initial_front.vertices[1].y;
    //    let back = initial_front.vertices[1].z;

    //    let indexer = Indexer3d::new(&size);
    //    let mut index = 0usize;
    //    let mut s = 0usize;
    //    for j in top..bottom {
    //        for i in left..right {
    //            let p = Point3d::new(i, j, front_);
    //            let index = indexer.get(&p) as usize;
    //            //assert_eq!(phi.borrow()[index], 0.0);
    //            //BOOST_CHECK(phi[index] == 0 && statuses[index] == Status::Front);
    //            //index = indexer({{i, j, back}});
    //            //BOOST_CHECK(phi[index] == 0 && statuses[index] == Status::Front);
    //            //s += 2;
    //        }
    //    }
    //}
}
