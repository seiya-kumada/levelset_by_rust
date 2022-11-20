//use crate::core::speed as sp;
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::upwind_scheme::UpwindScheme2d;
use std::rc::Rc;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn position_2d() {
        let p = Point2d::<i32>::new(1, 2);
        let space_size = SpaceSize2d::new(1, 2);
        let indexer = Rc::new(Indexer2d::new(&space_size));
        let phi = Rc::new(vec![0.0, 1.0, 2.0, 3.0, 4.0]);
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

    //#[test]
    //fn position_3d() {
    //    let p = types::IntPoint::<types::ThreeDim>::new(1, 1, 1);
    //    let space_size = types::SpaceSize::<types::ThreeDim>::new(1, 1, 1);
    //    let indexer = types::Indexer::<types::ThreeDim>::new(&space_size);
    //    let phi = vec![0.0, 1.0, 2.0, 3.0, 4.0];
    //    let scheme =
    //        us::UpwindScheme::<types::ThreeDim>::new(&p, &indexer, &phi, &sp::Speed::Positive);

    //    let r = scheme.position;
    //    assert_eq!(r.left, 2);
    //    assert_eq!(r.right, 4);
    //    assert_eq!(r.me, 3);
    //    assert_eq!(r.top, 2);
    //    assert_eq!(r.bottom, 4);
    //    assert_eq!(r.front, 2);
    //    assert_eq!(r.back, 4);

    //    let u = scheme.upwind;
    //    assert_eq!(u.fdxm, 1.0);
    //    assert_eq!(u.fdxp, 0.0);
    //    assert_eq!(u.fdym, 1.0);
    //    assert_eq!(u.fdyp, 0.0);
    //    assert_eq!(u.fdzm, 1.0);
    //    assert_eq!(u.fdzp, 0.0);
    //}
}
