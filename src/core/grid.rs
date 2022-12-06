use super::{
    initial_front::{InitialFront2d, InitialFront3d},
    space_size::{SpaceSize2d, SpaceSize3d},
};
use std::rc::Rc;

pub trait GridMethod<T> {
    fn create_initial_front(&mut self, front: &T);
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

    pub fn create_space_with_edge(space_size: Rc<SpaceSize2d>) -> Self {
        Self {
            left: -1,
            right: space_size.width,
            top: -1,
            bottom: space_size.height,
        }
    }

    pub fn create_space_without_edge(space_size: Rc<SpaceSize2d>) -> Self {
        Self {
            left: 0,
            right: space_size.width - 1,
            top: 0,
            bottom: space_size.height - 1,
        }
    }
    //pub fn create_initial_front(&mut self, front: &InitialFront2d) {
    //    self.left = front.vertices[0].x;
    //    self.top = front.vertices[0].y;
    //    self.right = front.vertices[1].x;
    //    self.bottom = front.vertices[1].y;
    //}
}

impl GridMethod<InitialFront2d> for Grid2d {
    fn create_initial_front(&mut self, front: &InitialFront2d) {
        self.left = front.vertices[0].x;
        self.top = front.vertices[0].y;
        self.right = front.vertices[1].x;
        self.bottom = front.vertices[1].y;
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

    pub fn create_space_with_edge(space_size: Rc<SpaceSize3d>) -> Self {
        Self {
            left: -1,
            right: space_size.width,
            top: -1,
            bottom: space_size.height,
            front: -1,
            back: space_size.depth,
        }
    }

    pub fn create_space_without_edge(space_size: Rc<SpaceSize3d>) -> Self {
        Self {
            left: 0,
            right: space_size.width - 1,
            top: 0,
            bottom: space_size.height - 1,
            front: 0,
            back: space_size.depth - 1,
        }
    }

    //pub fn create_initial_front(&mut self, front: &InitialFront3d) {
    //    self.left = front.vertices[0].x;
    //    self.top = front.vertices[0].y;
    //    self.right = front.vertices[1].x;
    //    self.bottom = front.vertices[1].y;
    //    self.front = front.vertices[0].z;
    //    self.back = front.vertices[1].z;
    //}
}

impl GridMethod<InitialFront3d> for Grid3d {
    fn create_initial_front(&mut self, front: &InitialFront3d) {
        self.left = front.vertices[0].x;
        self.top = front.vertices[0].y;
        self.right = front.vertices[1].x;
        self.bottom = front.vertices[1].y;
        self.front = front.vertices[0].z;
        self.back = front.vertices[1].z;
    }
}
