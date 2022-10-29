use std::ops::Add;

use crate::core::grid as gr;
use crate::core::indexer as id;
use crate::core::neighboring_point as np;
use crate::core::point as po;
use crate::core::point::{Point2d, Point3d};
use crate::core::position as ps;
use crate::core::space_size as ss;

use super::neighboring_point::NEIGHBORING_POINTS2D;
pub trait Type {
    type SpaceSize_;
    type Grid_;
    type Indexer_;
    type IntPoint_;
    type DoublePoint_;
    type Position_;

    fn make_position(p: &Self::IntPoint_, indexer: &Self::Indexer_) -> Self::Position_;
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
}

impl Type for ThreeDim {
    type Grid_ = gr::Grid3d;
    type SpaceSize_ = ss::SpaceSize3d;
    type Indexer_ = id::Indexer3d;
    type IntPoint_ = po::Point3d<i32>;
    type DoublePoint_ = po::Point3d<f64>;
    type Position_ = ps::Position3d;

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
}

pub type Grid<D> = <D as Type>::Grid_;
pub type SpaceSize<D> = <D as Type>::SpaceSize_;
pub type Indexer<D> = <D as Type>::Indexer_;
pub type IntPoint<D> = <D as Type>::IntPoint_;
pub type DoublePoint<D> = <D as Type>::DoublePoint_;
pub type Position<D> = <D as Type>::Position_;
