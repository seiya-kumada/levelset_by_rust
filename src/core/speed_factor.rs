use crate::core::types::{DifferentialU8, Indexer, IntPoint, SpaceSize, Type};
use std::rc::Rc;

// https://amagramming.netlify.app/2019/03/25/rust-combine-traits
pub struct SpeedFactor<D: Type> {
    indexer: Rc<Indexer<D>>,
    differential: DifferentialU8<D>,
    factors: Vec<f64>,
}

impl<D: Type> SpeedFactor<D> {
    pub fn new(indexer: Rc<Indexer<D>>, gray: Rc<Vec<u8>>) -> Self {
        Self {
            indexer: Rc::clone(&indexer),
            differential: D::make_differential_u8(Rc::clone(&indexer), Rc::clone(&gray)),
            factors: Vec::<f64>::new(),
        }
    }

    pub fn get_factors(&mut self) -> &mut Vec<f64> {
        &mut self.factors
    }

    pub fn get_value(&self, p: &IntPoint<D>) -> f64 {
        self.factors[D::get_index(&self.indexer, p) as usize]
    }
}
