use crate::core::dimension_types as dim;
use std::marker::PhantomData;

pub type Point_<T, const D: usize> = [T; D];
pub type Point2d<T> = [T; dim::TWO];
pub type Point3d<T> = [T; dim::THREE];
pub type IntPoint<const D: usize> = [i32; D];

//https://stackoverflow.com/questions/66832882/generics-partial-specialization-in-rust

pub struct NumericDim<T, const N: usize> {
    _marker: PhantomData<T>,
}
pub type TwoDim<T> = NumericDim<T, 2>;
pub type ThreeDim<T> = NumericDim<T, 3>;
pub type IntTwoDim = NumericDim<i32, 2>;
pub type IntThreeDim = NumericDim<i32, 3>;

pub trait PointSize {
    type Data;
}

impl<T> PointSize for TwoDim<T> {
    type Data = Point_<T, 2>;
}

impl<T> PointSize for ThreeDim<T> {
    type Data = Point_<T, 3>;
}
//
pub struct Point<Z>(pub Z::Data)
where
    Z: PointSize;
