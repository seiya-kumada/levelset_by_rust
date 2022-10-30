use crate::core::front::{Front, Front2d, Front3d};
use crate::core::types::{IntPoint, ThreeDim, TwoDim};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn front() {
        let a = IntPoint::<TwoDim>::new(1, 2);
        let b = IntPoint::<TwoDim>::new(1, 2);
        let c: Front<TwoDim> = vec![a, b];
        let d: Front2d = vec![a, b];
        assert_eq!(d[0].x, 1);
        assert_eq!(d[0].y, 2);
        assert_eq!(d[1].x, 1);
        assert_eq!(d[1].y, 2);

        let a = IntPoint::<ThreeDim>::new(1, 2, 3);
        let b = IntPoint::<ThreeDim>::new(1, 2, 3);
        let c: Front3d = vec![a, b];
        assert_eq!(c[0].x, 1);
        assert_eq!(c[0].y, 2);
        assert_eq!(c[0].z, 3);
        assert_eq!(c[1].x, 1);
        assert_eq!(c[1].y, 2);
        assert_eq!(c[1].z, 3);
    }
}
