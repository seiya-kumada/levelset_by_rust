use num_traits::ToPrimitive;
use std::ops::Add;
use std::rc::Rc;

use super::differential::Differential2d;
use super::neighboring_point::NEIGHBORING_POINTS2D;
use crate::core::differential as df;
use crate::core::grid as gr;
use crate::core::indexer as id;
use crate::core::neighboring_point as np;
use crate::core::point as po;
use crate::core::point::{Point2d, Point3d};
use crate::core::position as ps;
use crate::core::space_size as ss;
use crate::core::upwind as uw;
use crate::core::util;
use num_traits::Zero;

pub trait Type {
    type SpaceSize_; //
    type Grid_; //
    type Indexer_; //
    type IntPoint_; //
    type DoublePoint_; //
    type Position_; //
    type Upwind_; //
    type DifferentialU8_;
    const NUM: i32;

    fn make_differential_u8(
        indexer: Rc<Self::Indexer_>,
        buffer: Rc<Vec<u8>>,
    ) -> Self::DifferentialU8_;
    fn make_position(p: &Self::IntPoint_, indexer: &Self::Indexer_) -> Self::Position_; //
    fn make_upwind_with_positive_speed(p: &Self::Position_, phi: &Vec<f64>) -> Self::Upwind_; //
    fn make_upwind_with_negative_speed(p: &Self::Position_, phi: &Vec<f64>) -> Self::Upwind_; //
}

pub struct TwoDim;
pub struct ThreeDim;

impl Type for TwoDim {
    type Grid_ = gr::Grid2d;
    type SpaceSize_ = ss::SpaceSize2d;
    type Indexer_ = id::Indexer2d;
    type IntPoint_ = po::Point2d<i32>;
    type DoublePoint_ = po::Point2d<f64>;
    type Position_ = ps::Position2d;
    type Upwind_ = uw::Upwind2d;
    //type DifferentialF64 = df::DifferentialDouble2d<'a>;
    type DifferentialU8_ = df::Differential2d<u8>;
    const NUM: i32 = 2;

    fn make_differential_u8(
        indexer: Rc<Self::Indexer_>,
        buffer: Rc<Vec<u8>>,
    ) -> Self::DifferentialU8_ {
        df::Differential2d::<u8>::new(indexer, buffer)
    }

    fn make_position(p: &Self::IntPoint_, indexer: &Self::Indexer_) -> Self::Position_ {
        let a = p + np::NEIGHBORING_POINTS2D.get(-1, 0);
        let b = p + np::NEIGHBORING_POINTS2D.get(1, 0);
        let c = p + np::NEIGHBORING_POINTS2D.get(0, -1);
        let d = p + np::NEIGHBORING_POINTS2D.get(0, 1);
        ps::Position2d::new(
            indexer.get(&a),
            indexer.get(&b),
            indexer.get(p),
            indexer.get(&c),
            indexer.get(&d),
        )
    }

    fn make_upwind_with_positive_speed(p: &Self::Position_, phi: &Vec<f64>) -> Self::Upwind_ {
        let fdxm = util::max(phi[p.me as usize] - phi[p.left as usize], 0.0);
        let fdxp = util::min(phi[p.right as usize] - phi[p.me as usize], 0.0);
        let fdym = util::max(phi[p.me as usize] - phi[p.top as usize], 0.0);
        let fdyp = util::min(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
        uw::Upwind2d {
            fdxm,
            fdxp,
            fdym,
            fdyp,
        }
    }

    fn make_upwind_with_negative_speed(p: &Self::Position_, phi: &Vec<f64>) -> Self::Upwind_ {
        let fdxp = util::max(phi[p.right as usize] - phi[p.me as usize], 0.0);
        let fdxm = util::min(phi[p.me as usize] - phi[p.left as usize], 0.0);
        let fdyp = util::max(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
        let fdym = util::min(phi[p.me as usize] - phi[p.top as usize], 0.0);
        uw::Upwind2d {
            fdxm,
            fdxp,
            fdym,
            fdyp,
        }
    }
}

impl Type for ThreeDim {
    type Grid_ = gr::Grid3d;
    type SpaceSize_ = ss::SpaceSize3d;
    type Indexer_ = id::Indexer3d;
    type IntPoint_ = po::Point3d<i32>;
    type DoublePoint_ = po::Point3d<f64>;
    type Position_ = ps::Position3d;
    type Upwind_ = uw::Upwind3d;
    //type DifferentialF64 = df::DifferentialDouble3d<'a>;
    type DifferentialU8_ = df::Differential3d<u8>;
    const NUM: i32 = 3;
    fn make_differential_u8(
        indexer: Rc<Self::Indexer_>,
        buffer: Rc<Vec<u8>>,
    ) -> Self::DifferentialU8_ {
        df::Differential3d::<u8>::new(indexer, buffer)
    }
    fn make_position(p: &Self::IntPoint_, indexer: &Self::Indexer_) -> Self::Position_ {
        let a = p + np::NEIGHBORING_POINTS3D.get(-1, 0, 0);
        let b = p + np::NEIGHBORING_POINTS3D.get(1, 0, 0);
        let c = p + np::NEIGHBORING_POINTS3D.get(0, -1, 0);
        let d = p + np::NEIGHBORING_POINTS3D.get(0, 1, 0);
        let e = p + np::NEIGHBORING_POINTS3D.get(0, 0, -1);
        let f = p + np::NEIGHBORING_POINTS3D.get(0, 0, 1);
        ps::Position3d::new(
            indexer.get(&a),
            indexer.get(&b),
            indexer.get(p),
            indexer.get(&c),
            indexer.get(&d),
            indexer.get(&e),
            indexer.get(&f),
        )
    }

    fn make_upwind_with_positive_speed(p: &Self::Position_, phi: &Vec<f64>) -> Self::Upwind_ {
        let fdxm = util::max(phi[p.me as usize] - phi[p.left as usize], 0.0);
        let fdxp = util::min(phi[p.right as usize] - phi[p.me as usize], 0.0);
        let fdym = util::max(phi[p.me as usize] - phi[p.top as usize], 0.0);
        let fdyp = util::min(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
        let fdzm = util::max(phi[p.me as usize] - phi[p.front as usize], 0.0);
        let fdzp = util::min(phi[p.back as usize] - phi[p.me as usize], 0.0);
        uw::Upwind3d {
            fdxm,
            fdxp,
            fdym,
            fdyp,
            fdzm,
            fdzp,
        }
    }

    fn make_upwind_with_negative_speed(p: &Self::Position_, phi: &Vec<f64>) -> Self::Upwind_ {
        let fdxm = util::min(phi[p.me as usize] - phi[p.left as usize], 0.0);
        let fdxp = util::max(phi[p.right as usize] - phi[p.me as usize], 0.0);
        let fdym = util::min(phi[p.me as usize] - phi[p.top as usize], 0.0);
        let fdyp = util::max(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
        let fdzm = util::min(phi[p.me as usize] - phi[p.front as usize], 0.0);
        let fdzp = util::max(phi[p.back as usize] - phi[p.me as usize], 0.0);
        uw::Upwind3d {
            fdxm,
            fdxp,
            fdym,
            fdyp,
            fdzm,
            fdzp,
        }
    }
}

//impl<'a> TypeRef<'a> for ThreeDim {
//    type DifferentialF64 = df::Differential3d<'a, f64>;
//    type DifferentialU8 = df::Differential3d<'a, u8>;
//    //type Ind = Indexer<ThreeDim>;
//    //fn new(indexer: &Self::Ind, gray: &Vec<u8>) {}
//}
pub type Grid<D> = <D as Type>::Grid_;
pub type SpaceSize<D> = <D as Type>::SpaceSize_;
pub type Indexer<D> = <D as Type>::Indexer_;
pub type IntPoint<D> = <D as Type>::IntPoint_;
pub type DoublePoint<D> = <D as Type>::DoublePoint_;
pub type Position<D> = <D as Type>::Position_;
pub type Upwind<D> = <D as Type>::Upwind_;
//pub type DifferentialF64<'a, D> = <D as TypeRef<'a>>::DifferentialF64;
pub type DifferentialU8<D> = <D as Type>::DifferentialU8_;
