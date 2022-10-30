use crate::core::position as po;

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn position_2d() {
        let a = po::Position2d::new(1, 2, 3, 4, 5);
        assert_eq!(1, a.left);
        assert_eq!(2, a.right);
        assert_eq!(3, a.me);
        assert_eq!(4, a.top);
        assert_eq!(5, a.bottom);
    }

    fn position_3d() {
        let a = po::Position3d::new(1, 2, 3, 4, 5, 6, 7);
        assert_eq!(1, a.left);
        assert_eq!(2, a.right);
        assert_eq!(3, a.me);
        assert_eq!(4, a.top);
        assert_eq!(5, a.bottom);
        assert_eq!(6, a.front);
        assert_eq!(7, a.back);
    }
}
