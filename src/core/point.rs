use crate::core::dimension_types as dim;
use std::marker::PhantomData;

pub type Point_<T, const D: usize> = [T; D];
//pub type Point2d<T> = [T; dim::TWO];
//pub type Point3d<T> = [T; dim::THREE];
//pub type IntPoint<const D: usize> = [i32; D];

//https://stackoverflow.com/questions/66832882/generics-partial-specialization-in-rust

pub struct NumDim<T, const N: usize> {
    _marker: PhantomData<T>,
}

pub type TwoDim<T> = NumDim<T, 2>;
pub type ThreeDim<T> = NumDim<T, 3>;
pub type IntTwoDim = NumDim<i32, 2>;
pub type IntThreeDim = NumDim<i32, 3>;

pub trait DataType {
    type Data;
}

impl<T, const N: usize> DataType for NumDim<T, N> {
    type Data = Point_<T, N>;
}

pub struct Point<Z>(pub Z::Data)
where
    Z: DataType;
