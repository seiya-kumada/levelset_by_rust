use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::status::Status;
use std::ops::Range;
use std::rc::Rc;
pub struct GridRange2d {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

impl GridRange2d {
    pub fn new(space_size: &SpaceSize2d) -> Self {
        Self {
            x_range: (0, space_size.width),
            y_range: (0, space_size.height),
        }
    }

    pub fn foreach(
        &self,
        indexer: &Indexer2d,
        statuses: &Vec<Status>,
        band: &mut Vec<Point2d<i32>>,
        fun: fn(&Indexer2d, &Vec<Status>, &mut Vec<Point2d<i32>>, Point2d<i32>),
    ) {
        for j in self.y_range.0..self.y_range.1 {
            for i in self.x_range.0..self.x_range.1 {
                fun(indexer, statuses, band, Point2d::<i32>::new(i, j));
            }
        }
    }
}

pub struct GridRange3d {
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
}

impl GridRange3d {
    pub fn new(space_size: &SpaceSize3d) -> Self {
        Self {
            x_range: (0, space_size.width),
            y_range: (0, space_size.height),
            z_range: (0, space_size.depth),
        }
    }

    pub fn foreach(
        &self,
        indexer: &Indexer3d,
        statuses: &Vec<Status>,
        band: &mut Vec<Point3d<i32>>,
        fun: fn(&Indexer3d, &Vec<Status>, &mut Vec<Point3d<i32>>, Point3d<i32>),
    ) {
        for k in self.z_range.0..self.z_range.1 {
            for j in self.y_range.0..self.y_range.1 {
                for i in self.x_range.0..self.x_range.1 {
                    fun(indexer, statuses, band, Point3d::<i32>::new(i, j, k));
                }
            }
        }
    }
}
