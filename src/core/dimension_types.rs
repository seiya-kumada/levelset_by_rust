/// A trait modeling a dimension concept.
pub trait Dimension {
    fn dim() -> usize;
}

/// A set of parameters used for 2-dimension Level Set method.
pub struct TwoDimension {
    pub width: i32,
    pub height: i32,
    pub total: i32,
}

/// A set of parameters used for 3-dimension Level Set method.
pub struct ThreeDimension {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub total: i32,
}

impl TwoDimension {
    /// Constructs an instance of TwoDimension struct.
    pub fn new(width: i32, height: i32) -> Self {
        TwoDimension {
            width,
            height,
            total: width * height,
        }
    }
}

impl ThreeDimension {
    /// Constructs an instance of ThreeDimension struct.
    pub fn new(width: i32, height: i32, depth: i32) -> Self {
        ThreeDimension {
            width,
            height,
            depth,
            total: width * height * depth,
        }
    }
}

/// TwoDimension trait
impl Dimension for TwoDimension {
    fn dim() -> usize {
        2
    }
}

/// ThreeDimension trait
impl Dimension for ThreeDimension {
    fn dim() -> usize {
        3
    }
}

pub mod dimension_ {
    pub const TWO: usize = 2;
    pub const THREE: usize = 3;
}
