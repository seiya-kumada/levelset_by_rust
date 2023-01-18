use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::point::{Point2d, Point3d, PointMethod};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::speed_factor::{SpeedFactor2d, SpeedFactor3d, SpeedFactorMethod};
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speed_factor_2d() {
        let size = Rc::new(SpaceSize2d::new(3, 3));
        let indexer = Rc::new(Indexer2d::new(&size));
        let gray = Rc::new(RefCell::new(vec![50, 100, 20, 100, 0, 200, 70, 100, 30]));
        let mut factor = SpeedFactor2d::new(Rc::clone(&indexer), Rc::clone(&gray));
        factor.calculate_all(&size);
        let dx: f32 = 65.0 / 4.0;
        let dy: f32 = 15.0 / 4.0;
        let answer = 1.0 / (1.0 + (dx.powf(2.0) + dy.powf(2.0)).sqrt());
        let p = Point2d::<i32>::new(1, 1);
        let r = factor.get_value(&p) as f32;
        assert!(r == answer);
    }

    #[test]
    fn speed_factor_3d() {
        let size = Rc::new(SpaceSize3d::new(3, 3, 3));
        let indexer = Rc::new(Indexer3d::new(&size));
        let gray: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(vec![
            0, 100, 0, 100, 0, 100, 0, 100, 0, 0, 100, 0, 100, 0, 100, 0, 100, 0, 0, 100, 0, 100,
            100, 100, 0, 100, 0,
        ]));
        let mut factor = SpeedFactor3d::new(Rc::clone(&indexer), Rc::clone(&gray));
        factor.calculate_all(&size);
        let a = (3.0f32 * 10000.0).sqrt();
        let answer = 1.0 / (1.0 + a);
        let p = Point3d::<i32>::new(1, 1, 1);
        let r = factor.get_value(&p) as f32;
        //assert_eq!(answer, r);
    }
}
