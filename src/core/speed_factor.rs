use crate::core::types::{DifferentialU8, Indexer, IntPoint, SpaceSize, ThreeDim, TwoDim, Type};
use std::rc::Rc;

pub struct SpeedFactor2d {
    indexer: Rc<Indexer<TwoDim>>,
    differential: DifferentialU8<TwoDim>,
    factors: Vec<f64>,
}

impl SpeedFactor2d {
    pub fn new(indexer: Rc<Indexer<TwoDim>>, gray: Rc<Vec<u8>>) -> Self {
        Self {
            indexer: Rc::clone(&indexer),
            differential: DifferentialU8::<TwoDim>::new(Rc::clone(&indexer), Rc::clone(&gray)),
            factors: Vec::<f64>::new(),
        }
    }

    pub fn get_value(&self, p: &IntPoint<TwoDim>) -> f64 {
        self.factors[self.indexer.get(p) as usize]
    }

    fn calculate(&mut self, p: &IntPoint<TwoDim>) -> f64 {
        self.differential.make_point(p);
        let dx = self.differential.fx();
        let dy = self.differential.fy();
        1.0 / (1.0 + (dx * dx + dy * dy).sqrt())
    }

    fn calculate_all(&mut self, space_size: &SpaceSize<TwoDim>) {
        let w = space_size.width as usize;
        let h = space_size.height as usize;
        self.factors.resize(w * h, 0.0);
        for j in 1..(h - 1) {
            let wj = w * j;
            for i in 1..(w - 1) {
                let p = IntPoint::<TwoDim> {
                    x: i as i32,
                    y: j as i32,
                };
                self.factors[wj + i] = self.calculate(&p);
            }
        }
    }
}
pub struct SpeedFactor3d {
    indexer: Rc<Indexer<ThreeDim>>,
    differential: DifferentialU8<ThreeDim>,
    factors: Vec<f64>,
}

impl SpeedFactor3d {
    pub fn new(indexer: Rc<Indexer<ThreeDim>>, gray: Rc<Vec<u8>>) -> Self {
        Self {
            indexer: Rc::clone(&indexer),
            differential: DifferentialU8::<ThreeDim>::new(Rc::clone(&indexer), Rc::clone(&gray)),
            factors: Vec::<f64>::new(),
        }
    }

    pub fn get_factors(&mut self) -> &mut Vec<f64> {
        &mut self.factors
    }

    pub fn get_value(&self, p: &IntPoint<ThreeDim>) -> f64 {
        self.factors[self.indexer.get(p) as usize]
    }

    fn calculate(&mut self, p: &IntPoint<ThreeDim>) -> f64 {
        self.differential.make_point(p);
        let dx = self.differential.fx();
        let dy = self.differential.fy();
        let dz = self.differential.fz();
        1.0 / (1.0 + (dx * dx + dy * dy + dz * dz).sqrt())
    }

    fn calculate_all(&mut self, space_size: &SpaceSize<ThreeDim>) {
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
                    let p = IntPoint::<ThreeDim> {
                        x: i as i32,
                        y: j as i32,
                        z: k as i32,
                    };
                    self.factors[wj + i] = self.calculate(&p);
                }
            }
        }
    }
}
