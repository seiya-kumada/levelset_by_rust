use crate::core::differential as df;
use crate::core::differential::DifferentialMethod;
use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::types::{Indexer, IntPoint, SpaceSize, ThreeDim, TwoDim};
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn differential_tool_h0d_total() {
        assert_eq!(df::DifferentialTool::H0D_TOTAL, 4);
    }

    #[test]
    fn differential2d_new() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3]);
        let f = df::Differential2d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));

        let id = f.indexer;
        let p = IntPoint::<TwoDim>::new(1, 2);
        let q = indexer.get(&p);
        assert_eq!(q, 1 + 1 * 2);

        let bu = f.buffer;
        assert_eq!(bu.borrow()[0], 1);
        assert_eq!(bu.borrow()[1], 2);
        assert_eq!(bu.borrow()[2], 3);
    }

    #[test]
    fn differential3d_new() {
        let space_size = SpaceSize::<ThreeDim>::new(1, 2, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3]);
        let f = df::Differential3d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));

        let id = f.indexer;
        let p = IntPoint::<ThreeDim>::new(1, 2, 3);
        let q = indexer.get(&p);
        assert_eq!(q, 1 + 1 * 2 + 2 * (3));

        let bu = f.buffer;
        assert_eq!(bu.borrow()[0], 1);
        assert_eq!(bu.borrow()[1], 2);
        assert_eq!(bu.borrow()[2], 3);
    }

    #[test]
    fn differential2d_index() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Indexer::<TwoDim>::new(&space_size);
        let buffer = vec![1, 2, 3];

        let a = df::Differential2d::<i32>::index(1, 2);
        assert_eq!(2 + 9, a);
    }

    #[test]
    fn differential2d_value() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3, 4]);
        let f = df::Differential2d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));
        let p = IntPoint::<TwoDim>::new(1, 2);
        assert_eq!(4, f.value(&p));
        assert_eq!(f.values.len(), 9);
    }

    #[test]
    fn differential2d_set_v() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3, 4]);
        let mut f = df::Differential2d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));
        let p = IntPoint::<TwoDim>::new(1, 2);
        f.set_v(0, 1, 3);
        assert_eq!(f.values[7], 3);
        assert_eq!(f.values.len(), 9);
    }

    #[test]
    fn differential2d_set_value() {
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3, 4, 5]);
        let mut f = df::Differential2d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));
        let p = IntPoint::<TwoDim>::new(1, 2);

        f.set_value(&p, 0, 1);
        assert_eq!(f.values[7], 5);
    }

    #[test]
    fn differential2d_make_point() {
        let p = IntPoint::<TwoDim>::new(1, 1);
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3, 4, 5]);
        let mut f = df::Differential2d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));
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
        let a = df::Differential2d::<i32>::h1dx(1, 1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential2d_h1dy() {
        let a = df::Differential2d::<i32>::h1dy(1, 1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential2d_h2dx() {
        assert_eq!(1.0, df::Differential2d::<i32>::h2dx(-1, -1));
        assert_eq!(-2.0, df::Differential2d::<i32>::h2dx(0, -1));
        assert_eq!(1.0, df::Differential2d::<i32>::h2dx(1, -1));
        assert_eq!(2.0, df::Differential2d::<i32>::h2dx(-1, 0));
        assert_eq!(-4.0, df::Differential2d::<i32>::h2dx(0, 0));
        assert_eq!(2.0, df::Differential2d::<i32>::h2dx(1, 0));
        assert_eq!(1.0, df::Differential2d::<i32>::h2dx(-1, 1));
        assert_eq!(-2.0, df::Differential2d::<i32>::h2dx(0, 1));
        assert_eq!(1.0, df::Differential2d::<i32>::h2dx(1, 1));
    }

    #[test]
    fn differential2d_h2dy() {
        let a = df::Differential2d::<i32>::h2dy(1, 1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential2d_h3dxy() {
        let a = df::Differential2d::<i32>::h3dxy(1, 1);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn differential2d_v() {
        let p = IntPoint::<TwoDim>::new(1, 1);
        let space_size = SpaceSize::<TwoDim>::new(1, 2);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3, 4, 5]);
        let mut f = df::Differential2d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));
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
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3, 4, 5]);
        let mut f = df::Differential2d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));
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
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3, 4, 5]);
        let mut f = df::Differential2d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));
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
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3, 4, 5]);
        let mut f = df::Differential2d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));
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
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3, 4, 5]);
        let mut f = df::Differential2d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));
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
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let buffer = RefCell::new(vec![1, 2, 3, 4, 5]);
        let mut f = df::Differential2d::<i32>::new(Rc::clone(&indexer), RefCell::clone(&buffer));
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

    fn sobel_x_2d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<TwoDim>::new(3, 3);
        let indexer = Rc::new(Indexer2d::new(&space_size));
        let mut cg = df::DifferentialDouble2d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<TwoDim>::new(1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.sobel_x());
    }

    #[test]
    fn sobel_x_2d() {
        let v = RefCell::new(vec![0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0]);
        let e = 8.0;
        sobel_x_2d_core(RefCell::clone(&v), e);
    }

    fn fx_2d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<TwoDim>::new(3, 3);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let mut cg = df::DifferentialDouble2d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<TwoDim>::new(1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fx());
    }

    #[test]
    fn fx_2d() {
        let v = RefCell::new(vec![0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0]);
        let e = 1.0;
        fx_2d_core(RefCell::clone(&v), e);
    }

    fn sobel_y_2d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<TwoDim>::new(3, 3);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let mut cg = df::DifferentialDouble2d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<TwoDim>::new(1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.sobel_y());
    }

    #[test]
    fn sobel_y_2d() {
        let v = RefCell::new(vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0]);
        let e = 8.0;
        sobel_y_2d_core(RefCell::clone(&v), e);
    }

    fn fy_2d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<TwoDim>::new(3, 3);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let mut cg = df::DifferentialDouble2d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<TwoDim>::new(1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fy());
    }

    #[test]
    fn fy_2d() {
        let v = RefCell::new(vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0]);
        let e = 1.0;
        fy_2d_core(RefCell::clone(&v), e);
    }

    fn fxy_2d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<TwoDim>::new(3, 3);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let mut cg = df::DifferentialDouble2d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<TwoDim>::new(1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fxy());
    }

    #[test]
    fn fxy_2d() {
        let v = RefCell::new(vec![0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 3.0]);
        let e = 0.25;
        fxy_2d_core(RefCell::clone(&v), e);
    }

    fn fxx_2d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<TwoDim>::new(3, 3);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let mut cg = df::DifferentialDouble2d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<TwoDim>::new(1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fxx());
    }

    #[test]
    fn fxx_2d() {
        let v = RefCell::new(vec![1.0, 0.0, 3.0, 4.0, 0.0, 6.0, 7.0, 0.0, 9.0]);
        let e = 10.0;
        fxx_2d_core(RefCell::clone(&v), e);
    }

    fn fyy_2d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<TwoDim>::new(3, 3);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let mut cg = df::DifferentialDouble2d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<TwoDim>::new(1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fyy());
    }

    #[test]
    fn fyy_2d() {
        let v = RefCell::new(vec![1.0, 0.0, 3.0, 2.0, 2.0, 2.0, 1.0, 0.0, 3.0]);
        let e = -2.0;
        fyy_2d_core(RefCell::clone(&v), e);
    }

    fn fx_fy_with_u8_2d_core(input: RefCell<Vec<u8>>, expected_fx: f64, expected_fy: f64) {
        let space_size = SpaceSize::<TwoDim>::new(3, 3);
        let indexer = Rc::new(Indexer::<TwoDim>::new(&space_size));
        let mut cg = df::Differential2d::<u8>::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<TwoDim>::new(1, 1);
        cg.make_point(&p);
        assert_eq!(expected_fx, cg.fx());
        assert_eq!(expected_fy, cg.fy());
    }

    #[test]
    fn fx_fy_with_u8_2d() {
        let v = RefCell::new(vec![50, 100, 20, 100, 0, 200, 70, 100, 30]);
        let fx_e = 65.0 / 4.0;
        let fy_e = 15.0 / 4.0;
        fx_fy_with_u8_2d_core(RefCell::clone(&v), fx_e, fy_e);
    }

    #[test]
    fn differential3d_h1dx() {
        assert_eq!(-1.0, df::Differential3d::<i32>::h1dx(-1, -1, -1));
        assert_eq!(0.0, df::Differential3d::<i32>::h1dx(0, -1, -1));
        assert_eq!(1.0, df::Differential3d::<i32>::h1dx(1, -1, -1));
        assert_eq!(-2.0, df::Differential3d::<i32>::h1dx(-1, 0, -1));
        assert_eq!(0.0, df::Differential3d::<i32>::h1dx(0, 0, -1));
        assert_eq!(2.0, df::Differential3d::<i32>::h1dx(1, 0, -1));
        assert_eq!(-1.0, df::Differential3d::<i32>::h1dx(-1, 1, -1));
        assert_eq!(0.0, df::Differential3d::<i32>::h1dx(0, 1, -1));
        assert_eq!(1.0, df::Differential3d::<i32>::h1dx(1, 1, -1));

        assert_eq!(-2.0, df::Differential3d::<i32>::h1dx(-1, -1, 0));
        assert_eq!(0.0, df::Differential3d::<i32>::h1dx(0, -1, 0));
        assert_eq!(2.0, df::Differential3d::<i32>::h1dx(1, -1, 0));
        assert_eq!(-4.0, df::Differential3d::<i32>::h1dx(-1, 0, 0));
        assert_eq!(0.0, df::Differential3d::<i32>::h1dx(0, 0, 0));
        assert_eq!(4.0, df::Differential3d::<i32>::h1dx(1, 0, 0));
        assert_eq!(-2.0, df::Differential3d::<i32>::h1dx(-1, 1, 0));
        assert_eq!(0.0, df::Differential3d::<i32>::h1dx(0, 1, 0));
        assert_eq!(2.0, df::Differential3d::<i32>::h1dx(1, 1, 0));

        assert_eq!(-1.0, df::Differential3d::<i32>::h1dx(-1, -1, 1));
        assert_eq!(0.0, df::Differential3d::<i32>::h1dx(0, -1, 1));
        assert_eq!(1.0, df::Differential3d::<i32>::h1dx(1, -1, 1));
        assert_eq!(-2.0, df::Differential3d::<i32>::h1dx(-1, 0, 1));
        assert_eq!(0.0, df::Differential3d::<i32>::h1dx(0, 0, 1));
        assert_eq!(2.0, df::Differential3d::<i32>::h1dx(1, 0, 1));
        assert_eq!(-1.0, df::Differential3d::<i32>::h1dx(-1, 1, 1));
        assert_eq!(0.0, df::Differential3d::<i32>::h1dx(0, 1, 1));
        assert_eq!(1.0, df::Differential3d::<i32>::h1dx(1, 1, 1));
    }

    #[test]
    fn differential3d_h2dx() {
        assert_eq!(1.0, df::Differential3d::<i32>::h2dx(-1, -1, -1));
        assert_eq!(-2.0, df::Differential3d::<i32>::h2dx(0, -1, -1));
        assert_eq!(1.0, df::Differential3d::<i32>::h2dx(1, -1, -1));
        assert_eq!(2.0, df::Differential3d::<i32>::h2dx(-1, 0, -1));
        assert_eq!(-4.0, df::Differential3d::<i32>::h2dx(0, 0, -1));
        assert_eq!(2.0, df::Differential3d::<i32>::h2dx(1, 0, -1));
        assert_eq!(1.0, df::Differential3d::<i32>::h2dx(-1, 1, -1));
        assert_eq!(-2.0, df::Differential3d::<i32>::h2dx(0, 1, -1));
        assert_eq!(1.0, df::Differential3d::<i32>::h2dx(1, 1, -1));

        assert_eq!(2.0, df::Differential3d::<i32>::h2dx(-1, -1, 0));
        assert_eq!(-4.0, df::Differential3d::<i32>::h2dx(0, -1, 0));
        assert_eq!(2.0, df::Differential3d::<i32>::h2dx(1, -1, 0));
        assert_eq!(4.0, df::Differential3d::<i32>::h2dx(-1, 0, 0));
        assert_eq!(-8.0, df::Differential3d::<i32>::h2dx(0, 0, 0));
        assert_eq!(4.0, df::Differential3d::<i32>::h2dx(1, 0, 0));
        assert_eq!(2.0, df::Differential3d::<i32>::h2dx(-1, 1, 0));
        assert_eq!(-4.0, df::Differential3d::<i32>::h2dx(0, 1, 0));
        assert_eq!(2.0, df::Differential3d::<i32>::h2dx(1, 1, 0));
    }

    fn sobel_x_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.sobel_x());
    }

    #[test]
    fn sobel_x_3d() {
        let v = RefCell::new(vec![
            -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0,
            1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ]);
        let e = 32.0;
        sobel_x_3d_core(RefCell::clone(&v), e);
    }

    fn sobel_y_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.sobel_y());
    }

    #[test]
    fn sobel_y_3d() {
        let v = RefCell::new(vec![
            -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ]);
        let e = 32.0;
        sobel_y_3d_core(RefCell::clone(&v), e);
    }

    fn sobel_z_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.sobel_z());
    }

    #[test]
    fn sobel_z_3d() {
        let v = RefCell::new(vec![
            -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ]);
        let e = 32.0;
        sobel_z_3d_core(RefCell::clone(&v), e);
    }

    fn fx_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fx());
    }

    #[test]
    fn fx_3d() {
        let v = RefCell::new(vec![
            0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
        ]);
        let e = 0.5;
        fx_3d_core(RefCell::clone(&v), e);
    }

    fn fy_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fy());
    }

    #[test]
    fn fy_3d() {
        let v = RefCell::new(vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
        ]);
        let e = 0.5;
        fy_3d_core(RefCell::clone(&v), e);
    }

    fn fz_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fz());
    }

    #[test]
    fn fz_3d() {
        let v = RefCell::new(vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ]);
        let e = 0.5;
        fz_3d_core(RefCell::clone(&v), e);
    }

    fn fxy_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fxy());
    }

    #[test]
    fn fxy_3d() {
        let v = RefCell::new(vec![
            5.0, 0.0, 7.0, 0.0, 0.0, 0.0, 4.0, 0.0, 3.0, 6.0, 0.0, 8.0, 0.0, 0.0, 0.0, 5.0, 0.0,
            4.0, 7.0, 0.0, 9.0, 0.0, 0.0, 0.0, 6.0, 0.0, 5.0,
        ]);
        let e = -12.0 / 16.0;
        fxy_3d_core(RefCell::clone(&v), e);
    }

    fn fxz_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fxz());
    }

    #[test]
    fn fxz_3d() {
        let v = RefCell::new(vec![
            5.0, 0.0, 7.0, 6.0, 0.0, 8.0, 7.0, 0.0, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 4.0, 0.0, 3.0, 5.0, 0.0, 4.0, 6.0, 0.0, 5.0,
        ]);
        let e = -12.0 / 16.0;
        fxz_3d_core(RefCell::clone(&v), e);
    }

    fn fyz_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fyz());
    }

    #[test]
    fn fyz_3d() {
        let v = RefCell::new(vec![
            5.0, 6.0, 7.0, 0.0, 0.0, 0.0, 4.0, 5.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 7.0, 8.0, 9.0, 0.0, 0.0, 0.0, 3.0, 4.0, 5.0,
        ]);
        let e = -12.0 / 16.0;
        fyz_3d_core(RefCell::clone(&v), e);
    }

    fn fxx_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fxx());
    }

    #[test]
    fn fxx_3d() {
        let v = RefCell::new(vec![
            1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
            -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0,
        ]);
        let e = 4.0;
        fxx_3d_core(RefCell::clone(&v), e);
    }

    fn fyy_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fyy());
    }

    #[test]
    fn fyy_3d() {
        let v = RefCell::new(vec![
            1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0,
        ]);
        let e = 4.0;
        fyy_3d_core(RefCell::clone(&v), e);
    }

    fn fzz_3d_core(input: RefCell<Vec<f64>>, expected_output: f64) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::DifferentialDouble3d::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_output, cg.fzz());
    }

    #[test]
    fn fzz_3d() {
        let v = RefCell::new(vec![
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ]);
        let e = 4.0;
        fzz_3d_core(RefCell::clone(&v), e);
    }

    fn fx_fy_fz_3d_with_u8_core(
        input: RefCell<Vec<u8>>,
        expected_fx: f64,
        expected_fy: f64,
        expected_fz: f64,
    ) {
        let space_size = SpaceSize::<ThreeDim>::new(3, 3, 3);
        let indexer = Rc::new(Indexer::<ThreeDim>::new(&space_size));
        let mut cg = df::Differential3d::<u8>::new(Rc::clone(&indexer), RefCell::clone(&input));
        let p = IntPoint::<ThreeDim>::new(1, 1, 1);
        cg.make_point(&p);
        assert_eq!(expected_fx, cg.fx());
        assert_eq!(expected_fy, cg.fy());
        assert_eq!(expected_fz, cg.fz());
    }

    #[test]
    fn fx_fy_fz_3d_with_u8() {
        let v = RefCell::new(vec![
            50, 100, 20, 100, 0, 200, 70, 100, 30, 50, 100, 20, 100, 0, 200, 70, 100, 30, 50, 100,
            20, 100, 0, 200, 70, 100, 30,
        ]);
        let fx_e = 65.0 / 4.0;
        let fy_e = 15.0 / 4.0;
        let fz_e = 0.0;
        fx_fy_fz_3d_with_u8_core(RefCell::clone(&v), fx_e, fy_e, fz_e);
    }

    #[test]
    fn hoge() {
        trait Person {
            fn name(&self) -> String;
        }

        trait Student: Person {
            fn university(&self) -> String;
        }

        trait Programmer {
            fn fav_language(&self) -> String;
        }

        trait CompSciStudent: Programmer + Student {
            fn git_username(&self) -> String;
        }

        struct Kumada;
        impl CompSciStudent for Kumada {
            fn git_username(&self) -> String {
                String::from("Kumada")
            }
        }

        impl Programmer for Kumada {
            fn fav_language(&self) -> String {
                String::from("Kumada")
            }
        }

        impl Student for Kumada {
            fn university(&self) -> String {
                String::from("Osaka")
            }
        }

        impl Person for Kumada {
            fn name(&self) -> String {
                String::from("Kumada")
            }
        }

        let kumada = Kumada {};
        println!(
            "{},{},{},{}",
            kumada.name(),
            kumada.university(),
            kumada.fav_language(),
            kumada.git_username()
        );

        fn hoge(kumada: &dyn CompSciStudent) {
            println!(
                "{},{},{},{}",
                kumada.name(),
                kumada.university(),
                kumada.fav_language(),
                kumada.git_username()
            );
        }
        fn foo(kumada: &Kumada) {
            println!(
                "{},{},{},{}",
                kumada.name(),
                kumada.university(),
                kumada.fav_language(),
                kumada.git_username()
            );
        }
        fn bar<K: CompSciStudent>(kumada: &K) {
            println!(
                "{},{},{},{}",
                kumada.name(),
                kumada.university(),
                kumada.fav_language(),
                kumada.git_username()
            );
        }

        trait Foo {
            fn method(&self) -> String;
        }

        fn do_something(x: &dyn Foo) {
            x.method();
        }

        fn parse_csv_document<R: std::io::BufRead>(src: R) {}
        fn parse_csv_document_(src: impl std::io::BufRead) {}
    }
}
