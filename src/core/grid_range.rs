use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::status::Status;
use std::ops::Range;
use std::rc::Rc;

pub trait GridRangeMethod<T, I, P> {
    fn new(space_size: &T) -> Self;
    fn foreach<K>(
        &self,
        indexer: &I,
        statuses: &Vec<Status>,
        band: &mut K,
        fun: fn(&I, &Vec<Status>, &mut K, P),
    );
}

pub struct GridRange2d {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

impl GridRangeMethod<SpaceSize2d, Indexer2d, Point2d<i32>> for GridRange2d {
    fn new(space_size: &SpaceSize2d) -> Self {
        Self {
            x_range: (0, space_size.width),
            y_range: (0, space_size.height),
        }
    }

    fn foreach<K>(
        &self,
        indexer: &Indexer2d,
        statuses: &Vec<Status>,
        band: &mut K,
        fun: fn(&Indexer2d, &Vec<Status>, &mut K, Point2d<i32>),
    ) {
        for j in self.y_range.0..self.y_range.1 {
            for i in self.x_range.0..self.x_range.1 {
                fun(indexer, statuses, band, Point2d::<i32>::new(i, j));
            }
        }
    }
}

impl GridRange2d {
    //pub fn new(space_size: &SpaceSize2d) -> Self {
    //    Self {
    //        x_range: (0, space_size.width),
    //        y_range: (0, space_size.height),
    //    }
    //}
    //pub fn foreach<T>(
    //    &self,
    //    indexer: &Indexer2d,
    //    statuses: &Vec<Status>,
    //    band: &mut T,
    //    fun: fn(&Indexer2d, &Vec<Status>, &mut T, Point2d<i32>),
    //) {
    //    for j in self.y_range.0..self.y_range.1 {
    //        for i in self.x_range.0..self.x_range.1 {
    //            fun(indexer, statuses, band, Point2d::<i32>::new(i, j));
    //        }
    //    }
    //}
}

pub struct GridRange3d {
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
}

impl GridRangeMethod<SpaceSize3d, Indexer3d, Point3d<i32>> for GridRange3d {
    fn new(space_size: &SpaceSize3d) -> Self {
        Self {
            x_range: (0, space_size.width),
            y_range: (0, space_size.height),
            z_range: (0, space_size.depth),
        }
    }

    fn foreach<K>(
        &self,
        indexer: &Indexer3d,
        statuses: &Vec<Status>,
        band: &mut K,
        fun: fn(&Indexer3d, &Vec<Status>, &mut K, Point3d<i32>),
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

impl GridRange3d {
    //pub fn new(space_size: &SpaceSize3d) -> Self {
    //    Self {
    //        x_range: (0, space_size.width),
    //        y_range: (0, space_size.height),
    //        z_range: (0, space_size.depth),
    //    }
    //}

    //pub fn foreach<T>(
    //    &self,
    //    indexer: &Indexer3d,
    //    statuses: &Vec<Status>,
    //    band: &mut T,
    //    fun: fn(&Indexer3d, &Vec<Status>, &mut T, Point3d<i32>),
    //) {
    //    for k in self.z_range.0..self.z_range.1 {
    //        for j in self.y_range.0..self.y_range.1 {
    //            for i in self.x_range.0..self.x_range.1 {
    //                fun(indexer, statuses, band, Point3d::<i32>::new(i, j, k));
    //            }
    //        }
    //    }
    //}
}
