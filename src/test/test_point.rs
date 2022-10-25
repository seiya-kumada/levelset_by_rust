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

        let p = IntPoint::<TwoDim>::new(1, 2);
        assert_eq!(1, p.x);
        assert_eq!(2, p.y);
    }
}
