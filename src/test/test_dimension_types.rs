use crate::core::dimension_types as dim;

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_two_dimension() {
        assert_eq!(2, dim::TWO);
    }

    fn test_three_dimension() {
        assert_eq!(3, dim::THREE);
    }
}
