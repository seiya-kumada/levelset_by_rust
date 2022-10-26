use crate::core::grid as gr;
use crate::core::indexer as id;
use crate::core::point as po;
use crate::core::position as ps;
use crate::core::space_size as ss;
pub trait Type {
    type SpaceSize;
    type Grid;
    type Indexer;
    type IntPoint;
    type DoublePoint;
    type Position;
}

pub struct TwoDim;
pub struct ThreeDim;

impl Type for TwoDim {
    type Grid = gr::Grid2d;
    type SpaceSize = ss::SpaceSize2d;
    type Indexer = id::Indexer2d;
    type IntPoint = po::Point2d<i32>;
    type DoublePoint = po::Point2d<f64>;
    type Position = ps::Position2d;
}

impl Type for ThreeDim {
    type Grid = gr::Grid3d;
    type SpaceSize = ss::SpaceSize3d;
    type Indexer = id::Indexer3d;
    type IntPoint = po::Point3d<i32>;
    type DoublePoint = po::Point3d<f64>;
    type Position = ps::Position3d;
}

pub type Grid<D> = <D as Type>::Grid;
pub type SpaceSize<D> = <D as Type>::SpaceSize;
pub type Indexer<D> = <D as Type>::Indexer;
pub type IntPoint<D> = <D as Type>::IntPoint;
pub type DoublePoint<D> = <D as Type>::DoublePoint;
pub type Position<D> = <D as Type>::Position;
