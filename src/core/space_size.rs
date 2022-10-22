use crate::core::dimension_types::{Dimension, ThreeDimension, TwoDimension};
pub struct SpaceSize<D: Dimension> {
    pub dim: D,
}

impl SpaceSize<TwoDimension> {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            dim: TwoDimension::new(w, h),
        }
    }
}

impl SpaceSize<ThreeDimension> {
    pub fn new(w: i32, h: i32, d: i32) -> Self {
        Self {
            dim: ThreeDimension::new(w, h, d),
        }
    }
}
