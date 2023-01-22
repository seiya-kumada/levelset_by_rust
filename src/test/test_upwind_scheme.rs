use crate::core::indexer;
use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::speed::Speed;
use crate::core::upwind_scheme::{UpwindScheme2d, UpwindScheme3d, UpwindSchemeMethod};
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use crate::core::upwind_scheme;

    use super::*;
    #[test]
    fn position_2d() {
        let p = Point2d::<i32>::new(1, 2);
        let space_size = SpaceSize2d::new(1, 2);
        let indexer = Rc::new(Indexer2d::new(&space_size));
        let phi = Rc::new(RefCell::new(vec![0.0, 1.0, 2.0, 3.0, 4.0]));
        let mut scheme = UpwindScheme2d::new(Rc::clone(&indexer), Rc::clone(&phi));
        scheme.position.set_position(&p, Rc::clone(&indexer));
        let r = &scheme.position;
        assert_eq!(r.left, 2);
        assert_eq!(r.right, 4);
        assert_eq!(r.me, 3);
        assert_eq!(r.top, 2);
        assert_eq!(r.bottom, 4);

        scheme.calculate_with_positive_speed();
        let u = &scheme.upwind;
        assert_eq!(u.fdxm, 1.0);
        assert_eq!(u.fdxp, 0.0);
        assert_eq!(u.fdym, 1.0);
        assert_eq!(u.fdyp, 0.0);
    }

    #[test]
    fn set_position_with_2d() {
        let size = SpaceSize2d::new(3, 3);
        let phi = Rc::new(RefCell::new(vec![
            0.0, 3.0, 0.0, 4.0, 2.0, 6.0, 0.0, 5.0, 0.0,
        ]));
        let indexer = Rc::new(Indexer2d::new(&size));
        let mut scheme = UpwindScheme2d::new(Rc::clone(&indexer), Rc::clone(&phi));
        let p = Point2d::<i32>::new(1, 1);
        scheme.position.set_position(&p, Rc::clone(&indexer));
        let r = &scheme.position;

        assert!(r.left == 3);
        assert!(r.right == 5);
        assert!(r.top == 1);
        assert!(r.bottom == 7);
    }

    #[test]
    fn calculate_with_2d_positive() {
        let size = SpaceSize2d::new(3, 3);
        let phi = Rc::new(RefCell::new(vec![
            0.0, 3.0, 0.0, 4.0, 2.0, 6.0, 0.0, 5.0, 0.0,
        ]));
        let indexer = Rc::new(Indexer2d::new(&size));
        let mut scheme = UpwindScheme2d::new(Rc::clone(&indexer), Rc::clone(&phi));

        let p = Point2d::<i32>::new(1, 1);

        scheme.position.set_position(&p, Rc::clone(&indexer));
        scheme.calculate_with_positive_speed();

        assert!(scheme.upwind.fdxm == 0.0);
        assert!(scheme.upwind.fdxp == 0.0);
        assert!(scheme.upwind.fdym == 0.0);
        assert!(scheme.upwind.fdyp == 0.0);
    }

    #[test]
    fn calculate_with_2d_negative() {
        let size = SpaceSize2d::new(3, 3);
        let phi = Rc::new(RefCell::new(vec![
            0.0, 3.0, 0.0, 4.0, 2.0, 6.0, 0.0, 5.0, 0.0,
        ]));

        let indexer = Rc::new(Indexer2d::new(&size));
        let mut scheme = UpwindScheme2d::new(Rc::clone(&indexer), Rc::clone(&phi));

        let p = Point2d::<i32>::new(1, 1);

        scheme.position.set_position(&p, Rc::clone(&indexer));

        scheme.calculate_with_negative_speed();

        assert!(scheme.upwind.fdxp == 4.0);
        assert!(scheme.upwind.fdxm == -2.0);
        assert!(scheme.upwind.fdyp == 3.0);
        assert!(scheme.upwind.fdym == -1.0);
    }

    #[test]
    fn calculate_2d() {
        let size = SpaceSize2d::new(3, 3);
        let phi = Rc::new(RefCell::new(vec![
            0.0, 3.0, 0.0, 4.0, 2.0, 6.0, 0.0, 5.0, 0.0,
        ]));
        let indexer = Rc::new(Indexer2d::new(&size));
        let mut scheme = UpwindScheme2d::new(Rc::clone(&indexer), Rc::clone(&phi));

        let p = Point2d::<i32>::new(1, 1);
        let r = scheme.calculate(&p, Speed::Positive);
        assert!(0.0 == r);
        let s = scheme.calculate(&p, Speed::Negative);
        assert!(30.0f64.sqrt() == s);
    }

    #[test]
    fn position_3d() {
        let p = Point3d::<i32>::new(1, 1, 1);
        let space_size = SpaceSize3d::new(1, 1, 1);
        let indexer = Rc::new(<Indexer3d as indexer::IndexerMethod<
            SpaceSize3d,
            Point3d<i32>,
        >>::new(&space_size));
        let phi = Rc::new(RefCell::new(vec![0.0, 1.0, 2.0, 3.0, 4.0]));

        let mut scheme = <UpwindScheme3d as upwind_scheme::UpwindSchemeMethod<
            Indexer3d,
            Point3d<i32>,
        >>::new(Rc::clone(&indexer), Rc::clone(&phi));
        scheme.position.set_position(&p, Rc::clone(&indexer));

        let r = &scheme.position;
        assert_eq!(r.left, 2);
        assert_eq!(r.right, 4);
        assert_eq!(r.me, 3);
        assert_eq!(r.top, 2);
        assert_eq!(r.bottom, 4);
        assert_eq!(r.front, 2);
        assert_eq!(r.back, 4);

        scheme.calculate_with_positive_speed();
        let u = &scheme.upwind;
        assert_eq!(u.fdxm, 1.0);
        assert_eq!(u.fdxp, 0.0);
        assert_eq!(u.fdym, 1.0);
        assert_eq!(u.fdyp, 0.0);
        assert_eq!(u.fdzm, 1.0);
        assert_eq!(u.fdzp, 0.0);
    }

    #[test]
    fn calculate_3d() {
        let size = SpaceSize3d::new(3, 3, 3);
        let phi = Rc::new(RefCell::new(vec![
            0.0, 0.0, 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 4.0, 2.0, 6.0, 0.0, 5.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0,
        ]));

        let indexer = Rc::new(Indexer3d::new(&size));
        let mut scheme = UpwindScheme3d::new(Rc::clone(&indexer), Rc::clone(&phi));

        let p = Point3d::<i32>::new(1, 1, 1);

        let r = scheme.calculate(&p, Speed::Positive);
        assert!(0.0 == r);
        let s = scheme.calculate(&p, Speed::Negative);
        assert!(91.0f64.sqrt() == s);
    }
}
