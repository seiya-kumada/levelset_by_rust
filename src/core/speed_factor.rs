use crate::core::differential::{Differential2d, Differential3d, DifferentialMethod};
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use std::cell::RefCell;
use std::rc::Rc;

use super::speed::Speed;
pub trait SpeedFactorMethod<T, P, S> {
    fn new(indexer: Rc<T>, gray: Rc<RefCell<Vec<u8>>>) -> Self;
    fn get_value(&self, p: &P) -> f64;
    fn calculate_all(&mut self, space_size: &Rc<S>);
}

pub struct SpeedFactor2d {
    indexer: Rc<Indexer2d>,
    differential: Differential2d<u8>,
    factors: Vec<f64>,
}

impl SpeedFactorMethod<Indexer2d, Point2d<i32>, SpaceSize2d> for SpeedFactor2d {
    fn new(indexer: Rc<Indexer2d>, gray: Rc<RefCell<Vec<u8>>>) -> Self {
        Self {
            indexer: Rc::clone(&indexer),
            differential: Differential2d::<u8>::new(Rc::clone(&indexer), Rc::clone(&gray)),
            factors: Vec::<f64>::new(),
        }
    }

    fn get_value(&self, p: &Point2d<i32>) -> f64 {
        use crate::core::indexer::IndexerMethod;
        self.factors[self.indexer.get(p) as usize]
    }

    fn calculate_all(&mut self, space_size: &Rc<SpaceSize2d>) {
        let w = space_size.width as usize;
        let h = space_size.height as usize;
        self.factors.resize(w * h, 0.0);
        for j in 1..(h - 1) {
            let wj = w * j;
            for i in 1..(w - 1) {
                let p = Point2d::<i32>::new(i as i32, j as i32);
                self.factors[wj + i] = self.calculate(&p);
            }
        }
    }
}

impl SpeedFactor2d {
    fn calculate(&mut self, p: &Point2d<i32>) -> f64 {
        self.differential.make_point(p);
        let dx = self.differential.fx();
        let dy = self.differential.fy();
        1.0 / (1.0 + (dx * dx + dy * dy).sqrt())
    }
}
pub struct SpeedFactor3d {
    indexer: Rc<Indexer3d>,
    differential: Differential3d<u8>,
    factors: Vec<f64>,
}

impl SpeedFactorMethod<Indexer3d, Point3d<i32>, SpaceSize3d> for SpeedFactor3d {
    fn new(indexer: Rc<Indexer3d>, gray: Rc<RefCell<Vec<u8>>>) -> Self {
        Self {
            indexer: Rc::clone(&indexer),
            differential: Differential3d::<u8>::new(Rc::clone(&indexer), Rc::clone(&gray)),
            factors: Vec::<f64>::new(),
        }
    }

    fn get_value(&self, p: &Point3d<i32>) -> f64 {
        use crate::core::indexer::IndexerMethod;
        self.factors[self.indexer.get(p) as usize]
    }

    fn calculate_all(&mut self, space_size: &Rc<SpaceSize3d>) {
        let w = space_size.width as usize;
        let h = space_size.height as usize;
        let a = w * h;
        let d = space_size.depth as usize;

        self.factors.resize(a * d, 0.0);
        for k in 1..(d - 1) {
            let ak = a * k;
            for j in 1..(h - 1) {
                let wj = w * j + ak;
                for i in 1..(w - 1) {
                    let p = Point3d::<i32>::new(i as i32, j as i32, k as i32);
                    self.factors[wj + i] = self.calculate(&p);
                }
            }
        }
    }
}

impl SpeedFactor3d {
    pub fn get_factors(&mut self) -> &mut Vec<f64> {
        &mut self.factors
    }

    fn calculate(&mut self, p: &Point3d<i32>) -> f64 {
        self.differential.make_point(p);
        let dx = self.differential.fx();
        let dy = self.differential.fy();
        let dz = self.differential.fz();
        1.0 / (1.0 + (dx * dx + dy * dy + dz * dz).sqrt())
    }
}
