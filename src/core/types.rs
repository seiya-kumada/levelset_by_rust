use super::differential::{Differential2d, Differential3d};
use super::neighboring_point::{NEIGHBORING_POINTS2D, NEIGHBORING_POINTS3D};
use crate::core::distance_map_generator;
use crate::core::distance_map_generator::{DistanceMapGenerator2d, DistanceMapGenerator3d};
use crate::core::front::{Front2d, Front3d};
use crate::core::grid;
use crate::core::grid::{Grid2d, Grid3d};
use crate::core::grid_range;
use crate::core::grid_range::{GridRange2d, GridRange3d};
use crate::core::indexer;
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::initial_front::{InitialFront2d, InitialFront3d};
use crate::core::inside_estimator;
use crate::core::inside_estimator::{InsideEstimator2d, InsideEstimator3d};
use crate::core::point::{Point2d, Point3d};
use crate::core::position::{Position2d, Position3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::speed_factor;
use crate::core::speed_factor::{SpeedFactor2d, SpeedFactor3d};
use crate::core::status::Status;
use crate::core::upwind::{Upwind2d, Upwind3d};
use crate::core::upwind_scheme;
use crate::core::upwind_scheme::{UpwindScheme2d, UpwindScheme3d};
use crate::core::util;
use num_traits::ToPrimitive;
use num_traits::Zero;
use std::ops::Add;
use std::rc::Rc;

//pub trait Method_<SpaceSize, IntPoint, Indexer: indexer::IndexerMethod<SpaceSize, IntPoint>> {}
//pub trait Method<
//    SpaceSize,
//    Indexer: indexer::IndexerMethod<SpaceSize, IntPoint>,
//    UpwindScheme: upwind_scheme::New<Indexer>,
//    SpeedFactor: speed_factor::SpeedFactorMethod<Indexer, IntPoint, SpaceSize>,
//    GridRange: grid_range::GridRangeMethod<SpaceSize, Indexer, IntPoint>,
//    IntPoint,
//    DoublePoint,
//    DistanceMapGenerator: distance_map_generator::DistanceMapGeneratorMethod<Indexer>,
//    InitialFront,
//    Grid: grid::GridMethod<InitialFront, SpaceSize>,
//    InsideEstimator: inside_estimator::InsideEstimatorMethod<Grid, IntPoint>,
//>
pub trait Method_<
    SpaceSize,
    Indexer,
    UpwindScheme,
    SpeedFactor,
    GridRange,
    IntPoint,
    DoublePoint,
    DistanceMapGenerator,
    InitialFront,
    Grid,
    InsideEstimator,
> where
    Indexer: indexer::IndexerMethod<SpaceSize, IntPoint>,
    UpwindScheme: upwind_scheme::New<Indexer>,
    SpeedFactor: speed_factor::SpeedFactorMethod<Indexer, IntPoint, SpaceSize>,
    GridRange: grid_range::GridRangeMethod<SpaceSize, Indexer, IntPoint>,
    DistanceMapGenerator: distance_map_generator::DistanceMapGeneratorMethod<Indexer>,
    Grid: grid::GridMethod<InitialFront, SpaceSize>,
    InsideEstimator: inside_estimator::InsideEstimatorMethod<Grid, IntPoint>,
{
    fn make_indexer(space_size: &SpaceSize) -> Indexer {
        Indexer::new(space_size)
    }

    fn make_upwind_scheme(indexer: Rc<Indexer>, phi: Rc<Vec<f64>>) -> UpwindScheme {
        UpwindScheme::new(indexer, phi)
    }

    fn get_total(space_size: &SpaceSize) -> usize;

    fn make_speed_factor(indexer: Rc<Indexer>, gray: Rc<Vec<u8>>) -> SpeedFactor {
        SpeedFactor::new(indexer, gray)
    }

    fn make_grid_range(space_size: &SpaceSize) -> GridRange {
        GridRange::new(space_size)
    }

    fn make_int_point_vec() -> Vec<IntPoint> {
        Vec::<IntPoint>::new()
    }

    fn make_double_point_vec() -> Vec<DoublePoint> {
        Vec::<DoublePoint>::new()
    }

    fn make_distance_map_generator(
        wband: i32,
        indexer: Rc<Indexer>,
        statuses: Rc<Vec<Status>>,
    ) -> DistanceMapGenerator {
        DistanceMapGenerator::new(wband, Rc::clone(&indexer), Rc::clone(&statuses))
    }

    fn create_initial_front(front: &InitialFront, grid: &mut Grid) {
        grid.create_initial_front(front);
    }

    fn initialize_inside_estimator() -> InsideEstimator {
        InsideEstimator::new()
    }

    fn set_grid(front: &Grid, inside: &mut InsideEstimator) {
        inside.set_grid(front);
    }

    fn create_space_with_edge(space_size: Rc<SpaceSize>) -> InsideEstimator {
        InsideEstimator::from_grid(Grid::create_space_with_edge(space_size))
    }

    fn create_space_without_edge(space_size: Rc<SpaceSize>) -> InsideEstimator {
        InsideEstimator::from_grid(Grid::create_space_without_edge(space_size))
    }

    // これと次の、一緒にできそうね。
    fn loop_in_grid_range_with_phi(
        grid_range: &GridRange,
        indexer: &Indexer,
        statuses: &Vec<Status>,
        phi: &mut Rc<Vec<f64>>,
        fun: fn(&Indexer, &Vec<Status>, &mut Rc<Vec<f64>>, IntPoint),
    ) {
        grid_range.foreach(indexer, statuses, phi, fun);
    }

    fn loop_in_grid_range(
        grid_range: &GridRange,
        indexer: &Indexer,
        statuses: &Vec<Status>,
        band: &mut Vec<IntPoint>,
        fun: fn(&Indexer, &Vec<Status>, &mut Vec<IntPoint>, IntPoint),
    ) {
        grid_range.foreach(indexer, statuses, band, fun);
    }

    fn get_index(indexer: &Indexer, p: &IntPoint) -> i32 {
        indexer.get(p)
    }

    fn is_inside(estimator: &InsideEstimator, p: &IntPoint) -> bool {
        estimator.is_inside(p)
    }

    fn get_speed_factor(factor: &SpeedFactor, p: &IntPoint) -> f64 {
        factor.get_value(p)
    }

    //fn calculate_all(factor: &mut SpeedFactor, size: &SpaceSize) {
    //    factor.calculate_all(size);
    //}

    fn create_distance_map(distance_map_generator: &mut DistanceMapGenerator) {
        distance_map_generator.create_distance_map();
    }
}

impl
    Method_<
        SpaceSize2d,
        Indexer2d,
        UpwindScheme2d,
        SpeedFactor2d,
        GridRange2d,
        Point2d<i32>,
        Point2d<f64>,
        DistanceMapGenerator2d,
        InitialFront2d,
        Grid2d,
        InsideEstimator2d,
    > for TwoDim
{
    fn get_total(space_size: &SpaceSize2d) -> usize {
        space_size.total as usize
    }
}
impl
    Method_<
        SpaceSize3d,
        Indexer3d,
        UpwindScheme3d,
        SpeedFactor3d,
        GridRange3d,
        Point3d<i32>,
        Point3d<f64>,
        DistanceMapGenerator3d,
        InitialFront3d,
        Grid3d,
        InsideEstimator3d,
    > for ThreeDim
{
    fn get_total(space_size: &SpaceSize3d) -> usize {
        space_size.total as usize
    }
}

//pub trait Type_ {
//    type SpaceSize_;
//    type Indexer_;
//    type UpwindScheme_;
//}
//
//impl Type_ for TwoDim {
//    type SpaceSize_ = SpaceSize2d;
//    type Indexer_ = Indexer2d;
//    type UpwindScheme_ = UpwindScheme2d;
//}
//
//impl Type_ for ThreeDim {
//    type SpaceSize_ = SpaceSize3d;
//    type Indexer_ = Indexer3d;
//    type UpwindScheme_ = UpwindScheme3d;
//}
//
//pub type SpaceSize_<D> = <D as Type_>::SpaceSize_;
//pub type Indexer_<D> = <D as Type_>::Indexer_;
//pub type UpwindScheme_<D> = <D as Type_>::UpwindScheme_;

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

    //
    //fn make_indexer(space_size: &Self::SpaceSize_) -> Self::Indexer_;

    ////
    //fn make_upwind_scheme(indexer: Rc<Self::Indexer_>, phi: Rc<Vec<f64>>) -> Self::UpwindScheme_;

    ////
    //fn get_total(space_size: &Self::SpaceSize_) -> usize;

    ////
    //fn make_speed_factor(indexer: Rc<Self::Indexer_>, gray: Rc<Vec<u8>>) -> Self::SpeedFactor_;

    ////
    //fn make_grid_range(space_size: &Self::SpaceSize_) -> Self::GridRange_;

    ////
    //fn make_int_point_vec() -> Vec<Self::IntPoint_>;

    ////
    //fn make_double_point_vec() -> Vec<Self::DoublePoint_>;

    ////
    //fn make_distance_map_generator(
    //    wband: i32,
    //    indexer: Rc<Self::Indexer_>,
    //    statuses: Rc<Vec<Status>>,
    //) -> Self::DistanceMapGenerator_;

    ////
    //fn create_initial_front(front: &Self::InitialFront_, grid: &mut Self::Grid_);

    ////
    //fn initialize_inside_estimator() -> Self::InsideEstimator_;

    ////
    //fn set_grid(front: &Self::Grid_, inside: &mut Self::InsideEstimator_);

    ////
    //fn create_space_with_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_;

    ////
    //fn create_space_without_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_;

    ////
    //fn loop_in_grid_range(
    //    grid_range: &Self::GridRange_,
    //    indexer: &Self::Indexer_,
    //    statuses: &Vec<Status>,
    //    band: &mut Vec<Self::IntPoint_>,
    //    fun: fn(&Self::Indexer_, &Vec<Status>, &mut Vec<Self::IntPoint_>, Self::IntPoint_),
    //);

    ////
    //fn loop_in_grid_range_with_phi(
    //    grid_range: &Self::GridRange_,
    //    indexer: &Self::Indexer_,
    //    statuses: &Vec<Status>,
    //    phi: &mut Rc<Vec<f64>>,
    //    fun: fn(&Self::Indexer_, &Vec<Status>, &mut Rc<Vec<f64>>, Self::IntPoint_),
    //);

    ////
    //fn get_index(indexer: &Self::Indexer_, p: &Self::IntPoint_) -> i32;

    ////
    //fn is_inside(estimator: &Self::InsideEstimator_, p: &Self::IntPoint_) -> bool;

    ////
    //fn get_speed_factor(factor: &Self::SpeedFactor_, p: &Self::IntPoint_) -> f64;

    ////
    //fn calculate_all(factor: &mut Self::SpeedFactor_, size: &Self::SpaceSize_);

    ////
    //fn create_distance_map(distance_map_generator: &mut Self::DistanceMapGenerator_);
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
    //fn make_indexer(space_size: &Self::SpaceSize_) -> Self::Indexer_ {
    //    use crate::core::indexer::IndexerMethod;
    //    Self::Indexer_::new(space_size)
    //}

    //fn make_upwind_scheme(indexer: Rc<Self::Indexer_>, phi: Rc<Vec<f64>>) -> Self::UpwindScheme_ {
    //    use crate::core::upwind_scheme::New;
    //    Self::UpwindScheme_::new(indexer, phi)
    //}

    //fn get_total(space_size: &Self::SpaceSize_) -> usize {
    //    space_size.total as usize
    //}

    //fn make_speed_factor(indexer: Rc<Self::Indexer_>, gray: Rc<Vec<u8>>) -> Self::SpeedFactor_ {
    //    use crate::core::speed_factor::SpeedFactorMethod;
    //    Self::SpeedFactor_::new(indexer, gray)
    //}

    //fn make_grid_range(space_size: &Self::SpaceSize_) -> Self::GridRange_ {
    //    use crate::core::grid_range::GridRangeMethod;
    //    Self::GridRange_::new(space_size)
    //}

    //fn make_int_point_vec() -> Vec<Self::IntPoint_> {
    //    Vec::<Self::IntPoint_>::new()
    //}

    //fn make_double_point_vec() -> Vec<Self::DoublePoint_> {
    //    Vec::<Self::DoublePoint_>::new()
    //}

    //fn make_distance_map_generator(
    //    wband: i32,
    //    indexer: Rc<Self::Indexer_>,
    //    statuses: Rc<Vec<Status>>,
    //) -> Self::DistanceMapGenerator_ {
    //    use crate::core::distance_map_generator::DistanceMapGeneratorMethod;
    //    Self::DistanceMapGenerator_::new(wband, Rc::clone(&indexer), Rc::clone(&statuses))
    //}

    //fn create_initial_front(front: &Self::InitialFront_, grid: &mut Self::Grid_) {
    //    use crate::core::grid::GridMethod;
    //    grid.create_initial_front(front);
    //}

    //fn initialize_inside_estimator() -> Self::InsideEstimator_ {
    //    use crate::core::inside_estimator::InsideEstimatorMethod;
    //    Self::InsideEstimator_::new()
    //}

    //fn set_grid(front: &Self::Grid_, inside: &mut Self::InsideEstimator_) {
    //    use crate::core::inside_estimator::InsideEstimatorMethod;
    //    inside.set_grid(&front);
    //}

    //fn create_space_with_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_ {
    //    use crate::core::grid::GridMethod;
    //    use crate::core::inside_estimator::InsideEstimatorMethod;
    //    Self::InsideEstimator_::from_grid(Self::Grid_::create_space_with_edge(space_size))
    //}

    //fn create_space_without_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_ {
    //    use crate::core::grid::GridMethod;
    //    use crate::core::inside_estimator::InsideEstimatorMethod;
    //    Self::InsideEstimator_::from_grid(Self::Grid_::create_space_without_edge(space_size))
    //}

    //fn loop_in_grid_range(
    //    grid_range: &Self::GridRange_,
    //    indexer: &Self::Indexer_,
    //    statuses: &Vec<Status>,
    //    band: &mut Vec<Self::IntPoint_>,
    //    fun: fn(&Self::Indexer_, &Vec<Status>, &mut Vec<Self::IntPoint_>, Self::IntPoint_),
    //) {
    //    use crate::core::grid_range::GridRangeMethod;
    //    grid_range.foreach(indexer, statuses, band, fun);
    //}

    //fn loop_in_grid_range_with_phi(
    //    grid_range: &Self::GridRange_,
    //    indexer: &Self::Indexer_,
    //    statuses: &Vec<Status>,
    //    phi: &mut Rc<Vec<f64>>,
    //    fun: fn(&Self::Indexer_, &Vec<Status>, &mut Rc<Vec<f64>>, Self::IntPoint_),
    //) {
    //    use crate::core::grid_range::GridRangeMethod;
    //    grid_range.foreach(indexer, statuses, phi, fun);
    //}

    //fn get_index(indexer: &Self::Indexer_, p: &Self::IntPoint_) -> i32 {
    //    use crate::core::indexer::IndexerMethod;
    //    indexer.get(p)
    //}

    //fn is_inside(estimator: &Self::InsideEstimator_, p: &Self::IntPoint_) -> bool {
    //    use crate::core::inside_estimator::InsideEstimatorMethod;
    //    estimator.is_inside(p)
    //}

    //fn get_speed_factor(factor: &Self::SpeedFactor_, p: &Self::IntPoint_) -> f64 {
    //    use crate::core::speed_factor::SpeedFactorMethod;
    //    factor.get_value(p)
    //}

    //fn calculate_all(factor: &mut Self::SpeedFactor_, size: &Self::SpaceSize_) {
    //    use crate::core::speed_factor::SpeedFactorMethod;
    //    factor.calculate_all(size);
    //}

    //fn create_distance_map(distance_map_generator: &mut Self::DistanceMapGenerator_) {
    //    use crate::core::distance_map_generator::DistanceMapGeneratorMethod;
    //    distance_map_generator.create_distance_map();
    //}
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
    //fn make_indexer(space_size: &Self::SpaceSize_) -> Self::Indexer_ {
    //    use crate::core::indexer::IndexerMethod;
    //    Self::Indexer_::new(space_size)
    //}

    //fn make_upwind_scheme(indexer: Rc<Self::Indexer_>, phi: Rc<Vec<f64>>) -> Self::UpwindScheme_ {
    //    use crate::core::upwind_scheme::New;
    //    Self::UpwindScheme_::new(indexer, phi)
    //}

    //fn get_total(space_size: &Self::SpaceSize_) -> usize {
    //    space_size.total as usize
    //}

    //fn make_speed_factor(indexer: Rc<Self::Indexer_>, gray: Rc<Vec<u8>>) -> Self::SpeedFactor_ {
    //    use crate::core::speed_factor::SpeedFactorMethod;
    //    Self::SpeedFactor_::new(indexer, gray)
    //}

    //fn make_grid_range(space_size: &Self::SpaceSize_) -> Self::GridRange_ {
    //    use crate::core::grid_range::GridRangeMethod;
    //    Self::GridRange_::new(space_size)
    //}

    //fn make_int_point_vec() -> Vec<Self::IntPoint_> {
    //    Vec::<Self::IntPoint_>::new()
    //}

    //fn make_double_point_vec() -> Vec<Self::DoublePoint_> {
    //    Vec::<Self::DoublePoint_>::new()
    //}

    //fn make_distance_map_generator(
    //    wband: i32,
    //    indexer: Rc<Self::Indexer_>,
    //    statuses: Rc<Vec<Status>>,
    //) -> Self::DistanceMapGenerator_ {
    //    use crate::core::distance_map_generator::DistanceMapGeneratorMethod;
    //    Self::DistanceMapGenerator_::new(wband, Rc::clone(&indexer), Rc::clone(&statuses))
    //}

    //fn create_initial_front(front: &Self::InitialFront_, grid: &mut Self::Grid_) {
    //    use crate::core::grid::GridMethod;
    //    grid.create_initial_front(front);
    //}

    //fn initialize_inside_estimator() -> Self::InsideEstimator_ {
    //    use crate::core::inside_estimator::InsideEstimatorMethod;
    //    Self::InsideEstimator_::new()
    //}

    //fn set_grid(front: &Self::Grid_, inside: &mut Self::InsideEstimator_) {
    //    inside.set_grid(front.clone());
    //}

    //fn create_space_with_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_ {
    //    use crate::core::grid::GridMethod;
    //    use crate::core::inside_estimator::InsideEstimatorMethod;
    //    Self::InsideEstimator_::from_grid(Self::Grid_::create_space_with_edge(space_size))
    //}

    //fn create_space_without_edge(space_size: Rc<Self::SpaceSize_>) -> Self::InsideEstimator_ {
    //    use crate::core::grid::GridMethod;
    //    use crate::core::inside_estimator::InsideEstimatorMethod;
    //    Self::InsideEstimator_::from_grid(Self::Grid_::create_space_without_edge(space_size))
    //}

    //fn loop_in_grid_range(
    //    grid_range: &Self::GridRange_,
    //    indexer: &Self::Indexer_,
    //    statuses: &Vec<Status>,
    //    band: &mut Vec<Self::IntPoint_>,
    //    fun: fn(&Self::Indexer_, &Vec<Status>, &mut Vec<Self::IntPoint_>, Self::IntPoint_),
    //) {
    //    use crate::core::grid_range::GridRangeMethod;
    //    grid_range.foreach(indexer, statuses, band, fun);
    //}

    //fn loop_in_grid_range_with_phi(
    //    grid_range: &Self::GridRange_,
    //    indexer: &Self::Indexer_,
    //    statuses: &Vec<Status>,
    //    phi: &mut Rc<Vec<f64>>,
    //    fun: fn(&Self::Indexer_, &Vec<Status>, &mut Rc<Vec<f64>>, Self::IntPoint_),
    //) {
    //    use crate::core::grid_range::GridRangeMethod;
    //    grid_range.foreach(indexer, statuses, phi, fun);
    //}

    //fn get_index(indexer: &Self::Indexer_, p: &Self::IntPoint_) -> i32 {
    //    use crate::core::indexer::IndexerMethod;
    //    indexer.get(p)
    //}

    //fn is_inside(estimator: &Self::InsideEstimator_, p: &Self::IntPoint_) -> bool {
    //    use crate::core::inside_estimator::InsideEstimatorMethod;
    //    estimator.is_inside(p)
    //}

    //fn get_speed_factor(factor: &Self::SpeedFactor_, p: &Self::IntPoint_) -> f64 {
    //    use crate::core::speed_factor::SpeedFactorMethod;
    //    factor.get_value(p)
    //}

    //fn calculate_all(factor: &mut Self::SpeedFactor_, size: &Self::SpaceSize_) {
    //    use crate::core::speed_factor::SpeedFactorMethod;
    //    factor.calculate_all(size);
    //}

    //fn create_distance_map(distance_map_generator: &mut Self::DistanceMapGenerator_) {
    //    use crate::core::distance_map_generator::DistanceMapGeneratorMethod;
    //    distance_map_generator.create_distance_map();
    //}
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
