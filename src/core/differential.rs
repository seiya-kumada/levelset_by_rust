use crate::core::neighboring_point::{NEIGHBORING_POINTS2D, NEIGHBORING_POINTS3D};
use crate::core::types::{Indexer, IntPoint, ThreeDim, TwoDim, Type};

pub struct DifferentialTool;

impl DifferentialTool {
    // test ok
    pub const H0D: [f64; 3] = [1.0, 2.0, 1.0];
    pub const H1D: [f64; 3] = [-1.0, 0.0, 1.0];
    pub const H2D: [f64; 3] = [1.0, -2.0, 1.0];
    pub const H3D: [f64; 3] = [1.0, 0.0, -1.0];

    pub const H0D_TOTAL: i32 = 1 + 2 + 1;

    // test ok
    pub fn index(x: i32) -> usize {
        (x + 1) as usize
    }

    // test ok
    pub fn h(x: i32) -> f64 {
        Self::H0D[Self::index(x)]
    }

    // test ok
    pub fn h1d(x: i32) -> f64 {
        Self::H1D[Self::index(x)]
    }

    // test ok
    pub fn h2d(x: i32) -> f64 {
        Self::H2D[Self::index(x)]
    }

    // test ok
    pub fn h3d(x: i32) -> f64 {
        Self::H3D[Self::index(x)]
    }
}

pub struct Differential2d<'a> {
    pub indexer: &'a Indexer<TwoDim>,
    pub buffer: &'a Vec<i32>,
    pub values: Vec<i32>,
}

impl<'a> Differential2d<'a> {
    // test ok
    pub fn new(indexer: &'a Indexer<TwoDim>, buffer: &'a Vec<i32>) -> Self {
        let s = 3i32.pow(2);
        let values = vec![0; s as usize];
        Self {
            indexer,
            buffer,
            values,
        }
    }

    // test ok
    pub fn h1dx(x: i32, y: i32) -> f64 {
        DifferentialTool::h1d(x) * DifferentialTool::h(y)
    }

    // test ok
    pub fn h1dy(x: i32, y: i32) -> f64 {
        DifferentialTool::h1d(y) * DifferentialTool::h(x)
    }

    // test ok
    pub fn h2dx(x: i32, y: i32) -> f64 {
        DifferentialTool::h2d(x) * DifferentialTool::h(y)
    }

    // test ok
    pub fn h2dy(x: i32, y: i32) -> f64 {
        DifferentialTool::h2d(y) * DifferentialTool::h(x)
    }

    // test ok
    pub fn h3dxy(x: i32, y: i32) -> f64 {
        DifferentialTool::h3d(x) * DifferentialTool::h3d(y)
    }

    // test ok
    pub fn v(&self, x: i32, y: i32) -> i32 {
        self.values[Self::index(x, y)]
    }

    pub fn vx(&self, x: i32, y: i32) -> f64 {
        self.v(x, y) as f64 * Self::h1dx(x, y)
    }

    pub fn vy(&self, x: i32, y: i32) -> f64 {
        self.v(x, y) as f64 * Self::h1dy(x, y)
    }

    pub fn vxx(&self, x: i32, y: i32) -> f64 {
        self.v(x, y) as f64 * Self::h2dx(x, y)
    }

    pub fn vyy(&self, x: i32, y: i32) -> f64 {
        self.v(x, y) as f64 * Self::h2dy(x, y)
    }

    pub fn vxy(&self, x: i32, y: i32) -> f64 {
        self.v(x, y) as f64 * Self::h3dxy(x, y)
    }

    pub fn sobel_x(&self) -> f64 {
        self.vx(-1, -1)
            + self.vx(0, -1)
            + self.vx(1, -1)
            + self.vx(-1, 0)
            + self.vx(0, 0)
            + self.vx(1, 0)
            + self.vx(-1, 1)
            + self.vx(0, 1)
            + self.vx(1, 1)
    }

    pub fn sobel_y(&self) -> f64 {
        self.vy(-1, -1)
            + self.vy(0, -1)
            + self.vy(1, -1)
            + self.vy(-1, 0)
            + self.vy(0, 0)
            + self.vy(1, 0)
            + self.vy(-1, 1)
            + self.vy(0, 1)
            + self.vy(1, 1)
    }

    pub fn fx(&self) -> f64 {
        self.sobel_x() / (2 * DifferentialTool::H0D_TOTAL) as f64
    }

    pub fn fy(&self) -> f64 {
        self.sobel_y() / (2 * DifferentialTool::H0D_TOTAL) as f64
    }

    pub fn fxx(&self) -> f64 {
        (self.vxx(-1, -1)
            + self.vxx(0, -1)
            + self.vxx(1, -1)
            + self.vxx(-1, 0)
            + self.vxx(0, 0)
            + self.vxx(1, 0)
            + self.vxx(-1, 1)
            + self.vxx(0, 1)
            + self.vxx(1, 1))
            / DifferentialTool::H0D_TOTAL as f64
    }

    pub fn fyy(&self) -> f64 {
        (self.vyy(-1, -1)
            + self.vyy(0, -1)
            + self.vyy(1, -1)
            + self.vyy(-1, 0)
            + self.vyy(0, 0)
            + self.vyy(1, 0)
            + self.vyy(-1, 1)
            + self.vyy(0, 1)
            + self.vyy(1, 1))
            / DifferentialTool::H0D_TOTAL as f64
    }

    pub fn fxy(&self) -> f64 {
        (self.vxx(-1, -1)
            + self.vxy(0, -1)
            + self.vxy(1, -1)
            + self.vxy(-1, 0)
            + self.vxy(0, 0)
            + self.vxy(1, 0)
            + self.vxy(-1, 1)
            + self.vxy(0, 1)
            + self.vxy(1, 1))
            / DifferentialTool::H0D_TOTAL as f64
    }

    // test ok
    pub fn value(&self, p: &IntPoint<TwoDim>) -> i32 {
        self.buffer[self.indexer.get(p) as usize]
    }

    // test ok
    pub fn index(i: i32, j: i32) -> usize {
        ((i + 1) + 3 * (j + 1)) as usize
    }

    // test ok
    pub fn set_v(&mut self, x: i32, y: i32, v: i32) {
        let i = Self::index(x, y);
        self.values[i] = v;
    }

    // test ok
    pub fn set_value(&mut self, p: &IntPoint<TwoDim>, x: i32, y: i32) {
        let a = self.value(&(p + NEIGHBORING_POINTS2D.get(x, y)));
        self.set_v(x, y, a);
    }

    // test ok
    pub fn make_point(&mut self, p: &IntPoint<TwoDim>) {
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
pub struct Differential3d<'a> {
    pub indexer: &'a Indexer<ThreeDim>,
    pub buffer: &'a Vec<i32>,
    pub values: Vec<i32>,
}

impl<'a> Differential3d<'a> {
    pub fn new(indexer: &'a Indexer<ThreeDim>, buffer: &'a Vec<i32>) -> Self {
        let s = 3i32.pow(3);
        let values = vec![0; s as usize];
        Self {
            indexer,
            buffer,
            values,
        }
    }

    pub fn value(&self, p: &IntPoint<ThreeDim>) -> i32 {
        self.buffer[self.indexer.get(p) as usize]
    }

    pub fn index(&self, i: i32, j: i32, k: i32) -> usize {
        ((i + 1) + 3 * (j + 1) + 9 * (k + 1)) as usize
    }

    pub fn set_v(&mut self, x: i32, y: i32, z: i32, v: i32) {
        let i = self.index(x, y, z);
        self.values[i] = v;
    }

    pub fn set_value(&mut self, p: &IntPoint<ThreeDim>, x: i32, y: i32, z: i32) {
        let a = self.value(&(p + NEIGHBORING_POINTS3D.get(x, y, z)));
        self.set_v(x, y, z, a);
    }

    pub fn make_point(&mut self, p: &IntPoint<ThreeDim>) {
        self.set_value(p, -1, -1, -1);
        self.set_value(p, 0, -1, -1);
        self.set_value(p, 1, -1, -1);

        self.set_value(p, -1, 0, -1);
        self.set_value(p, 0, 0, -1);
        self.set_value(p, 1, 0, -1);

        self.set_value(p, -1, 1, -1);
        self.set_value(p, 0, 1, -1);
        self.set_value(p, 1, 1, -1);

        self.set_value(p, -1, -1, 0);
        self.set_value(p, 0, -1, 0);
        self.set_value(p, 1, -1, 0);

        self.set_value(p, -1, 0, 0);
        self.set_value(p, 0, 0, 0);
        self.set_value(p, 1, 0, 0);

        self.set_value(p, -1, 1, 0);
        self.set_value(p, 0, 1, 0);
        self.set_value(p, 1, 1, 0);

        self.set_value(p, -1, -1, 1);
        self.set_value(p, 0, -1, 1);
        self.set_value(p, 1, -1, 1);

        self.set_value(p, -1, 0, 1);
        self.set_value(p, 0, 0, 1);
        self.set_value(p, 1, 0, 1);

        self.set_value(p, -1, 1, 1);
        self.set_value(p, 0, 1, 1);
        self.set_value(p, 1, 1, 1);
    }
}
