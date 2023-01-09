use crate::core::differential::{DifferentialDouble2d, DifferentialDouble3d, DifferentialMethod};
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::point::{Point2d, Point3d};
use std::cell::RefCell;
use std::rc::Rc;

pub trait CurvatureGeneratorMethod<Indexer, IntPoint, DoublePoint> {
    fn new(indexer: Rc<Indexer>, phi: Rc<RefCell<Vec<f64>>>) -> Self;
    fn calculate_normal(&mut self, p: &IntPoint) -> DoublePoint;
    fn generate(&mut self, p: &IntPoint) -> f64;
}

pub struct CurvatureGenerator2d {
    differential: DifferentialDouble2d,
}

impl CurvatureGeneratorMethod<Indexer2d, Point2d<i32>, Point2d<f64>> for CurvatureGenerator2d {
    fn new(indexer: Rc<Indexer2d>, phi: Rc<RefCell<Vec<f64>>>) -> Self {
        Self {
            differential: DifferentialDouble2d::new(Rc::clone(&indexer), Rc::clone(&phi)),
        }
    }

    fn calculate_normal(&mut self, p: &Point2d<i32>) -> Point2d<f64> {
        self.differential.make_point(p);
        return Point2d::<f64>::new(self.differential.fx(), self.differential.fy());
    }

    fn generate(&mut self, p: &Point2d<i32>) -> f64 {
        self.differential.make_point(p);
        let dfx = self.differential.fx();
        let dfy = self.differential.fy();

        let dfxy = self.differential.fxy();

        let dfxx = self.differential.fxx();
        let dfyy = self.differential.fyy();

        let dfx2 = dfx * dfx;
        let dfy2 = dfy * dfy;

        let df = (dfx2 + dfy2).sqrt();

        if df != 0.0 {
            return (dfxx * dfy2 + dfyy * dfx2 - 2.0 * dfx * dfy * dfxy) / (df * df * df);
        } else {
            return 0.0;
        }
    }
}

pub struct CurvatureGenerator3d {
    differential: DifferentialDouble3d,
}

impl CurvatureGeneratorMethod<Indexer3d, Point3d<i32>, Point3d<f64>> for CurvatureGenerator3d {
    fn new(indexer: Rc<Indexer3d>, phi: Rc<RefCell<Vec<f64>>>) -> Self {
        Self {
            differential: DifferentialDouble3d::new(Rc::clone(&indexer), Rc::clone(&phi)),
        }
    }

    fn calculate_normal(&mut self, p: &Point3d<i32>) -> Point3d<f64> {
        self.differential.make_point(p);
        return Point3d::<f64>::new(
            self.differential.fx(),
            self.differential.fy(),
            self.differential.fz(),
        );
    }

    fn generate(&mut self, p: &Point3d<i32>) -> f64 {
        self.differential.make_point(p);
        let dfx = self.differential.fx();
        let dfy = self.differential.fy();
        let dfz = self.differential.fz(); // test ok

        let dfxy = self.differential.fxy(); // test ok
        let dfxz = self.differential.fxz(); // test ok
        let dfyz = self.differential.fyz(); // test ok

        let dfxx = self.differential.fxx(); // test ok
        let dfyy = self.differential.fyy(); // test ok
        let dfzz = self.differential.fzz(); // test ok

        let dfx2 = dfx * dfx;
        let dfy2 = dfy * dfy;
        let dfz2 = dfz * dfz;

        let df = (dfx2 + dfy2 + dfz2).sqrt();
        if df != 0.0 {
            return ((dfyy + dfzz) * dfx2 + (dfxx + dfzz) * dfy2 + (dfxx + dfyy) * dfz2
                - 2.0 * dfx * dfy * dfxy
                - 2.0 * dfx * dfz * dfxz
                - 2.0 * dfy * dfz * dfyz)
                / (df * df * df);
        } else {
            return 0.0;
        }
    }
}
