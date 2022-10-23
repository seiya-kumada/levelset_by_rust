use crate::core::dimension_types::{Dimension, ThreeDimension, TwoDimension};

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_two_dimension() {
        let td = TwoDimension::new(1, 2);
        assert_eq!(TwoDimension::dim(), 2);
        assert_eq!(td.width, 1);
        assert_eq!(td.height, 2);
        assert_eq!(td.total, 2);
    }

    fn test_three_dimension() {
        let td = ThreeDimension::new(1, 2, 3);
        assert_eq!(ThreeDimension::dim(), 3);
        assert_eq!(td.width, 1);
        assert_eq!(td.height, 2);
        assert_eq!(td.depth, 3);
        assert_eq!(td.total, 6);
    }
}
