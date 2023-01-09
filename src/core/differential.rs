use crate::core::dim;
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::neighboring_point::{NEIGHBORING_POINTS2D, NEIGHBORING_POINTS3D};
use crate::core::point::{Point2d, Point3d};
use num_traits::cast::ToPrimitive;
use num_traits::Zero;
use std::cell::RefCell;
use std::rc::Rc;
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

pub trait DifferentialMethod<IntPoint> {
    fn make_point(&mut self, p: &IntPoint);
}

pub struct Differential2d<T: ToPrimitive + Zero + Clone + Copy> {
    pub indexer: Rc<Indexer2d>,
    pub buffer: Rc<RefCell<Vec<T>>>,
    pub values: Vec<T>,
}

impl<T: ToPrimitive + Zero + Clone + Copy> DifferentialMethod<Point2d<i32>> for Differential2d<T> {
    fn make_point(&mut self, p: &Point2d<i32>) {
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

impl<T: ToPrimitive + Zero + Clone + Copy> Differential2d<T> {
    pub fn new(indexer: Rc<Indexer2d>, buffer: Rc<RefCell<Vec<T>>>) -> Self {
        let s = 3usize.pow(dim::TWO as u32);
        let values = vec![T::zero(); s];
        Self {
            indexer: Rc::clone(&indexer),
            buffer: Rc::clone(&buffer),
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
    pub fn index(i: i32, j: i32) -> usize {
        ((i + 1) + 3 * (j + 1)) as usize
    }

    // test ok
    pub fn v(&self, x: i32, y: i32) -> T {
        self.values[Self::index(x, y)]
    }

    // test ok
    pub fn vx(&self, x: i32, y: i32) -> f64 {
        self.v(x, y).to_f64().unwrap() * Self::h1dx(x, y)
    }

    // test ok
    pub fn vy(&self, x: i32, y: i32) -> f64 {
        self.v(x, y).to_f64().unwrap() * Self::h1dy(x, y)
    }

    // test ok
    pub fn vxx(&self, x: i32, y: i32) -> f64 {
        self.v(x, y).to_f64().unwrap() * Self::h2dx(x, y)
    }

    // test ok
    pub fn vyy(&self, x: i32, y: i32) -> f64 {
        self.v(x, y).to_f64().unwrap() * Self::h2dy(x, y)
    }

    // test ok
    pub fn vxy(&self, x: i32, y: i32) -> f64 {
        self.v(x, y).to_f64().unwrap() * Self::h3dxy(x, y)
    }

    fn sobel(&self, f: fn(&Self, x: i32, y: i32) -> f64) -> f64 {
        f(&self, -1, -1)
            + f(&self, 0, -1)
            + f(&self, 1, -1)
            + f(&self, -1, 0)
            + f(&self, 0, 0)
            + f(&self, 1, 0)
            + f(&self, -1, 1)
            + f(&self, 0, 1)
            + f(&self, 1, 1)
    }

    // test ok
    pub fn sobel_x(&self) -> f64 {
        self.sobel(Self::vx)
    }

    // test ok
    pub fn sobel_y(&self) -> f64 {
        self.sobel(Self::vy)
    }

    // test ok
    pub fn fx(&self) -> f64 {
        self.sobel_x() / (2 * DifferentialTool::H0D_TOTAL) as f64
    }

    // test ok
    pub fn fy(&self) -> f64 {
        self.sobel_y() / (2 * DifferentialTool::H0D_TOTAL) as f64
    }

    // test ok
    pub fn fxx(&self) -> f64 {
        self.sobel(Self::vxx) / 4.0
    }

    // test ok
    pub fn fyy(&self) -> f64 {
        self.sobel(Self::vyy) / 4.0
    }

    // test ok
    pub fn fxy(&self) -> f64 {
        self.sobel(Self::vxy) / 4.0
    }

    // test ok
    pub fn value(&self, p: &Point2d<i32>) -> T {
        use crate::core::indexer::IndexerMethod;
        self.buffer.borrow()[self.indexer.get(p) as usize]
    }

    // test ok
    pub fn set_v(&mut self, x: i32, y: i32, v: T) {
        let i = Self::index(x, y);
        self.values[i] = v;
    }

    // test ok
    pub fn set_value(&mut self, p: &Point2d<i32>, x: i32, y: i32) {
        let a = self.value(&(p + NEIGHBORING_POINTS2D.get(x, y)));
        self.set_v(x, y, a);
    }
}
pub struct Differential3d<T: ToPrimitive + Zero + Clone + Copy> {
    pub indexer: Rc<Indexer3d>,
    pub buffer: Rc<RefCell<Vec<T>>>,
    pub values: Vec<T>,
}

impl<T: ToPrimitive + Zero + Clone + Copy> DifferentialMethod<Point3d<i32>> for Differential3d<T> {
    fn make_point(&mut self, p: &Point3d<i32>) {
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

impl<T: ToPrimitive + Zero + Clone + Copy> Differential3d<T> {
    // test ok
    pub fn new(indexer: Rc<Indexer3d>, buffer: Rc<RefCell<Vec<T>>>) -> Self {
        let s = 3usize.pow(dim::THREE as u32);
        let values = vec![T::zero(); s];
        Self {
            indexer: Rc::clone(&indexer),
            buffer: Rc::clone(&buffer),
            values,
        }
    }

    // test ok
    pub fn h1dx(x: i32, y: i32, z: i32) -> f64 {
        DifferentialTool::h1d(x) * DifferentialTool::h(y) * DifferentialTool::h(z)
    }

    // test ok
    pub fn h1dy(x: i32, y: i32, z: i32) -> f64 {
        DifferentialTool::h(x) * DifferentialTool::h1d(y) * DifferentialTool::h(z)
    }

    // test ok
    pub fn h2dx(x: i32, y: i32, z: i32) -> f64 {
        DifferentialTool::h2d(x) * DifferentialTool::h(y) * DifferentialTool::h(z)
    }

    pub fn h2dy(x: i32, y: i32, z: i32) -> f64 {
        DifferentialTool::h(x) * DifferentialTool::h2d(y) * DifferentialTool::h(z)
    }

    pub fn h3dxy(x: i32, y: i32, z: i32) -> f64 {
        DifferentialTool::h3d(x) * DifferentialTool::h3d(y) * DifferentialTool::h(z)
    }

    pub fn hdz(x: i32, y: i32, z: i32) -> f64 {
        DifferentialTool::h(x) * DifferentialTool::h(y) * DifferentialTool::h1d(z)
    }

    pub fn h2dz(x: i32, y: i32, z: i32) -> f64 {
        DifferentialTool::h(x) * DifferentialTool::h(y) * DifferentialTool::h2d(z)
    }

    pub fn h3dxz(x: i32, y: i32, z: i32) -> f64 {
        DifferentialTool::h3d(x) * DifferentialTool::h(y) * DifferentialTool::h3d(z)
    }

    pub fn h3dyz(x: i32, y: i32, z: i32) -> f64 {
        DifferentialTool::h(x) * DifferentialTool::h3d(y) * DifferentialTool::h3d(z)
    }

    pub fn index(i: i32, j: i32, k: i32) -> usize {
        ((i + 1) + 3 * (j + 1) + 9 * (k + 1)) as usize
    }

    pub fn v(&self, x: i32, y: i32, z: i32) -> T {
        self.values[Self::index(x, y, z)]
    }

    pub fn vx(&self, x: i32, y: i32, z: i32) -> f64 {
        self.v(x, y, z).to_f64().unwrap() * Self::h1dx(x, y, z)
    }

    pub fn vy(&self, x: i32, y: i32, z: i32) -> f64 {
        self.v(x, y, z).to_f64().unwrap() * Self::h1dy(x, y, z)
    }

    pub fn vz(&self, x: i32, y: i32, z: i32) -> f64 {
        self.v(x, y, z).to_f64().unwrap() * Self::hdz(x, y, z)
    }

    pub fn vxx(&self, x: i32, y: i32, z: i32) -> f64 {
        self.v(x, y, z).to_f64().unwrap() * Self::h2dx(x, y, z)
    }

    pub fn vyy(&self, x: i32, y: i32, z: i32) -> f64 {
        self.v(x, y, z).to_f64().unwrap() * Self::h2dy(x, y, z)
    }

    pub fn vzz(&self, x: i32, y: i32, z: i32) -> f64 {
        self.v(x, y, z).to_f64().unwrap() * Self::h2dz(x, y, z)
    }

    pub fn vxy(&self, x: i32, y: i32, z: i32) -> f64 {
        self.v(x, y, z).to_f64().unwrap() * Self::h3dxy(x, y, z)
    }

    pub fn vxz(&self, x: i32, y: i32, z: i32) -> f64 {
        self.v(x, y, z).to_f64().unwrap() * Self::h3dxz(x, y, z)
    }

    pub fn vyz(&self, x: i32, y: i32, z: i32) -> f64 {
        self.v(x, y, z).to_f64().unwrap() * Self::h3dyz(x, y, z)
    }

    fn sobel(&self, f: fn(&Self, x: i32, y: i32, z: i32) -> f64) -> f64 {
        f(&self, -1, -1, -1)
            + f(&self, 0, -1, -1)
            + f(&self, 1, -1, -1)
            + f(&self, -1, 0, -1)
            + f(&self, 0, 0, -1)
            + f(&self, 1, 0, -1)
            + f(&self, -1, 1, -1)
            + f(&self, 0, 1, -1)
            + f(&self, 1, 1, -1)
            + f(&self, -1, -1, 0)
            + f(&self, 0, -1, 0)
            + f(&self, 1, -1, 0)
            + f(&self, -1, 0, 0)
            + f(&self, 0, 0, 0)
            + f(&self, 1, 0, 0)
            + f(&self, -1, 1, 0)
            + f(&self, 0, 1, 0)
            + f(&self, 1, 1, 0)
            + f(&self, -1, -1, 1)
            + f(&self, 0, -1, 1)
            + f(&self, 1, -1, 1)
            + f(&self, -1, 0, 1)
            + f(&self, 0, 0, 1)
            + f(&self, 1, 0, 1)
            + f(&self, -1, 1, 1)
            + f(&self, 0, 1, 1)
            + f(&self, 1, 1, 1)
    }

    // test ok
    pub fn sobel_x(&self) -> f64 {
        self.sobel(Self::vx)
    }

    // test ok
    pub fn sobel_y(&self) -> f64 {
        self.sobel(Self::vy)
    }

    // test ok
    pub fn sobel_z(&self) -> f64 {
        self.sobel(Self::vz)
    }

    // test ok
    pub fn fx(&self) -> f64 {
        self.sobel_x() / 32.0
    }

    // test ok
    pub fn fy(&self) -> f64 {
        self.sobel_y() / 32.0
    }

    // test ok
    pub fn fz(&self) -> f64 {
        self.sobel_z() / 32.0
    }

    // test ok
    pub fn fxx(&self) -> f64 {
        self.sobel(Self::vxx) / 16.0
    }

    // test ok
    pub fn fyy(&self) -> f64 {
        self.sobel(Self::vyy) / 16.0
    }

    // test ok
    pub fn fzz(&self) -> f64 {
        self.sobel(Self::vzz) / 16.0
    }

    // test ok
    pub fn fxy(&self) -> f64 {
        self.sobel(Self::vxy) / 16.0
    }

    // test ok
    pub fn fxz(&self) -> f64 {
        self.sobel(Self::vxz) / 16.0
    }

    // test ok
    pub fn fyz(&self) -> f64 {
        self.sobel(Self::vyz) / 16.0
    }

    pub fn value(&self, p: &Point3d<i32>) -> T {
        use crate::core::indexer::IndexerMethod;
        self.buffer.borrow()[self.indexer.get(p) as usize]
    }

    pub fn set_v(&mut self, x: i32, y: i32, z: i32, v: T) {
        let i = Self::index(x, y, z);
        self.values[i] = v;
    }

    pub fn set_value(&mut self, p: &Point3d<i32>, x: i32, y: i32, z: i32) {
        let a = self.value(&(p + NEIGHBORING_POINTS3D.get(x, y, z)));
        self.set_v(x, y, z, a);
    }
}

pub type DifferentialDouble2d = Differential2d<f64>;
pub type DifferentialDouble3d = Differential3d<f64>;
