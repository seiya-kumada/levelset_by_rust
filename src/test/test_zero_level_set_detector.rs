use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d, SpaceSizeMethod};
use crate::core::zero_level_set_detector::{
    ZeroLevelSetDetector2d, ZeroLevelSetDetector3d, ZeroLevelSetDetectorMethod,
};
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2d() {
        let size = Rc::new(SpaceSize2d::new(3, 3));
        let indexer = Rc::new(Indexer2d::new(&size));

        let phi = Rc::new(RefCell::new(vec![
            0.0, 3.0, 0.0, 4.0, 2.0, 6.0, 0.0, 5.0, 0.0,
        ]));

        let includes_zero_level_set =
            ZeroLevelSetDetector2d::new(Rc::clone(&phi), Rc::clone(&indexer));

        let p = Point2d::<i32>::new(1, 1);
        let flag = includes_zero_level_set.detect(&p);
        assert!(flag == false);

        phi.borrow_mut()[1] = -10.0;
        let p = Point2d::<i32>::new(1, 1);
        let flag = includes_zero_level_set.detect(&p);
        assert!(flag == true);
    }

    #[test]
    fn test_3d() {
        let size = Rc::new(SpaceSize3d::new(3, 3, 3));
        let indexer = Rc::new(Indexer3d::new(&size));

        let phi = Rc::new(RefCell::new(vec![
            0.0, 0.0, 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 4.0, 2.0, 6.0, 0.0, 5.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0,
        ]));

        let includes_zero_level_set =
            ZeroLevelSetDetector3d::new(Rc::clone(&phi), Rc::clone(&indexer));

        let p = Point3d::<i32>::new(1, 1, 1);
        let flag = includes_zero_level_set.detect(&p);
        assert!(flag == false);

        phi.borrow_mut()[4] = -10.0;
        let flag = includes_zero_level_set.detect(&p);
        assert!(flag == true);
    }
}
