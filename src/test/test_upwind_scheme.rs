use crate::core::indexer;
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
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
        let indexer = Rc::new(<Indexer2d as indexer::IndexerMethod<
            SpaceSize2d,
            Point2d<i32>,
        >>::new(&space_size));
        let phi = Rc::new(RefCell::new(vec![0.0, 1.0, 2.0, 3.0, 4.0]));
        let mut scheme = <UpwindScheme2d as upwind_scheme::UpwindSchemeMethod<
            Indexer2d,
            Point2d<i32>,
        >>::new(Rc::clone(&indexer), Rc::clone(&phi));
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
}
