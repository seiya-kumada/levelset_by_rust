pub trait Dimension {
    fn dim(&self) -> i32;
}

pub struct TwoDimension {
    pub width: i32,
    pub height: i32,
    pub total: i32,
}
pub struct ThreeDimension {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub total: i32,
}

impl TwoDimension {
    pub fn new(width: i32, height: i32) -> Self {
        TwoDimension {
            width,
            height,
            total: width * height,
        }
    }
}

impl ThreeDimension {
    pub fn new(width: i32, height: i32, depth: i32) -> Self {
        ThreeDimension {
            width,
            height,
            depth,
            total: width * height * depth,
        }
    }
}

impl Dimension for TwoDimension {
    fn dim(&self) -> i32 {
        2
    }
}

impl Dimension for ThreeDimension {
    fn dim(&self) -> i32 {
        3
    }
}
