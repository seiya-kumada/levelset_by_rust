use crate::core::differential::{DifferentialDouble2d, DifferentialDouble3d, DifferentialMethod};
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::point::{Point2d, Point3d};
use std::cell::RefCell;
use std::rc::Rc;

pub trait CurvatureGeneratorMethod<Indexer, IntPoint, DoublePoint> {
    fn new(indexer: Rc<Indexer>, phi: RefCell<Vec<f64>>) -> Self;
    fn calculate_normal(&mut self, p: &IntPoint) -> DoublePoint;
}

pub struct CurvatureGenerator2d {
    differential: DifferentialDouble2d,
}

impl CurvatureGeneratorMethod<Indexer2d, Point2d<i32>, Point2d<f64>> for CurvatureGenerator2d {
    fn new(indexer: Rc<Indexer2d>, phi: RefCell<Vec<f64>>) -> Self {
        Self {
            differential: DifferentialDouble2d::new(Rc::clone(&indexer), RefCell::clone(&phi)),
        }
    }

    fn calculate_normal(&mut self, p: &Point2d<i32>) -> Point2d<f64> {
        self.differential.make_point(p);
        return Point2d::<f64>::new(self.differential.fx(), self.differential.fy());
    }
}

pub struct CurvatureGenerator3d {
    differential: DifferentialDouble3d,
}

impl CurvatureGeneratorMethod<Indexer3d, Point3d<i32>, Point3d<f64>> for CurvatureGenerator3d {
    fn new(indexer: Rc<Indexer3d>, phi: RefCell<Vec<f64>>) -> Self {
        Self {
            differential: DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&phi)),
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
}
