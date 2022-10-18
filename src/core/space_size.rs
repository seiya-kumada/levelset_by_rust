pub struct SpaceSize2d {
    width: i32,
    height: i32,
    total: i32,
}

impl SpaceSize2d {
    fn new(w: i32, h: i32) -> Self {
        SpaceSize2d {
            width: w,
            height: h,
            total: w * h,
        }
    }
}

pub struct SpaceSize3d {
    width: i32,
    height: i32,
    depth: i32,
    total: i32,
}

impl SpaceSize3d {
    fn new(w: i32, h: i32, d: i32) -> Self {
        SpaceSize3d {
            width: w,
            height: h,
            depth: d,
            total: w * h * d,
        }
    }
}

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
