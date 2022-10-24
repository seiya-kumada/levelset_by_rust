use crate::core::space_size::{DataType, Dim, SpaceSize, SpaceSize2d, SpaceSize3d};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_size_2d_new() {
        let b = <Dim<2> as DataType>::Data::new(1, 2);
        let a = SpaceSize::<Dim<2>>(b);
        assert_eq!(a.0.width, 1);
        assert_eq!(a.0.height, 2);
        assert_eq!(a.0.total, 2);
    }

    #[test]
    fn space_size_3d_new() {
        let b = <Dim<3> as DataType>::Data::new(1, 2, 3);
        let a = SpaceSize::<Dim<3>>(b);
        assert_eq!(a.0.width, 1);
        assert_eq!(a.0.height, 2);
        assert_eq!(a.0.depth, 3);
        assert_eq!(a.0.total, 6);
    }
}
