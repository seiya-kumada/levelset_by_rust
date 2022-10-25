use crate::core::types::{Grid, ThreeDim, TwoDim};
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_2d_new() {
        let a = Grid::<TwoDim>::new(1, 2, 3, 4);
        assert_eq!(a.left, 1);
        assert_eq!(a.top, 2);
        assert_eq!(a.right, 3);
        assert_eq!(a.bottom, 4);
    }

    #[test]
    fn grid_3d_new() {
        let a = Grid::<ThreeDim>::new(1, 2, 3, 4, 5, 6);
        assert_eq!(a.left, 1);
        assert_eq!(a.top, 2);
        assert_eq!(a.right, 3);
        assert_eq!(a.bottom, 4);
        assert_eq!(a.front, 5);
        assert_eq!(a.back, 6);
    }
}
