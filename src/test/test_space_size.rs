use crate::core::space_size::SpaceSize2d;
use crate::core::space_size::SpaceSize3d;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_size_2d_new() {
        let a = SpaceSize2d::new(1, 2);
        assert_eq!(a.width, 1);
        assert_eq!(a.height, 2);
        assert_eq!(a.total, 2);
    }

    #[test]
    fn space_size_3d_new() {
        let a = SpaceSize3d::new(1, 2, 3);
        assert_eq!(a.width, 1);
        assert_eq!(a.height, 2);
        assert_eq!(a.depth, 3);
        assert_eq!(a.total, 6);
    }
}
