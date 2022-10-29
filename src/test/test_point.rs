use crate::core::types::{IntPoint, ThreeDim, TwoDim};
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point() {
        let p = IntPoint::<ThreeDim>::new(1, 2, 3);
        assert_eq!(1, p.x);
        assert_eq!(2, p.y);
        assert_eq!(3, p.z);
        let q = IntPoint::<ThreeDim>::new(3, 2, 1);
        let r = &p + &q;
        assert_eq!(4, r.x);
        assert_eq!(4, r.y);
        assert_eq!(4, r.z);

        let p = IntPoint::<TwoDim>::new(1, 2);
        assert_eq!(1, p.x);
        assert_eq!(2, p.y);
        let q = IntPoint::<TwoDim>::new(2, 1);
        let r = &p + &q;
        assert_eq!(3, r.x);
        assert_eq!(3, r.y);
    }
}
