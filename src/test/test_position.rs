use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::point::{Point2d, Point3d};
use crate::core::position::{Position2d, Position3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use std::rc::Rc;

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn position_2d() {
        let mut a = Position2d::new();
        let p = Point2d::<i32>::new(1, 2);
        let space_size = SpaceSize2d::new(1, 2);
        let indexer = Rc::new(Indexer2d::new(&space_size));
        a.set_position(&p, Rc::clone(&indexer));
        assert_eq!(a.left, 2);
        assert_eq!(a.right, 4);
        assert_eq!(a.me, 3);
        assert_eq!(a.top, 2);
        assert_eq!(a.bottom, 4);
    }

    #[test]
    fn position_3d() {
        let mut a = Position3d::new();
        let p = Point3d::<i32>::new(1, 1, 1);
        let space_size = SpaceSize3d::new(1, 1, 1);
        let indexer = Rc::new(Indexer3d::new(&space_size));
        let phi = Rc::new(vec![0.0, 1.0, 2.0, 3.0, 4.0]);

        a.set_position(&p, Rc::clone(&indexer));

        assert_eq!(a.left, 2);
        assert_eq!(a.right, 4);
        assert_eq!(a.me, 3);
        assert_eq!(a.top, 2);
        assert_eq!(a.bottom, 4);
        assert_eq!(a.front, 2);
        assert_eq!(a.back, 4);
    }
}
