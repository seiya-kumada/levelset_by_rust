use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::neighboring_point as np;
use crate::core::point::{Point2d, Point3d};
use std::rc::Rc;
pub struct Position2d {
    pub left: i32,
    pub right: i32,
    pub me: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Position2d {
    pub fn new() -> Self {
        Self {
            left: 0,
            right: 0,
            me: 0,
            top: 0,
            bottom: 0,
        }
    }

    // test ok
    pub fn set_position(&mut self, p: &Point2d<i32>, indexer: Rc<Indexer2d>) {
        let a = p + np::NEIGHBORING_POINTS2D.get(-1, 0);
        let b = p + np::NEIGHBORING_POINTS2D.get(1, 0);
        let c = p + np::NEIGHBORING_POINTS2D.get(0, -1);
        let d = p + np::NEIGHBORING_POINTS2D.get(0, 1);
        self.left = indexer.get(&a);
        self.right = indexer.get(&b);
        self.me = indexer.get(p);
        self.top = indexer.get(&c);
        self.bottom = indexer.get(&d);
    }
}

pub struct Position3d {
    pub left: i32,
    pub right: i32,
    pub me: i32,
    pub top: i32,
    pub bottom: i32,
    pub front: i32,
    pub back: i32,
}

impl Position3d {
    pub fn new() -> Self {
        Self {
            left: 0,
            right: 0,
            me: 0,
            top: 0,
            bottom: 0,
            front: 0,
            back: 0,
        }
    }

    pub fn set_position(&mut self, p: &Point3d<i32>, indexer: Rc<Indexer3d>) {
        let a = p + np::NEIGHBORING_POINTS3D.get(-1, 0, 0);
        let b = p + np::NEIGHBORING_POINTS3D.get(1, 0, 0);
        let c = p + np::NEIGHBORING_POINTS3D.get(0, -1, 0);
        let d = p + np::NEIGHBORING_POINTS3D.get(0, 1, 0);
        let e = p + np::NEIGHBORING_POINTS3D.get(0, 0, -1);
        let f = p + np::NEIGHBORING_POINTS3D.get(0, 0, 1);
        self.left = indexer.get(&a);
        self.right = indexer.get(&b);
        self.me = indexer.get(p);
        self.top = indexer.get(&c);
        self.bottom = indexer.get(&d);
        self.front = indexer.get(&e);
        self.back = indexer.get(&f);
    }
}
