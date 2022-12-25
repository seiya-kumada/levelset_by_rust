use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::status::Status;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use crate::core::level_set_method::{LevelSetMethod2d, LevelSetMethod3d};
pub trait GridRangeMethod<T, I, P, L> {
    fn new(space_size: &T) -> Self;
    fn foreach_band(
        &self,
        indexer: &I,
        statuses: RefCell<Vec<Status>>,
        band: &mut Vec<P>,
        fun: fn(&I, RefCell<Vec<Status>>, &mut Vec<P>, P),
    );
    fn foreach_phi(&self, lsm: &L);
}

pub struct GridRange2d {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

impl GridRangeMethod<SpaceSize2d, Indexer2d, Point2d<i32>, LevelSetMethod2d> for GridRange2d {
    fn new(space_size: &SpaceSize2d) -> Self {
        Self {
            x_range: (0, space_size.width),
            y_range: (0, space_size.height),
        }
    }

    fn foreach_band(
        &self,
        indexer: &Indexer2d,
        statuses: RefCell<Vec<Status>>,
        band: &mut Vec<Point2d<i32>>,
        fun: fn(&Indexer2d, RefCell<Vec<Status>>, &mut Vec<Point2d<i32>>, Point2d<i32>),
    ) {
        for j in self.y_range.0..self.y_range.1 {
            for i in self.x_range.0..self.x_range.1 {
                fun(
                    indexer,
                    RefCell::clone(&statuses),
                    band,
                    Point2d::<i32>::new(i, j),
                );
            }
        }
    }

    fn foreach_phi(&self, lsm: &LevelSetMethod2d) {
        for j in self.y_range.0..self.y_range.1 {
            for i in self.x_range.0..self.x_range.1 {
                let p = Point2d::<i32>::new(i, j);
                lsm.register_to_phi_(&p);
            }
        }
    }
}

pub struct GridRange3d {
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
}

impl GridRangeMethod<SpaceSize3d, Indexer3d, Point3d<i32>, LevelSetMethod3d> for GridRange3d {
    fn new(space_size: &SpaceSize3d) -> Self {
        Self {
            x_range: (0, space_size.width),
            y_range: (0, space_size.height),
            z_range: (0, space_size.depth),
        }
    }

    fn foreach_band(
        &self,
        indexer: &Indexer3d,
        statuses: RefCell<Vec<Status>>,
        band: &mut Vec<Point3d<i32>>,
        fun: fn(&Indexer3d, RefCell<Vec<Status>>, &mut Vec<Point3d<i32>>, Point3d<i32>),
    ) {
        for k in self.z_range.0..self.z_range.1 {
            for j in self.y_range.0..self.y_range.1 {
                for i in self.x_range.0..self.x_range.1 {
                    fun(
                        indexer,
                        RefCell::clone(&statuses),
                        band,
                        Point3d::<i32>::new(i, j, k),
                    );
                }
            }
        }
    }

    fn foreach_phi(&self, lsm: &LevelSetMethod3d) {
        for k in self.z_range.0..self.z_range.1 {
            for j in self.y_range.0..self.y_range.1 {
                for i in self.x_range.0..self.x_range.1 {
                    let p = Point3d::<i32>::new(i, j, k);
                    lsm.register_to_phi_(&p);
                }
            }
        }
    }
}
