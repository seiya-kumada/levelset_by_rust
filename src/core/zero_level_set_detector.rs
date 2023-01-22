use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::neighboring_point::{NEIGHBORING_POINTS2D, NEIGHBORING_POINTS3D};
use crate::core::point::{Point2d, Point3d};
use std::cell::RefCell;
use std::rc::Rc;
pub trait ZeroLevelSetDetectorMethod<Indexer, Point> {
    fn new(phi: Rc<RefCell<Vec<f64>>>, indexer: Rc<Indexer>) -> Self;
    fn detect(&self, p: &Point) -> bool;
}

pub struct ZeroLevelSetDetector2d {
    phi: Rc<RefCell<Vec<f64>>>,
    indexer: Rc<Indexer2d>,
}

impl ZeroLevelSetDetector2d {
    fn is_negative(&self, a: &f64, b: &Point2d<i32>) -> bool {
        let index = self.indexer.get(b) as usize;
        a + self.phi.borrow()[index] <= 0.0
    }

    fn is_positive(&self, a: &f64, b: &Point2d<i32>) -> bool {
        let index = self.indexer.get(b) as usize;
        a + self.phi.borrow()[index] > 0.0
    }
}

impl ZeroLevelSetDetectorMethod<Indexer2d, Point2d<i32>> for ZeroLevelSetDetector2d {
    fn new(phi: Rc<RefCell<Vec<f64>>>, indexer: Rc<Indexer2d>) -> Self {
        Self { phi, indexer }
    }

    fn detect(&self, p: &Point2d<i32>) -> bool {
        let index = self.indexer.get(p) as usize;
        let phi_p = self.phi.borrow()[index];
        if phi_p >= 0.0 {
            let q = p + NEIGHBORING_POINTS2D.get(-1, 0);
            if self.is_negative(&phi_p, &q) {
                return true;
            }

            let q = p + NEIGHBORING_POINTS2D.get(1, 0);
            if self.is_negative(&phi_p, &q) {
                return true;
            }

            let q = p + NEIGHBORING_POINTS2D.get(0, -1);
            if self.is_negative(&phi_p, &q) {
                return true;
            }

            let q = p + NEIGHBORING_POINTS2D.get(0, 1);
            if self.is_negative(&phi_p, &q) {
                return true;
            }
            return false;
        } else {
            let q = p + NEIGHBORING_POINTS2D.get(-1, 0);
            if self.is_positive(&phi_p, &q) {
                return true;
            }
            let q = p + NEIGHBORING_POINTS2D.get(1, 0);
            if self.is_positive(&phi_p, &q) {
                return true;
            }
            let q = p + NEIGHBORING_POINTS2D.get(0, -1);
            if self.is_positive(&phi_p, &q) {
                return true;
            }
            let q = p + NEIGHBORING_POINTS2D.get(0, 1);
            if self.is_positive(&phi_p, &q) {
                return true;
            }
            return false;
        }
    }
}

pub struct ZeroLevelSetDetector3d {
    phi: Rc<RefCell<Vec<f64>>>,
    indexer: Rc<Indexer3d>,
}

impl ZeroLevelSetDetector3d {
    fn is_negative(&self, a: &f64, b: &Point3d<i32>) -> bool {
        let index = self.indexer.get(b) as usize;
        a + self.phi.borrow()[index] <= 0.0
    }

    fn is_positive(&self, a: &f64, b: &Point3d<i32>) -> bool {
        let index = self.indexer.get(b) as usize;
        a + self.phi.borrow()[index] > 0.0
    }
}

impl ZeroLevelSetDetectorMethod<Indexer3d, Point3d<i32>> for ZeroLevelSetDetector3d {
    fn new(phi: Rc<RefCell<Vec<f64>>>, indexer: Rc<Indexer3d>) -> Self {
        Self { phi, indexer }
    }

    fn detect(&self, p: &Point3d<i32>) -> bool {
        let index = self.indexer.get(p) as usize;
        let phi_p = self.phi.borrow()[index];
        if phi_p >= 0.0 {
            let q = p + NEIGHBORING_POINTS3D.get(-1, 0, 0);
            if self.is_negative(&phi_p, &q) {
                return true;
            }

            let q = p + NEIGHBORING_POINTS3D.get(1, 0, 0);
            if self.is_negative(&phi_p, &q) {
                return true;
            }

            let q = p + NEIGHBORING_POINTS3D.get(0, -1, 0);
            if self.is_negative(&phi_p, &q) {
                return true;
            }

            let q = p + NEIGHBORING_POINTS3D.get(0, 1, 0);
            if self.is_negative(&phi_p, &q) {
                return true;
            }
            let q = p + NEIGHBORING_POINTS3D.get(0, 0, 1);
            if self.is_negative(&phi_p, &q) {
                return true;
            }

            let q = p + NEIGHBORING_POINTS3D.get(0, 0, -1);
            if self.is_negative(&phi_p, &q) {
                return true;
            }
            return false;
        } else {
            let q = p + NEIGHBORING_POINTS3D.get(-1, 0, 0);
            if self.is_positive(&phi_p, &q) {
                return true;
            }
            let q = p + NEIGHBORING_POINTS3D.get(1, 0, 0);
            if self.is_positive(&phi_p, &q) {
                return true;
            }
            let q = p + NEIGHBORING_POINTS3D.get(0, -1, 0);
            if self.is_positive(&phi_p, &q) {
                return true;
            }
            let q = p + NEIGHBORING_POINTS3D.get(0, 1, 0);
            if self.is_positive(&phi_p, &q) {
                return true;
            }
            let q = p + NEIGHBORING_POINTS3D.get(0, 0, 1);
            if self.is_positive(&phi_p, &q) {
                return true;
            }
            let q = p + NEIGHBORING_POINTS3D.get(0, 0, -1);
            if self.is_positive(&phi_p, &q) {
                return true;
            }
            return false;
        }
    }
}
