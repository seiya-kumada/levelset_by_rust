//use crate::core::dimension_types as dim;
//use crate::core::indexer::{Indexer2d, Indexer3d};
//use crate::core::point::IntPoint;
//use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
//
//#[cfg(test)]
//mod tests {
//
//    use super::*;
//    #[test]
//    fn test_indexer2d() {
//        let space_size = SpaceSize2d::new(1, 2);
//        let indexer = Indexer2d::new(&space_size);
//        let p: IntPoint<{ dim::TWO }> = [1, 2];
//        let q = indexer.get(&p);
//        assert_eq!(q, 1 + 1 * 2);
//    }
//
//    fn test_indexer3d() {
//        let space_size = SpaceSize3d::new(1, 2, 3);
//        let indexer = Indexer3d::new(&space_size);
//        let p: IntPoint<{ dim::THREE }> = [1, 2, 3];
//        let q = indexer.get(&p);
//        assert_eq!(q, 1 + 1 * 2 + 3 * 2);
//    }
//}
