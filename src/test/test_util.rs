use crate::core::util as ut;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max() {
        assert_eq!(3.0, ut::max(1.0, 3.0));
    }

    #[test]
    fn min() {
        assert_eq!(1.0, ut::min(1.0, 3.0));
    }
}
