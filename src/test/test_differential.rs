use crate::core::differential as df;
use crate::core::types::{Indexer, IntPoint, SpaceSize, ThreeDim, TwoDim};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn differential2d_new() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3];
        let f = df::Differential2d::new(&indexer, &buffer);

        let id = f.indexer;
        let p = IntPoint::<TwoDim>::new(1, 2);
        let q = indexer.get(&p);
        assert_eq!(q, 1 + 1 * 2);

        let bu = f.buffer;
        assert_eq!(bu[0], 1);
        assert_eq!(bu[1], 2);
        assert_eq!(bu[2], 3);
    }

    #[test]
    fn differential2d_index() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3];
        let f = df::Differential2d::new(&indexer, &buffer);

        let a = f.index(1, 2);
        assert_eq!(2 + 9, a);
    }

    #[test]
    fn differential2d_value() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3, 4];
        let f = df::Differential2d::new(&indexer, &buffer);
        let p = IntPoint::<TwoDim>::new(1, 2);
        assert_eq!(4, f.value(&p));
    }

    #[test]
    fn differential2d_set_v() {
        //let space_size = SpaceSize::<TwoDim>::new(1, 2);
        //let indexer = Indexer::<TwoDim>::new(&space_size);
        //let buffer = vec![1, 2, 3, 4];
        //let f = df::Differential2d::new(&indexer, &buffer);
        //let p = IntPoint::<TwoDim>::new(1, 2);
        //assert_eq!(4, f.value(&p));
    }

    fn differential3d() {
        //let space_size = SpaceSize::<ThreeDim>::new(1, 2, 3);
        //let indexer = Indexer::<ThreeDim>::new(&space_size);
        //let buffer = vec![1, 2, 3];
        //let f = df::Differential::<ThreeDim, i32>::new(&indexer, &buffer);

        //let id = f.indexer;
        //let p = IntPoint::<ThreeDim>::new(1, 2, 3);
        //let q = id.get(&p);
        //assert_eq!(q, 1 + 1 * 2 + 3 * 2);

        //let bu = f.buffer;
        //assert_eq!(bu[0], 1);
        //assert_eq!(bu[1], 2);
        //assert_eq!(bu[2], 3);
    }

    fn hnd() {
        let h0d = &df::H0D;
        assert_eq!(h0d[0], 1.0);
        assert_eq!(h0d[1], 2.0);
        assert_eq!(h0d[2], 1.0);

        let h1d = &df::H1D;
        assert_eq!(h1d[0], -1.0);
        assert_eq!(h1d[1], 0.0);
        assert_eq!(h1d[2], 1.0);

        let h2d = &df::H2D;
        assert_eq!(h2d[0], 1.0);
        assert_eq!(h2d[1], -2.0);
        assert_eq!(h2d[2], 1.0);

        let h3d = &df::H3D;
        assert_eq!(h2d[0], 1.0);
        assert_eq!(h2d[1], 0.0);
        assert_eq!(h2d[2], -1.0);
    }
}
