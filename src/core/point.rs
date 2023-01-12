use num_traits::Num;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

impl<T: Copy> Clone for Point2d<T> {
    fn clone(&self) -> Self {
        Point2d::<T> {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

#[derive(Copy, Hash, PartialEq, Eq, Debug)]
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

#[derive(Copy, Hash, PartialEq, Eq, Debug)]
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

pub trait PointMethod {
    type Type;
    fn add(&self, rhs: &Self::Type) -> Self::Type;
    fn print(&self);
}

impl<T: std::fmt::Display + std::ops::Add<Output = T> + Clone> PointMethod for Point2d<T> {
    type Type = Point2d<T>;
    fn add(&self, rhs: &Self::Type) -> Self::Type {
        Point2d::<T>::new(
            self.x.clone() + rhs.x.clone(),
            self.y.clone() + rhs.y.clone(),
        )
    }

    fn print(&self) {
        println!("{},{}", self.x, self.y);
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

impl<T: std::fmt::Display + std::ops::Add<Output = T> + Clone> PointMethod for Point3d<T> {
    type Type = Point3d<T>;
    fn add(&self, rhs: &Self::Type) -> Self::Type {
        Point3d::<T>::new(
            self.x.clone() + rhs.x.clone(),
            self.y.clone() + rhs.y.clone(),
            self.z.clone() + rhs.z.clone(),
        )
    }

    fn print(&self) {
        println!("{},{},{}", self.x, self.y, self.z);
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
