use crate::core::speed as sp;
use crate::core::types;
use crate::core::upwind_scheme as us;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn position_2d() {
        let p = types::IntPoint::<types::TwoDim>::new(1, 2);
        let space_size = types::SpaceSize::<types::TwoDim>::new(1, 2);
        let indexer = types::Indexer::<types::TwoDim>::new(&space_size);
        let phi = vec![1.0; 100];
        let scheme =
            us::UpwindScheme::<types::TwoDim>::new(&p, &indexer, &phi, &sp::Speed::Positive);
        let r = scheme.position;
        assert_eq!(r.left, 2);
        assert_eq!(r.right, 4);
        assert_eq!(r.me, 3);
        assert_eq!(r.top, 2);
        assert_eq!(r.bottom, 4);
    }

    #[test]
    fn position_3d() {
        let p = types::IntPoint::<types::ThreeDim>::new(1, 1, 1);
        let space_size = types::SpaceSize::<types::ThreeDim>::new(1, 1, 1);
        let indexer = types::Indexer::<types::ThreeDim>::new(&space_size);
        let phi = vec![1.0; 100];
        let scheme =
            us::UpwindScheme::<types::ThreeDim>::new(&p, &indexer, &phi, &sp::Speed::Positive);
        let r = scheme.position;
        assert_eq!(r.left, 2);
        assert_eq!(r.right, 4);
        assert_eq!(r.me, 3);
        assert_eq!(r.top, 2);
        assert_eq!(r.bottom, 4);
        assert_eq!(r.front, 2);
        assert_eq!(r.back, 4);
    }
}
