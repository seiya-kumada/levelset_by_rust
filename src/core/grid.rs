use super::{
    initial_front::{InitialFront2d, InitialFront3d},
    level_set_method::{LevelSetMethod2d, LevelSetMethod3d},
    point::{Point2d, Point3d},
    space_size::{SpaceSize2d, SpaceSize3d},
};
use std::rc::Rc;

pub trait GridMethod<T, U, L, P> {
    fn create_initial_front(&mut self, front: &T);
    fn create_space_with_edge(space_size: Rc<U>) -> Self;
    fn create_space_without_edge(space_size: Rc<U>) -> Self;
    fn initialize_along_front(&self, lsm: &mut L);
}

#[derive(Clone)]
pub struct Grid2d {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Grid2d {
    pub fn new() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}

impl GridMethod<InitialFront2d, SpaceSize2d, LevelSetMethod2d, Point2d<i32>> for Grid2d {
    fn create_initial_front(&mut self, front: &InitialFront2d) {
        self.left = front.vertices[0].x;
        self.top = front.vertices[0].y;
        self.right = front.vertices[1].x;
        self.bottom = front.vertices[1].y;
    }

    fn create_space_with_edge(space_size: Rc<SpaceSize2d>) -> Self {
        Self {
            left: -1,
            right: space_size.width,
            top: -1,
            bottom: space_size.height,
        }
    }

    fn create_space_without_edge(space_size: Rc<SpaceSize2d>) -> Self {
        Self {
            left: 0,
            right: space_size.width - 1,
            top: 0,
            bottom: space_size.height - 1,
        }
    }

    fn initialize_along_front(&self, lsm: &mut LevelSetMethod2d) {
        for i in self.left..self.right {
            let p = Point2d::<i32>::new(i, self.top);
            lsm.initialize_point_on_front(&p);
        }
        for j in self.top..self.bottom {
            let p = Point2d::<i32>::new(self.right, j);
            lsm.initialize_point_on_front(&p);
        }
        for i in (self.left..self.right).rev() {
            let p = Point2d::<i32>::new(i, self.bottom);
            lsm.initialize_point_on_front(&p);
        }
        for j in (self.top..self.bottom).rev() {
            let p = Point2d::<i32>::new(self.left, j);
            lsm.initialize_point_on_front(&p);
        }
    }
}

#[derive(Clone)]
pub struct Grid3d {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub front: i32,
    pub back: i32,
}

impl Grid3d {
    pub fn new() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            front: 0,
            back: 0,
        }
    }
}

impl GridMethod<InitialFront3d, SpaceSize3d, LevelSetMethod3d, Point3d<i32>> for Grid3d {
    fn create_initial_front(&mut self, front: &InitialFront3d) {
        self.left = front.vertices[0].x;
        self.top = front.vertices[0].y;
        self.right = front.vertices[1].x;
        self.bottom = front.vertices[1].y;
        self.front = front.vertices[0].z;
        self.back = front.vertices[1].z;
    }

    fn create_space_with_edge(space_size: Rc<SpaceSize3d>) -> Self {
        Self {
            left: -1,
            right: space_size.width,
            top: -1,
            bottom: space_size.height,
            front: -1,
            back: space_size.depth,
        }
    }

    fn create_space_without_edge(space_size: Rc<SpaceSize3d>) -> Self {
        Self {
            left: 0,
            right: space_size.width - 1,
            top: 0,
            bottom: space_size.height - 1,
            front: 0,
            back: space_size.depth - 1,
        }
    }

    fn initialize_along_front(&self, lsm: &mut LevelSetMethod3d) {
        for j in self.top..(self.bottom + 1) {
            for i in self.left..(self.right + 1) {
                let p = Point3d::<i32>::new(i, j, self.front);
                lsm.initialize_point_on_front(&p);
                let p = Point3d::<i32>::new(i, j, self.back);
                lsm.initialize_point_on_front(&p);
            }
        }

        for k in (self.front + 1)..self.back {
            for i in self.left..(self.right + 1) {
                let p = Point3d::<i32>::new(i, self.top, k);
                lsm.initialize_point_on_front(&p);
                let p = Point3d::<i32>::new(i, self.bottom, k);
                lsm.initialize_point_on_front(&p);
            }
        }

        for j in (self.top + 1)..self.bottom {
            for k in (self.front + 1)..self.back {
                let p = Point3d::<i32>::new(self.left, j, k);
                lsm.initialize_point_on_front(&p);
                let p = Point3d::<i32>::new(self.right, j, k);
                lsm.initialize_point_on_front(&p);
            }
        }
    }
}
