use crate::core::point::Point2d;
use crate::core::point::Point3d;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_2d() {
        let a = Point2d { x: 1, y: 2 };
        assert_eq!(a.x, 1);
        assert_eq!(a.y, 2);

        let a = Point2d { x: 1.0, y: 2.0 };
        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 2.0);

        //let a: Point2d_<i32> = [1, 2];
        //assert_eq!(a[0], 1);
        //assert_eq!(a[1], 2);

        //let a: Point2d_<f64> = [3.0, 4.0];
        //assert_eq!(a[0], 3.0);
        //assert_eq!(a[1], 4.0);
    }

    #[test]
    fn point_3d() {
        let a = Point3d { x: 1, y: 2, z: 3 };
        assert_eq!(a.x, 1);
        assert_eq!(a.y, 2);
        assert_eq!(a.z, 3);

        let a = Point3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 2.0);
        assert_eq!(a.z, 3.0);
    }
}
