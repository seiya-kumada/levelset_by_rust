pub struct Grid2d {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Grid2d {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}

pub struct Grid3d {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub front: i32,
    pub back: i32,
}

impl Grid3d {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32, front: i32, back: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
            front,
            back,
        }
    }
}
