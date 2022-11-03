use crate::core::neighboring_point::NEIGHBORING_POINTS2D;
use crate::core::types::{Indexer, IntPoint, TwoDim, Type};

pub const H0D: [f64; 3] = [1.0, 2.0, 1.0];
pub const H1D: [f64; 3] = [-1.0, 0.0, 1.0];
pub const H2D: [f64; 3] = [1.0, -2.0, 1.0];
pub const H3D: [f64; 3] = [1.0, 0.0, -1.0];

/// This class is used to implement Differential class.
pub struct Differential2d<'a> {
    pub indexer: &'a Indexer<TwoDim>,
    pub buffer: &'a Vec<i32>,
    pub values: Vec<i32>,
}

impl<'a> Differential2d<'a> {
    pub fn new(indexer: &'a Indexer<TwoDim>, buffer: &'a Vec<i32>) -> Self {
        let s = 3i32.pow(2);
        let values = vec![0; s as usize];
        Self {
            indexer,
            buffer,
            values,
        }
    }

    fn value(&self, p: &IntPoint<TwoDim>) -> &'a i32 {
        &self.buffer[self.indexer.get(p) as usize]
    }

    fn index(&self, i: i32, j: i32) -> usize {
        ((i + 1) + 3 * (j + 1)) as usize
    }

    fn v(&self, x: i32, y: i32) -> &i32 {
        &self.values[self.index(x, y)]
    }

    fn set_v(&mut self, x: i32, y: i32, v: i32) {
        //self.values[self.index(x, y)] = v;
    }

    fn make_point(&mut self, p: &IntPoint<TwoDim>) {
        //let a = self.value(&(p + NEIGHBORING_POINTS2D.get(-1, -1)));
        //self.set_v(-1, -1, *a);
    }
}
