pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
pub struct Point3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3d<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
//https://stackoverflow.com/questions/66832882/generics-partial-specialization-in-rust
