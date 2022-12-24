use crate::core::grid::Grid3d;
use crate::core::indexer::{Indexer3d, IndexerMethod};
use crate::core::initial_front::InitialFront3d;
use crate::core::level_set_method::LevelSetMethod3d;
use crate::core::parameters::Parameters;
use crate::core::point::Point3d;
use crate::core::space_size::SpaceSize3d;
use crate::core::status::Status;
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
        let gray = Rc::new(RefCell::new(vec![0u8]));
        let grid = Grid3d::new();
        let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray), grid);
        lsm.initialize_along_front(&initial_front);

        let front = lsm.get_front();
        assert_eq!(front.borrow().len(), 26);
    }

    #[test]
    fn initialize_along_front_3d() {
        let mut params = Parameters::new();
        params.wband = 3;
        let mut initial_front = InitialFront3d::new();

        initial_front.vertices[0] = Point3d::<i32>::new(10, 15, 32);
        initial_front.vertices[1] = Point3d::<i32>::new(82, 74, 61);

        let size = Rc::new(SpaceSize3d::new(101, 143, 131));
        let gray = Rc::new(RefCell::new(vec![0u8]));
        let grid = Grid3d::new();
        let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray), grid);
        lsm.initialize_along_front(&initial_front);

        let phi = lsm.get_phi();
        let statuses = lsm.get_statuses();

        let left = initial_front.vertices[0].x;
        let top = initial_front.vertices[0].y;
        let front_ = initial_front.vertices[0].z;
        let right = initial_front.vertices[1].x;
        let bottom = initial_front.vertices[1].y;
        let back = initial_front.vertices[1].z;

        let indexer = Indexer3d::new(&size);

        for j in top..bottom {
            for i in left..right {
                let p = Point3d::new(i, j, front_);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
                let p = Point3d::<i32>::new(i, j, back);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
            }
        }

        for k in front_..back {
            for i in left..right {
                let p = Point3d::new(i, top, k);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
                let p = Point3d::<i32>::new(i, bottom, k);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
            }
        }

        for j in top..bottom {
            for k in front_..back {
                let p = Point3d::new(left, j, k);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
                let p = Point3d::<i32>::new(right, j, k);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
            }
        }
    }

    #[test]
    fn initialize_over_all_3d() {
        let mut params = Parameters::new();
        params.wband = 3;
        let mut initial_front = InitialFront3d::new();
        initial_front.vertices[0] = Point3d::<i32>::new(10, 15, 32);
        initial_front.vertices[1] = Point3d::<i32>::new(82, 74, 61);

        let size = Rc::new(SpaceSize3d::new(101, 143, 131));
        let gray = Rc::new(RefCell::new(vec![0u8]));
        let grid = Grid3d::new();
        let mut lsm =
            LevelSetMethod3d::new(params.clone(), Rc::clone(&size), Rc::clone(&gray), grid);
        lsm.initialize_along_front(&initial_front);
        //これが終わらない。
        lsm.initailze_over_all(&initial_front);

        //let phi = lsm.get_phi();
        //let width = size.width;
        //let height = size.height;
        //let depth = size.depth;
        //let indexer = lsm.get_indexer();
        //let statuses = lsm.get_statuses();
        //for k in 0..depth {
        //    for j in 0..height {
        //        for i in 0..width {
        //            let p = Point3d::<i32>::new(i, j, k);
        //            let index = indexer.get(&p) as usize;
        //            if statuses.borrow()[index] != Status::Front {
        //                assert_eq!(phi.borrow()[index], -params.wband as f64);
        //            } else {
        //                assert_eq!(phi.borrow()[index], params.wband as f64);
        //            }
        //        }
        //    }
        //}
    }
}
