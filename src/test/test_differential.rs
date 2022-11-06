use crate::core::differential as df;
use crate::core::types::{Indexer, IntPoint, SpaceSize, ThreeDim, TwoDim};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn differential2d_new() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3];
        let f = df::Differential2d::new(&indexer, &buffer);

        let id = f.indexer;
        let p = IntPoint::<TwoDim>::new(1, 2);
        let q = indexer.get(&p);
        assert_eq!(q, 1 + 1 * 2);

        let bu = f.buffer;
        assert_eq!(bu[0], 1);
        assert_eq!(bu[1], 2);
        assert_eq!(bu[2], 3);
    }

    #[test]
    fn differential2d_index() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3];

        let a = df::Differential2d::index(1, 2);
        assert_eq!(2 + 9, a);
    }

    #[test]
    fn differential2d_value() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3, 4];
        let f = df::Differential2d::new(&indexer, &buffer);
        let p = IntPoint::<TwoDim>::new(1, 2);
        assert_eq!(4, f.value(&p));
    }

    #[test]
    fn differential2d_set_v() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3, 4];
        let mut f = df::Differential2d::new(&indexer, &buffer);
        let p = IntPoint::<TwoDim>::new(1, 2);
        f.set_v(0, 1, 3);
        assert_eq!(f.values[7], 3);
    }

    #[test]
    fn differential2d_set_value() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3, 4, 5];
        let mut f = df::Differential2d::new(&indexer, &buffer);
        let p = IntPoint::<TwoDim>::new(1, 2);

        f.set_value(&p, 0, 1);
        assert_eq!(f.values[7], 5);
    }

    #[test]
    fn differential2d_make_point() {
        let p = IntPoint::<TwoDim>::new(1, 1);
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3, 4, 5];
        let mut f = df::Differential2d::new(&indexer, &buffer);
        f.make_point(&p);
        assert_eq!(f.values[0], 1);
        assert_eq!(f.values[1], 2);
        assert_eq!(f.values[2], 3);
        assert_eq!(f.values[3], 2);
        assert_eq!(f.values[4], 3);
        assert_eq!(f.values[5], 4);
        assert_eq!(f.values[6], 3);
        assert_eq!(f.values[7], 4);
        assert_eq!(f.values[8], 5);
    }

    #[test]
    fn differential2d_h1dx() {
        let a = df::Differential2d::h1dx(1, 1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential2d_h1dy() {
        let a = df::Differential2d::h1dy(1, 1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential2d_h2dx() {
        let a = df::Differential2d::h2dx(1, 1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential2d_h2dy() {
        let a = df::Differential2d::h2dy(1, 1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential2d_h3dxy() {
        let a = df::Differential2d::h3dxy(1, 1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential2d_v() {
        let p = IntPoint::<TwoDim>::new(1, 1);
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3, 4, 5];
        let mut f = df::Differential2d::new(&indexer, &buffer);
        f.make_point(&p);
        assert_eq!(f.values[0], 1);
        assert_eq!(f.values[1], 2);
        assert_eq!(f.values[2], 3);
        assert_eq!(f.values[3], 2);
        assert_eq!(f.values[4], 3);
        assert_eq!(f.values[5], 4);
        assert_eq!(f.values[6], 3);
        assert_eq!(f.values[7], 4);
        assert_eq!(f.values[8], 5);
        let a = f.v(1, 0);
        assert_eq!(a, 4);
    }

    #[test]
    fn differential2d_vx() {
        let p = IntPoint::<TwoDim>::new(1, 1);
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3, 4, 5];
        let mut f = df::Differential2d::new(&indexer, &buffer);
        f.make_point(&p);
        assert_eq!(f.values[0], 1);
        assert_eq!(f.values[1], 2);
        assert_eq!(f.values[2], 3);
        assert_eq!(f.values[3], 2);
        assert_eq!(f.values[4], 3);
        assert_eq!(f.values[5], 4);
        assert_eq!(f.values[6], 3);
        assert_eq!(f.values[7], 4);
        assert_eq!(f.values[8], 5);
        let a = f.vx(1, 0);
        assert_eq!(a, 8.0);
    }

    #[test]
    fn differential2d_vy() {
        let p = IntPoint::<TwoDim>::new(1, 1);
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3, 4, 5];
        let mut f = df::Differential2d::new(&indexer, &buffer);
        f.make_point(&p);
        assert_eq!(f.values[0], 1);
        assert_eq!(f.values[1], 2);
        assert_eq!(f.values[2], 3);
        assert_eq!(f.values[3], 2);
        assert_eq!(f.values[4], 3);
        assert_eq!(f.values[5], 4);
        assert_eq!(f.values[6], 3);
        assert_eq!(f.values[7], 4);
        assert_eq!(f.values[8], 5);
        let a = f.vy(1, 0);
        assert_eq!(a, 0.0);
    }

    #[test]
    fn differential2d_vxx() {
        let p = IntPoint::<TwoDim>::new(1, 1);
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3, 4, 5];
        let mut f = df::Differential2d::new(&indexer, &buffer);
        f.make_point(&p);
        assert_eq!(f.values[0], 1);
        assert_eq!(f.values[1], 2);
        assert_eq!(f.values[2], 3);
        assert_eq!(f.values[3], 2);
        assert_eq!(f.values[4], 3);
        assert_eq!(f.values[5], 4);
        assert_eq!(f.values[6], 3);
        assert_eq!(f.values[7], 4);
        assert_eq!(f.values[8], 5);
        let a = f.vxx(1, 0);
        assert_eq!(a, 8.0);
    }

    #[test]
    fn differential2d_vyy() {
        let p = IntPoint::<TwoDim>::new(1, 1);
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3, 4, 5];
        let mut f = df::Differential2d::new(&indexer, &buffer);
        f.make_point(&p);
        assert_eq!(f.values[0], 1);
        assert_eq!(f.values[1], 2);
        assert_eq!(f.values[2], 3);
        assert_eq!(f.values[3], 2);
        assert_eq!(f.values[4], 3);
        assert_eq!(f.values[5], 4);
        assert_eq!(f.values[6], 3);
        assert_eq!(f.values[7], 4);
        assert_eq!(f.values[8], 5);
        let a = f.vyy(1, 0);
        assert_eq!(a, -8.0);
    }

    #[test]
    fn differential2d_vxy() {
        let p = IntPoint::<TwoDim>::new(1, 1);
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3, 4, 5];
        let mut f = df::Differential2d::new(&indexer, &buffer);
        f.make_point(&p);
        assert_eq!(f.values[0], 1);
        assert_eq!(f.values[1], 2);
        assert_eq!(f.values[2], 3);
        assert_eq!(f.values[3], 2);
        assert_eq!(f.values[4], 3);
        assert_eq!(f.values[5], 4);
        assert_eq!(f.values[6], 3);
        assert_eq!(f.values[7], 4);
        assert_eq!(f.values[8], 5);
        let a = f.vxy(1, 0);
        assert_eq!(a, 0.0);
    }

    #[test]
    fn differential_tool_hnd() {
        let h0d = &df::DifferentialTool::H0D;
        assert_eq!(h0d[0], 1.0);
        assert_eq!(h0d[1], 2.0);
        assert_eq!(h0d[2], 1.0);

        let h1d = &df::DifferentialTool::H1D;
        assert_eq!(h1d[0], -1.0);
        assert_eq!(h1d[1], 0.0);
        assert_eq!(h1d[2], 1.0);

        let h2d = &df::DifferentialTool::H2D;
        assert_eq!(h2d[0], 1.0);
        assert_eq!(h2d[1], -2.0);
        assert_eq!(h2d[2], 1.0);

        let h3d = &df::DifferentialTool::H3D;
        assert_eq!(h3d[0], 1.0);
        assert_eq!(h3d[1], 0.0);
        assert_eq!(h3d[2], -1.0);
    }

    #[test]
    fn differential_tool_index() {
        let a = df::DifferentialTool::index(1);
        assert_eq!(a, 2);
    }

    #[test]
    fn differential_tool_h() {
        let a = df::DifferentialTool::h(1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential_tool_h1d() {
        let a = df::DifferentialTool::h1d(1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential_tool_h2d() {
        let a = df::DifferentialTool::h2d(1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential_tool_h3d() {
        let a = df::DifferentialTool::h3d(1);
        assert_eq!(a, -1.0);
    }
}
