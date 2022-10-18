pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

pub struct Point3d<T> {
    x: T,
    y: T,
    z: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_2d() {
        let a = Point2d { x: 1, y: 2 };
        assert_eq!(a.x, 1);
        assert_eq!(a.y, 2);

        let a = Point2d { x: 1.0, y: 2.0 };
        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 2.0);
    }

    #[test]
    fn point_3d() {
        let a = Point3d { x: 1, y: 2, z: 3 };
        assert_eq!(a.x, 1);
        assert_eq!(a.y, 2);
        assert_eq!(a.z, 3);

        let a = Point3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 2.0);
        assert_eq!(a.z, 3.0);
    }
}
