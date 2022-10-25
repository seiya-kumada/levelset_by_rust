use crate::core::types::{Indexer, IntPoint, SpaceSize, ThreeDim, TwoDim};

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_indexer2d() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let p = IntPoint::<TwoDim>::new(1, 2);
        let q = indexer.get(&p);
        assert_eq!(q, 1 + 1 * 2);
    }

    fn test_indexer3d() {
        let space_size = SpaceSize::<ThreeDim>::new(1, 2, 3);
        let indexer = Indexer::<ThreeDim>::new(&space_size);
        let p = IntPoint::<ThreeDim>::new(1, 2, 3);
        let q = indexer.get(&p);
        assert_eq!(q, 1 + 1 * 2 + 3 * 2);
    }
}
