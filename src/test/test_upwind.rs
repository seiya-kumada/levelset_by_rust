use crate::core::upwind as up;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn upwind() {
        let u = up::Upwind2d {
            fdxm: 1.0,
            fdxp: 2.0,
            fdym: 3.0,
            fdyp: 4.0,
        };
        assert_eq!(u.fdxm, 1.0);
        assert_eq!(u.fdxp, 2.0);
        assert_eq!(u.fdym, 3.0);
        assert_eq!(u.fdyp, 4.0);

        let u = up::Upwind3d {
            fdxm: 1.0,
            fdxp: 2.0,
            fdym: 3.0,
            fdyp: 4.0,
            fdzm: 5.0,
            fdzp: 6.0,
        };
        assert_eq!(u.fdxm, 1.0);
        assert_eq!(u.fdxp, 2.0);
        assert_eq!(u.fdym, 3.0);
        assert_eq!(u.fdyp, 4.0);
        assert_eq!(u.fdzm, 5.0);
        assert_eq!(u.fdzp, 6.0);
    }
}
