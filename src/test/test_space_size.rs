//use crate::core::dimension_types::{ThreeDimension, TwoDimension};
//use crate::core::space_size::SpaceSize;
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn space_size_2d_new() {
//        let a = SpaceSize::<TwoDimension>::new(1, 2);
//        assert_eq!(a.dim.width, 1);
//        assert_eq!(a.dim.height, 2);
//        assert_eq!(a.dim.total, 2);
//    }
//
//    #[test]
//    fn space_size_3d_new() {
//        let a = SpaceSize::<ThreeDimension>::new(1, 2, 3);
//        assert_eq!(a.dim.width, 1);
//        assert_eq!(a.dim.height, 2);
//        assert_eq!(a.dim.depth, 3);
//        assert_eq!(a.dim.total, 6);
//    }
//}
