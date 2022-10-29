use crate::core::point::{Point2d, Point3d};
pub struct NeighboringPoints2d {
    points: [Point2d<i32>; 9],
}
pub struct NeighboringPoints3d {
    points: [Point3d<i32>; 27],
}

impl NeighboringPoints2d {
    const fn new() -> Self {
        Self {
            points: [
                Point2d::<i32>::new(-1, -1),
                Point2d::<i32>::new(0, -1),
                Point2d::<i32>::new(1, -1),
                Point2d::<i32>::new(-1, 0),
                Point2d::<i32>::new(0, 0),
                Point2d::<i32>::new(1, 0),
                Point2d::<i32>::new(-1, 1),
                Point2d::<i32>::new(0, 1),
                Point2d::<i32>::new(1, 1),
            ],
        }
    }

    pub fn get(&self, x: i32, y: i32) -> &Point2d<i32> {
        let i: usize = ((1 + x) + 3 * (1 + y)) as usize;
        &self.points[i]
    }
}

impl NeighboringPoints3d {
    const fn new() -> Self {
        Self {
            points: [
                Point3d::<i32>::new(-1, -1, -1),
                Point3d::<i32>::new(0, -1, -1),
                Point3d::<i32>::new(1, -1, -1),
                Point3d::<i32>::new(-1, 0, -1),
                Point3d::<i32>::new(0, 0, -1),
                Point3d::<i32>::new(1, 0, -1),
                Point3d::<i32>::new(-1, 1, -1),
                Point3d::<i32>::new(0, 1, -1),
                Point3d::<i32>::new(1, 1, -1),
                Point3d::<i32>::new(-1, -1, 0),
                Point3d::<i32>::new(0, -1, 0),
                Point3d::<i32>::new(1, -1, 0),
                Point3d::<i32>::new(-1, 0, 0),
                Point3d::<i32>::new(0, 0, 0),
                Point3d::<i32>::new(1, 0, 0),
                Point3d::<i32>::new(-1, 1, 0),
                Point3d::<i32>::new(0, 1, 0),
                Point3d::<i32>::new(1, 1, 0),
                Point3d::<i32>::new(-1, -1, 1),
                Point3d::<i32>::new(0, -1, 1),
                Point3d::<i32>::new(1, -1, 1),
                Point3d::<i32>::new(-1, 0, 1),
                Point3d::<i32>::new(0, 0, 1),
                Point3d::<i32>::new(1, 0, 1),
                Point3d::<i32>::new(-1, 1, 1),
                Point3d::<i32>::new(0, 1, 1),
                Point3d::<i32>::new(1, 1, 1),
            ],
        }
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> &Point3d<i32> {
        let i: usize = ((1 + x) + 3 * (1 + y) + 9 * (1 + z)) as usize;
        &self.points[i]
    }
}

pub const NEIGHBORING_POINTS2D: NeighboringPoints2d = NeighboringPoints2d::new();
pub const NEIGHBORING_POINTS3D: NeighboringPoints3d = NeighboringPoints3d::new();
