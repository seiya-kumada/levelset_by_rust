use crate::core::dimension_types::dimension_ as dim;
pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

pub struct Point3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Point_<T, const D: usize> = [T; D];
pub type Point2d_<T> = [T; dim::TWO];
pub type Point3d_<T> = [T; dim::THREE];
pub type IntPoint<const D: usize> = [i32; D];
