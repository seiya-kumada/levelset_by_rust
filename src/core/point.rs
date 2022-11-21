use num_traits::Num;
use std::ops::Add;

impl<T: Copy> Clone for Point2d<T> {
    fn clone(&self) -> Self {
        Point2d::<T> {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

#[derive(Copy)]
pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2d<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy> Clone for Point3d<T> {
    fn clone(&self) -> Self {
        Point3d::<T> {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}
pub struct Point3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3d<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
//https://stackoverflow.com/questions/66832882/generics-partial-specialization-in-rust

impl<'a, T: std::ops::Add<Output = T> + Copy> Add for &'a Point2d<T> {
    type Output = Point2d<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point2d::<T>::new(
            self.x.clone() + rhs.x.clone(),
            self.y.clone() + rhs.y.clone(),
        )
    }
}

impl<'a, T: std::ops::Add<Output = T> + Copy> Add for &'a Point3d<T> {
    type Output = Point3d<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point3d::<T>::new(
            self.x.clone() + rhs.x.clone(),
            self.y.clone() + rhs.y.clone(),
            self.z.clone() + rhs.z.clone(),
        )
    }
}
