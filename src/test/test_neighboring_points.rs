use crate::core::neighboring_point as np;
use crate::core::point::{Point2d, Point3d};
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighboring_points_2d() {
        assert!(*np::NEIGHBORING_POINTS2D.get(-1, -1) == Point2d::<i32>::new(-1, -1));
        assert!(*np::NEIGHBORING_POINTS2D.get(0, -1) == Point2d::<i32>::new(0, -1));
        assert!(*np::NEIGHBORING_POINTS2D.get(1, -1) == Point2d::<i32>::new(1, -1));
        assert!(*np::NEIGHBORING_POINTS2D.get(-1, 0) == Point2d::<i32>::new(-1, 0));
        assert!(*np::NEIGHBORING_POINTS2D.get(0, 0) == Point2d::<i32>::new(0, 0));
        assert!(*np::NEIGHBORING_POINTS2D.get(1, 0) == Point2d::<i32>::new(1, 0));
        assert!(*np::NEIGHBORING_POINTS2D.get(-1, 1) == Point2d::<i32>::new(-1, 1));
        assert!(*np::NEIGHBORING_POINTS2D.get(0, 1) == Point2d::<i32>::new(0, 1));
        assert!(*np::NEIGHBORING_POINTS2D.get(1, 1) == Point2d::<i32>::new(1, 1));
    }

    #[test]
    fn neighboring_points_3d() {
        assert!(*np::NEIGHBORING_POINTS3D.get(-1, -1, 0) == Point3d::<i32>::new(-1, -1, 0));
        assert!(*np::NEIGHBORING_POINTS3D.get(0, -1, 0) == Point3d::<i32>::new(0, -1, 0));
        assert!(*np::NEIGHBORING_POINTS3D.get(1, -1, 0) == Point3d::<i32>::new(1, -1, 0));
        assert!(*np::NEIGHBORING_POINTS3D.get(-1, 0, 0) == Point3d::<i32>::new(-1, 0, 0));
        assert!(*np::NEIGHBORING_POINTS3D.get(0, 0, 0) == Point3d::<i32>::new(0, 0, 0));
        assert!(*np::NEIGHBORING_POINTS3D.get(1, 0, 0) == Point3d::<i32>::new(1, 0, 0));
        assert!(*np::NEIGHBORING_POINTS3D.get(-1, 1, 0) == Point3d::<i32>::new(-1, 1, 0));
        assert!(*np::NEIGHBORING_POINTS3D.get(0, 1, 0) == Point3d::<i32>::new(0, 1, 0));
        assert!(*np::NEIGHBORING_POINTS3D.get(1, 1, 0) == Point3d::<i32>::new(1, 1, 0));

        assert!(*np::NEIGHBORING_POINTS3D.get(-1, -1, -1) == Point3d::<i32>::new(-1, -1, -1));
        assert!(*np::NEIGHBORING_POINTS3D.get(0, -1, -1) == Point3d::<i32>::new(0, -1, -1));
        assert!(*np::NEIGHBORING_POINTS3D.get(1, -1, -1) == Point3d::<i32>::new(1, -1, -1));
        assert!(*np::NEIGHBORING_POINTS3D.get(-1, 0, -1) == Point3d::<i32>::new(-1, 0, -1));
        assert!(*np::NEIGHBORING_POINTS3D.get(0, 0, -1) == Point3d::<i32>::new(0, 0, -1));
        assert!(*np::NEIGHBORING_POINTS3D.get(1, 0, -1) == Point3d::<i32>::new(1, 0, -1));
        assert!(*np::NEIGHBORING_POINTS3D.get(-1, 1, -1) == Point3d::<i32>::new(-1, 1, -1));
        assert!(*np::NEIGHBORING_POINTS3D.get(0, 1, -1) == Point3d::<i32>::new(0, 1, -1));
        assert!(*np::NEIGHBORING_POINTS3D.get(1, 1, -1) == Point3d::<i32>::new(1, 1, -1));

        assert!(*np::NEIGHBORING_POINTS3D.get(-1, -1, 1) == Point3d::<i32>::new(-1, -1, 1));
        assert!(*np::NEIGHBORING_POINTS3D.get(0, -1, 1) == Point3d::<i32>::new(0, -1, 1));
        assert!(*np::NEIGHBORING_POINTS3D.get(1, -1, 1) == Point3d::<i32>::new(1, -1, 1));
        assert!(*np::NEIGHBORING_POINTS3D.get(-1, 0, 1) == Point3d::<i32>::new(-1, 0, 1));
        assert!(*np::NEIGHBORING_POINTS3D.get(0, 0, 1) == Point3d::<i32>::new(0, 0, 1));
        assert!(*np::NEIGHBORING_POINTS3D.get(1, 0, 1) == Point3d::<i32>::new(1, 0, 1));
        assert!(*np::NEIGHBORING_POINTS3D.get(-1, 1, 1) == Point3d::<i32>::new(-1, 1, 1));
        assert!(*np::NEIGHBORING_POINTS3D.get(0, 1, 1) == Point3d::<i32>::new(0, 1, 1));
        assert!(*np::NEIGHBORING_POINTS3D.get(1, 1, 1) == Point3d::<i32>::new(1, 1, 1));
    }
}
