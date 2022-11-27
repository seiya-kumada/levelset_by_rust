use super::differential::{Differential2d, Differential3d};
use super::neighboring_point::{NEIGHBORING_POINTS2D, NEIGHBORING_POINTS3D};
use crate::core::front::{Front2d, Front3d};
use crate::core::grid::{Grid2d, Grid3d};
use crate::core::grid_range::{GridRange2d, GridRange3d};
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::initial_front::{InitialFront2d, InitialFront3d};
use crate::core::inside_estimator::{InsideEstimator2d, InsideEstimator3d};
use crate::core::point::{Point2d, Point3d};
use crate::core::position::{Position2d, Position3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::speed_factor::{SpeedFactor2d, SpeedFactor3d};
use crate::core::status::Status;
use crate::core::upwind::{Upwind2d, Upwind3d};
use crate::core::upwind_scheme::{UpwindScheme2d, UpwindScheme3d};
use crate::core::util;
use num_traits::ToPrimitive;
use num_traits::Zero;
use std::ops::Add;
use std::rc::Rc;

pub trait Type {
    const NUM: i32;

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

    fn make_indexer(space_size: &Self::SpaceSize_) -> Self::Indexer_;
    fn make_upwind_scheme(indexer: Rc<Self::Indexer_>, phi: Rc<Vec<f64>>) -> Self::UpwindScheme_;
    fn get_total(space_size: &Self::SpaceSize_) -> usize;
    fn make_speed_factor(indexer: Rc<Self::Indexer_>, gray: Rc<Vec<u8>>) -> Self::SpeedFactor_;
    fn make_grid_range(space_size: &Self::SpaceSize_) -> Self::GridRange_;
    fn make_int_point_vec() -> Vec<Self::IntPoint_>;
    fn make_double_point_vec() -> Vec<Self::DoublePoint_>;
    fn create_initial_front(front: &Self::InitialFront_, grid: &mut Self::Grid_);
    fn initialize_inside_estimator() -> Self::InsideEstimator_;
    fn set_grid(front: &Self::Grid_, inside: &mut Self::InsideEstimator_);
    fn create_space_with_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_;
    fn create_space_without_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_;
    fn loop_in_grid_range(
        grid_range: &Self::GridRange_,
        indexer: &Self::Indexer_,
        statuses: &Vec<Status>,
        band: &mut Vec<Self::IntPoint_>,
        fun: fn(&Self::Indexer_, &Vec<Status>, &mut Vec<Self::IntPoint_>, Self::IntPoint_),
    );
    fn get_index(indexer: &Self::Indexer_, p: &Self::IntPoint_) -> i32;
    fn is_inside(estimator: &Self::InsideEstimator_, p: &Self::IntPoint_) -> bool;
    fn get_speed_factor(factor: &Self::SpeedFactor_, p: &Self::IntPoint_) -> f64;
    fn calculate_all(factor: &mut Self::SpeedFactor_, size: &Self::SpaceSize_);
}

pub struct TwoDim;
pub struct ThreeDim;

impl Type for TwoDim {
    const NUM: i32 = 2;

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

    fn make_indexer(space_size: &Self::SpaceSize_) -> Self::Indexer_ {
        Self::Indexer_::new(space_size)
    }

    fn make_upwind_scheme(indexer: Rc<Self::Indexer_>, phi: Rc<Vec<f64>>) -> Self::UpwindScheme_ {
        Self::UpwindScheme_::new(indexer, phi)
    }

    fn get_total(space_size: &Self::SpaceSize_) -> usize {
        space_size.total as usize
    }

    fn make_speed_factor(indexer: Rc<Self::Indexer_>, gray: Rc<Vec<u8>>) -> Self::SpeedFactor_ {
        Self::SpeedFactor_::new(indexer, gray)
    }

    fn make_grid_range(space_size: &Self::SpaceSize_) -> Self::GridRange_ {
        Self::GridRange_::new(space_size)
    }

    fn make_int_point_vec() -> Vec<Self::IntPoint_> {
        Vec::<Self::IntPoint_>::new()
    }

    fn make_double_point_vec() -> Vec<Self::DoublePoint_> {
        Vec::<Self::DoublePoint_>::new()
    }

    fn create_initial_front(front: &Self::InitialFront_, grid: &mut Self::Grid_) {
        grid.create_initial_front(front);
    }

    fn initialize_inside_estimator() -> Self::InsideEstimator_ {
        Self::InsideEstimator_::new()
    }

    fn set_grid(front: &Self::Grid_, inside: &mut Self::InsideEstimator_) {
        inside.set_grid(front.clone());
    }

    fn create_space_with_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_ {
        Self::InsideEstimator_::from_grid(Self::Grid_::create_space_with_edge(space_size))
    }

    fn create_space_without_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_ {
        Self::InsideEstimator_::from_grid(Self::Grid_::create_space_without_edge(space_size))
    }

    fn loop_in_grid_range(
        grid_range: &Self::GridRange_,
        indexer: &Self::Indexer_,
        statuses: &Vec<Status>,
        band: &mut Vec<Self::IntPoint_>,
        fun: fn(&Self::Indexer_, &Vec<Status>, &mut Vec<Self::IntPoint_>, Self::IntPoint_),
    ) {
        grid_range.foreach(indexer, statuses, band, fun);
    }

    fn get_index(indexer: &Self::Indexer_, p: &Self::IntPoint_) -> i32 {
        indexer.get(p)
    }

    fn is_inside(estimator: &Self::InsideEstimator_, p: &Self::IntPoint_) -> bool {
        estimator.is_inside(p)
    }

    fn get_speed_factor(factor: &Self::SpeedFactor_, p: &Self::IntPoint_) -> f64 {
        factor.get_value(p)
    }

    fn calculate_all(factor: &mut Self::SpeedFactor_, size: &Self::SpaceSize_) {
        factor.calculate_all(size);
    }
}

impl Type for ThreeDim {
    const NUM: i32 = 3;

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

    fn make_indexer(space_size: &Self::SpaceSize_) -> Self::Indexer_ {
        Self::Indexer_::new(space_size)
    }

    fn make_upwind_scheme(indexer: Rc<Self::Indexer_>, phi: Rc<Vec<f64>>) -> Self::UpwindScheme_ {
        Self::UpwindScheme_::new(indexer, phi)
    }

    fn get_total(space_size: &Self::SpaceSize_) -> usize {
        space_size.total as usize
    }

    fn make_speed_factor(indexer: Rc<Self::Indexer_>, gray: Rc<Vec<u8>>) -> Self::SpeedFactor_ {
        Self::SpeedFactor_::new(indexer, gray)
    }

    fn make_grid_range(space_size: &Self::SpaceSize_) -> Self::GridRange_ {
        Self::GridRange_::new(space_size)
    }

    fn make_int_point_vec() -> Vec<Self::IntPoint_> {
        Vec::<Self::IntPoint_>::new()
    }

    fn make_double_point_vec() -> Vec<Self::DoublePoint_> {
        Vec::<Self::DoublePoint_>::new()
    }

    fn create_initial_front(front: &Self::InitialFront_, grid: &mut Self::Grid_) {
        grid.create_initial_front(front);
    }

    fn initialize_inside_estimator() -> Self::InsideEstimator_ {
        Self::InsideEstimator_::new()
    }

    fn set_grid(front: &Self::Grid_, inside: &mut Self::InsideEstimator_) {
        inside.set_grid(front.clone());
    }

    fn create_space_with_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_ {
        Self::InsideEstimator_::from_grid(Self::Grid_::create_space_with_edge(space_size))
    }

    fn create_space_without_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_ {
        Self::InsideEstimator_::from_grid(Self::Grid_::create_space_without_edge(space_size))
    }

    fn loop_in_grid_range(
        grid_range: &Self::GridRange_,
        indexer: &Self::Indexer_,
        statuses: &Vec<Status>,
        band: &mut Vec<Self::IntPoint_>,
        fun: fn(&Self::Indexer_, &Vec<Status>, &mut Vec<Self::IntPoint_>, Self::IntPoint_),
    ) {
        grid_range.foreach(indexer, statuses, band, fun);
    }

    fn get_index(indexer: &Self::Indexer_, p: &Self::IntPoint_) -> i32 {
        indexer.get(p)
    }

    fn is_inside(estimator: &Self::InsideEstimator_, p: &Self::IntPoint_) -> bool {
        estimator.is_inside(p)
    }

    fn get_speed_factor(factor: &Self::SpeedFactor_, p: &Self::IntPoint_) -> f64 {
        factor.get_value(p)
    }

    fn calculate_all(factor: &mut Self::SpeedFactor_, size: &Self::SpaceSize_) {
        factor.calculate_all(size);
    }
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
