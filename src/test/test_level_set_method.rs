use crate::core::level_set_method as ls;
use crate::core::space_size as ss;
use crate::core::types::{Grid, Indexer, SpaceSize, ThreeDim, TwoDim};
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn level_set_method() {
        let g2 = Grid::<TwoDim>::new(1, 2, 3, 4);
        let g3 = Grid::<ThreeDim>::new(1, 2, 3, 4, 5, 6);

        let s2 = SpaceSize::<TwoDim>::new(1, 2);
        let s3 = SpaceSize::<ThreeDim>::new(1, 2, 3);

        let i2 = Indexer::<TwoDim>::new(&s2);
        let i3 = Indexer::<ThreeDim>::new(&s3);

        let a = ls::LevelSetMethod::<TwoDim> {
            size: s2,
            //initial_front: g2,
            //indexer: i2,
        };

        let a = ls::LevelSetMethod::<ThreeDim> {
            size: s3,
            //initial_front: g3,
            //indexer: i3,
        };
    }
}
