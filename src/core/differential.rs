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

    fn value(&self, p: &IntPoint<TwoDim>) -> i32 {
        self.buffer[self.indexer.get(p) as usize]
    }

    fn index(&self, i: i32, j: i32) -> usize {
        ((i + 1) + 3 * (j + 1)) as usize
    }

    fn set_v(&mut self, x: i32, y: i32, v: i32) {
        let i = self.index(x, y);
        self.values[i] = v;
    }

    fn set_value(&mut self, p: &IntPoint<TwoDim>, x: i32, y: i32) {
        let a = self.value(&(p + NEIGHBORING_POINTS2D.get(x, y)));
        self.set_v(x, y, a);
    }

    fn make_point(&mut self, p: &IntPoint<TwoDim>) {
        self.set_value(p, -1, -1);
        self.set_value(p, 0, -1);
        self.set_value(p, 1, -1);

        self.set_value(p, -1, 0);
        self.set_value(p, 0, 0);
        self.set_value(p, 1, 0);

        self.set_value(p, -1, 1);
        self.set_value(p, 0, 1);
        self.set_value(p, 1, 1);
    }
}
