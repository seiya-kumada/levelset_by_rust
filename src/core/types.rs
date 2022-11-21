use super::differential::Differential2d;
use super::neighboring_point::NEIGHBORING_POINTS2D;
use crate::core::differential as df;
use crate::core::grid as gr;
use crate::core::indexer as id;
use crate::core::initial_front::{InitialFront2d, InitialFront3d};
use crate::core::neighboring_point as np;
use crate::core::point as po;
use crate::core::point::{Point2d, Point3d};
use crate::core::position as ps;
use crate::core::space_size as ss;
use crate::core::speed_factor as sf;
use crate::core::upwind as uw;
use crate::core::util;
use num_traits::ToPrimitive;
use num_traits::Zero;
use std::ops::Add;
use std::rc::Rc;

pub trait Type {
    type SpaceSize_; //
    type Grid_; //
    type Indexer_; //
    type IntPoint_; //
    type DoublePoint_; //
    type Position_; //
    type Upwind_; //
    type DifferentialU8_;
    type SpeedFactor_;
    type InitialFront_;
    const NUM: i32;
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
    type DifferentialU8_ = df::Differential2d<u8>;
    type SpeedFactor_ = sf::SpeedFactor2d;
    type InitialFront_ = InitialFront2d;

    const NUM: i32 = 2;
}

impl Type for ThreeDim {
    type Grid_ = gr::Grid3d;
    type SpaceSize_ = ss::SpaceSize3d;
    type Indexer_ = id::Indexer3d;
    type IntPoint_ = po::Point3d<i32>;
    type DoublePoint_ = po::Point3d<f64>;
    type Position_ = ps::Position3d;
    type Upwind_ = uw::Upwind3d;
    type SpeedFactor_ = sf::SpeedFactor3d;
    type DifferentialU8_ = df::Differential3d<u8>;
    type InitialFront_ = InitialFront3d;
    const NUM: i32 = 3;
}

pub type Grid<D> = <D as Type>::Grid_;
pub type SpaceSize<D> = <D as Type>::SpaceSize_;
pub type Indexer<D> = <D as Type>::Indexer_;
pub type IntPoint<D> = <D as Type>::IntPoint_;
pub type InitialFront<D> = <D as Type>::InitialFront_;
pub type DoublePoint<D> = <D as Type>::DoublePoint_;
pub type Position<D> = <D as Type>::Position_;
pub type Upwind<D> = <D as Type>::Upwind_;
pub type DifferentialU8<D> = <D as Type>::DifferentialU8_;
pub type SpeedFactor<D> = <D as Type>::SpeedFactor_;
