pub trait SpaceSizeMethod {
    fn get_total(&self) -> usize;
}

#[derive(Clone)]
pub struct SpaceSize2d {
    pub width: i32,
    pub height: i32,
    pub total: i32,
}

impl SpaceSize2d {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            width: w,
            height: h,
            total: w * h,
        }
    }
}

impl SpaceSizeMethod for SpaceSize2d {
    fn get_total(&self) -> usize {
        self.total as usize
    }
}

#[derive(Clone)]
pub struct SpaceSize3d {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub total: i32,
}

impl SpaceSize3d {
    pub fn new(w: i32, h: i32, d: i32) -> Self {
        Self {
            width: w,
            height: h,
            depth: d,
            total: w * h * d,
        }
    }
}

impl SpaceSizeMethod for SpaceSize3d {
    fn get_total(&self) -> usize {
        self.total as usize
    }
}
