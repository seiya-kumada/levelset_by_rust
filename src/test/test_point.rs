//use crate::core::dimension_types as dim;
//use crate::core::point::IntPoint;
//use crate::core::point::Point2d;
//use crate::core::point::Point3d;
use crate::core::point::{IntThreeDim, Point};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point() {
        let p = Point::<IntThreeDim>([1, 2, 3]);
        assert_eq!(1, p.0[0]);
        assert_eq!(2, p.0[1]);
        assert_eq!(3, p.0[2]);
    }

    //    //#[test]
    //    //fn point_2d() {
    //    //    let p: Point2d<i32> = [1, 2];
    //    //    assert_eq!(p[0], 1);
    //    //    assert_eq!(p[1], 2);
    //    //}
    //
    //    //#[test]
    //    //fn point_3d() {
    //    //    let p: Point3d<i32> = [1, 2, 3];
    //    //    assert_eq!(p[0], 1);
    //    //    assert_eq!(p[1], 2);
    //    //    assert_eq!(p[2], 3);
    //    //}
}
