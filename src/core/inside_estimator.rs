use crate::core::grid::{Grid2d, Grid3d};
use crate::core::point::{Point2d, Point3d};

pub trait InsideEstimatorMethod<T> {
    fn new() -> Self;
    fn set_grid(&mut self, grid: &T);
    fn from_grid(grid: T) -> Self;
}

pub struct InsideEstimator2d {
    grid: Grid2d,
}

impl InsideEstimatorMethod<Grid2d> for InsideEstimator2d {
    fn new() -> Self {
        Self {
            grid: Grid2d::new(),
        }
    }

    fn set_grid(&mut self, grid: &Grid2d) {
        self.grid = grid.clone();
    }

    fn from_grid(grid: Grid2d) -> Self {
        Self { grid }
    }
}

impl InsideEstimator2d {
    //pub fn new() -> Self {
    //    Self {
    //        grid: Grid2d::new(),
    //    }
    //}

    //pub fn from_grid(grid: Grid2d) -> Self {
    //    Self { grid }
    //}

    pub fn is_inside(&self, p: &Point2d<i32>) -> bool {
        (self.grid.left < p.x)
            && (p.x < self.grid.right)
            && (self.grid.top < p.y)
            && (p.y < self.grid.bottom)
    }

    //pub fn set_grid(&mut self, grid: Grid2d) {
    //    self.grid = grid;
    //}
}

pub struct InsideEstimator3d {
    grid: Grid3d,
}

impl InsideEstimatorMethod<Grid3d> for InsideEstimator3d {
    fn new() -> Self {
        Self {
            grid: Grid3d::new(),
        }
    }

    fn set_grid(&mut self, grid: &Grid3d) {
        self.grid = grid.clone();
    }

    fn from_grid(grid: Grid3d) -> Self {
        Self { grid }
    }
}

impl InsideEstimator3d {
    //pub fn new() -> Self {
    //    Self {
    //        grid: Grid3d::new(),
    //    }
    //}

    //pub fn from_grid(grid: Grid3d) -> Self {
    //    Self { grid }
    //}

    pub fn is_inside(&self, p: &Point3d<i32>) -> bool {
        (self.grid.left < p.x)
            && (p.x < self.grid.right)
            && (self.grid.top < p.y)
            && (p.y < self.grid.bottom)
            && (self.grid.front < p.z)
            && (p.z < self.grid.back)
    }

    pub fn set_grid(&mut self, grid: Grid3d) {
        self.grid = grid;
    }
}
