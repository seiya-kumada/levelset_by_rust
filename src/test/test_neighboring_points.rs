use crate::core::neighboring_point as np;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighboring_points_2d() {
        let a = np::NEIGHBORING_POINTS2D.get(0, 0);
        assert_eq!(a.x, 0);
        assert_eq!(a.y, 0);

        let a = np::NEIGHBORING_POINTS2D.get(0, 1);
        assert_eq!(a.x, 0);
        assert_eq!(a.y, 1);
    }

    #[test]
    fn neighboring_points_3d() {
        let a = np::NEIGHBORING_POINTS3D.get(0, 0, 0);
        assert_eq!(a.x, 0);
        assert_eq!(a.y, 0);
        assert_eq!(a.z, 0);

        let a = np::NEIGHBORING_POINTS3D.get(1, 0, 0);
        assert_eq!(a.x, 1);
        assert_eq!(a.y, 0);
        assert_eq!(a.z, 0);
    }
}
