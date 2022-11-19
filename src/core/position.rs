use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::neighboring_point as np;
use crate::core::point::{Point2d, Point3d};

pub struct Position2d {
    pub left: i32,
    pub right: i32,
    pub me: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Position2d {
    pub fn new(left: i32, right: i32, me: i32, top: i32, bottom: i32) -> Self {
        Self {
            left,
            right,
            me,
            top,
            bottom,
        }
    }

    fn make_position(p: &Point2d<i32>, indexer: &Indexer2d) -> Self {
        let a = p + np::NEIGHBORING_POINTS2D.get(-1, 0);
        let b = p + np::NEIGHBORING_POINTS2D.get(1, 0);
        let c = p + np::NEIGHBORING_POINTS2D.get(0, -1);
        let d = p + np::NEIGHBORING_POINTS2D.get(0, 1);
        Self {
            left: indexer.get(&a),
            right: indexer.get(&b),
            me: indexer.get(p),
            top: indexer.get(&c),
            bottom: indexer.get(&d),
        }
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
    pub fn new(
        left: i32,
        right: i32,
        me: i32,
        top: i32,
        bottom: i32,
        front: i32,
        back: i32,
    ) -> Self {
        Self {
            left,
            right,
            me,
            top,
            bottom,
            front,
            back,
        }
    }

    fn set_position(p: &Point3d<i32>, indexer: &Indexer3d) -> Self {
        let a = p + np::NEIGHBORING_POINTS3D.get(-1, 0, 0);
        let b = p + np::NEIGHBORING_POINTS3D.get(1, 0, 0);
        let c = p + np::NEIGHBORING_POINTS3D.get(0, -1, 0);
        let d = p + np::NEIGHBORING_POINTS3D.get(0, 1, 0);
        let e = p + np::NEIGHBORING_POINTS3D.get(0, 0, -1);
        let f = p + np::NEIGHBORING_POINTS3D.get(0, 0, 1);
        Self {
            left: indexer.get(&a),
            right: indexer.get(&b),
            me: indexer.get(p),
            top: indexer.get(&c),
            bottom: indexer.get(&d),
            front: indexer.get(&e),
            back: indexer.get(&f),
        }
    }
}
