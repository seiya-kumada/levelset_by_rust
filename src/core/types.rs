use super::differential::{Differential2d, Differential3d};
use crate::core::distance_map_generator::{
    DistanceMap2d, DistanceMap3d, DistanceMapGenerator2d, DistanceMapGenerator3d,
};
use crate::core::front::{Front2d, Front3d};
use crate::core::grid::{Grid2d, Grid3d};
use crate::core::grid_range::{GridRange2d, GridRange3d};
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::initial_front::{InitialFront2d, InitialFront3d};
use crate::core::inside_estimator::{InsideEstimator2d, InsideEstimator3d};
use crate::core::level_set_method::{LevelSetMethod2d, LevelSetMethod3d};
use crate::core::point::{Point2d, Point3d};
use crate::core::position::{Position2d, Position3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::speed_factor::{SpeedFactor2d, SpeedFactor3d};
use crate::core::upwind::{Upwind2d, Upwind3d};
use crate::core::upwind_scheme::{UpwindScheme2d, UpwindScheme3d};

pub trait Type {
    const NUM: usize;

    type SpaceSize_;
    type Grid_;
    type Indexer_;
    type IntPoint_;
    type DoublePoint_;
    type Position_;
    type Upwind_;
    type DifferentialU8_;
    type SpeedFactor_;
    type InitialFront_;
    type GridRange_;
    type UpwindScheme_;
    type Front_;
    type InsideEstimator_;
    type DistanceMapGenerator_;
    type DistanceMap_;
    type LevelSetMethod_;
}

pub struct TwoDim;
pub struct ThreeDim;

impl Type for TwoDim {
    const NUM: usize = 2;

    type Grid_ = Grid2d;
    type SpaceSize_ = SpaceSize2d;
    type Indexer_ = Indexer2d;
    type IntPoint_ = Point2d<i32>;
    type DoublePoint_ = Point2d<f64>;
    type Position_ = Position2d;
    type Upwind_ = Upwind2d;
    type DifferentialU8_ = Differential2d<u8>;
    type SpeedFactor_ = SpeedFactor2d;
    type InitialFront_ = InitialFront2d;
    type GridRange_ = GridRange2d;
    type UpwindScheme_ = UpwindScheme2d;
    type Front_ = Front2d;
    type InsideEstimator_ = InsideEstimator2d;
    type DistanceMapGenerator_ = DistanceMapGenerator2d;
    type DistanceMap_ = DistanceMap2d;
    type LevelSetMethod_ = LevelSetMethod2d;
}

impl Type for ThreeDim {
    const NUM: usize = 3;

    type Grid_ = Grid3d;
    type SpaceSize_ = SpaceSize3d;
    type Indexer_ = Indexer3d;
    type IntPoint_ = Point3d<i32>;
    type DoublePoint_ = Point3d<f64>;
    type Position_ = Position3d;
    type Upwind_ = Upwind3d;
    type SpeedFactor_ = SpeedFactor3d;
    type DifferentialU8_ = Differential3d<u8>;
    type InitialFront_ = InitialFront3d;
    type GridRange_ = GridRange3d;
    type UpwindScheme_ = UpwindScheme3d;
    type Front_ = Front3d;
    type InsideEstimator_ = InsideEstimator3d;
    type DistanceMapGenerator_ = DistanceMapGenerator3d;
    type DistanceMap_ = DistanceMap3d;
    type LevelSetMethod_ = LevelSetMethod3d;
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
pub type GridRange<D> = <D as Type>::GridRange_;
pub type UpwindScheme<D> = <D as Type>::UpwindScheme_;
pub type Front<D> = <D as Type>::Front_;
pub type InsideEstimator<D> = <D as Type>::InsideEstimator_;
pub type DistanceMapGenerator<D> = <D as Type>::DistanceMapGenerator_;
pub type DistanceMap<D> = <D as Type>::DistanceMap_;
pub type LevelSetMethod<D> = <D as Type>::LevelSetMethod_;
