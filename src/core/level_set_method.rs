use crate::core::distance_map_generator::{
    DistanceMapGenerator2d, DistanceMapGenerator3d, DistanceMapGeneratorMethod,
};
use crate::core::grid::{Grid2d, Grid3d, GridMethod};
use crate::core::grid_range::{GridRange2d, GridRange3d, GridRangeMethod};
use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::initial_front::{InitialFront2d, InitialFront3d};
use crate::core::inside_estimator::{InsideEstimator2d, InsideEstimator3d, InsideEstimatorMethod};
use crate::core::parameters::Parameters;
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d, SpaceSizeMethod};
use crate::core::speed_factor::{SpeedFactor2d, SpeedFactor3d, SpeedFactorMethod};
use crate::core::status::Status;
use crate::core::stopping_condition::StoppingCondition;
use crate::core::upwind_scheme::{New, UpwindScheme2d, UpwindScheme3d};
use std::marker::PhantomData;
use std::rc::Rc;

pub struct LevelSetMethod<
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
    SpaceSize: SpaceSizeMethod,
    Indexer: IndexerMethod<SpaceSize, IntPoint>,
    UpwindScheme: New<Indexer>,
    SpeedFactor: SpeedFactorMethod<Indexer, IntPoint, SpaceSize>,
    GridRange: GridRangeMethod<SpaceSize, Indexer, IntPoint>,
    DistanceMapGenerator: DistanceMapGeneratorMethod<Indexer>,
    Grid: GridMethod<InitialFront, SpaceSize>,
    InsideEstimator: InsideEstimatorMethod<Grid, IntPoint>,
{
    phantom_initial_front: PhantomData<InitialFront>,

    /// input parameters
    parameters: Parameters,

    /// size of the input image/3Dmodel
    size: Rc<SpaceSize>,

    /// accessor of the array
    indexer: Indexer,

    /// input front(zero-levelset)
    initial_front: Grid,

    /// auxiliary function
    phi: Rc<Vec<f64>>,

    /// deviation of auxiliary function
    dphi: Rc<Vec<f64>>,

    /// velocity function
    speed: Vec<f64>,

    /// current statuses
    statuses: Rc<Vec<Status>>,

    /// front
    front: Vec<IntPoint>,

    /// normals
    normals: Vec<DoublePoint>,

    /// narrow band
    narrow_bands: Vec<IntPoint>,

    /// input image(gray image)
    input_object: Rc<Vec<u8>>,

    upwind_scheme: UpwindScheme,
    speed_factor: SpeedFactor,

    grid_range: GridRange,

    inside_estimator_of_space_without_edge: InsideEstimator,
    inside_estimator_of_space_with_edge: InsideEstimator,
    inside_estimator_of_initial_front: InsideEstimator,

    total_speed: f64,

    stopping_condition: StoppingCondition,
    zero_count: i32,
    distance_map_generator: DistanceMapGenerator,
}
impl<
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
    >
    LevelSetMethod<
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
    >
where
    SpaceSize: SpaceSizeMethod,
    Indexer: IndexerMethod<SpaceSize, IntPoint>,
    UpwindScheme: New<Indexer>,
    SpeedFactor: SpeedFactorMethod<Indexer, IntPoint, SpaceSize>,
    GridRange: GridRangeMethod<SpaceSize, Indexer, IntPoint>,
    DistanceMapGenerator: DistanceMapGeneratorMethod<Indexer>,
    Grid: GridMethod<InitialFront, SpaceSize>,
    InsideEstimator: InsideEstimatorMethod<Grid, IntPoint>,
{
    pub fn new(
        parameters: Parameters,
        size: Rc<SpaceSize>,
        gray: Rc<Vec<u8>>,
        initial_front: Grid,
    ) {
        let statuses = Rc::new(vec![Status::Farway; size.get_total()]);
        let indexer = Rc::new(Indexer::new(&size));
        let phi = Rc::new(vec![0.0; size.get_total()]);
        //Self {
        //    parameters: parameters.clone(),
        //    size: Rc::clone(&size),
        //    indexer: Rc::clone(&indexer),
        //    initial_front,
        //    phi: Rc::clone(&phi),
        //    dphi: Rc::new(vec![0.0; D::get_total(&size)]),
        //    speed: vec![0.0; D::get_total(&size)],
        //    statuses: Rc::clone(&statuses),
        //    upwind_scheme: D::make_upwind_scheme(Rc::clone(&indexer), Rc::clone(&phi)),
        //    speed_factor: D::make_speed_factor(Rc::clone(&indexer), Rc::clone(&gray)),
        //    grid_range: D::make_grid_range(&size),
        //    input_object: Rc::clone(&gray),
        //    front: D::make_int_point_vec(),
        //    narrow_bands: D::make_int_point_vec(),
        //    normals: D::make_double_point_vec(),
        //    inside_estimator_of_space_without_edge: D::create_space_without_edge(Rc::clone(&size)),
        //    inside_estimator_of_space_with_edge: D::create_space_with_edge(Rc::clone(&size)),
        //    inside_estimator_of_initial_front: D::initialize_inside_estimator(),
        //    total_speed: 0.0,
        //    stopping_condition: StoppingCondition::new(),
        //    zero_count: 0,
        //    distance_map_generator: D::make_distance_map_generator(
        //        parameters.wband,
        //        Rc::clone(&indexer),
        //        Rc::clone(&statuses),
        //    ),
        //}
    }

    pub fn initialize_narrow_band(&mut self) {
        self.narrow_bands.clear();
        self.set_speed_function(true);
    }

    fn calculate_speed_factors(&mut self) {
        self.speed_factor.calculate_all(&self.size);
    }

    fn initailze_distance_map(&mut self) {
        self.distance_map_generator.create_distance_map();
    }

    fn get_size(&self) -> Rc<SpaceSize> {
        Rc::clone(&self.size)
    }

    pub fn initialize_along_front(&mut self, initial_front: &InitialFront) {
        self.front.clear();
        self.normals.clear();
        self.initial_front.create_initial_front(initial_front);
        self.inside_estimator_of_initial_front
            .set_grid(&self.initial_front);
    }

    fn initailze_over_all(&self, initial_front: &InitialFront) {
        self.grid_range.foreach(
            &self.indexer,
            &self.statuses,
            &mut Rc::clone(&self.phi),
            Self::register_to_phi,
        );
    }

    fn register_to_phi(
        indexer: &Indexer,
        statuses: &Vec<Status>,
        phi: &mut Rc<Vec<f64>>,
        p: IntPoint,
    ) {
        let index = indexer.get(&p);
        match statuses[index as usize] {
            Status::Front => {
                //phi[index as usize] = 1.0;
            }
            _ => (),
        }
    }

    fn clear_speed_within_narrow_band(&self, reset: bool) {}

    fn set_speed_on_front(&mut self) -> f64 {
        let fs = 0.0;
        for p in &self.front {
            if self.inside_estimator_of_space_without_edge.is_inside(p) {
                let i = self.indexer.get(&p) as usize;
                let speed_factor = self.speed_factor.get_value(p);
                self.speed[i] = speed_factor.clone();
            }
        }
        0.0
    }

    fn copy_nearest_speed_to_narrow_band(&self, resets: bool) {}

    fn register_to_narrow_band(
        indexer: &Indexer,
        statuses: &Vec<Status>,
        band: &mut Vec<IntPoint>,
        p: IntPoint,
    ) {
        let index = indexer.get(&p);
        match statuses[index as usize] {
            Status::Farway => band.push(p),
            _ => (),
        }
    }

    fn set_speed_function(&mut self, resets: bool) -> bool {
        self.clear_speed_within_narrow_band(resets);
        self.total_speed = self.set_speed_on_front();
        self.copy_nearest_speed_to_narrow_band(resets);
        if resets {
            self.narrow_bands.clear();
            self.grid_range.foreach(
                &self.indexer,
                &self.statuses,
                &mut self.narrow_bands,
                Self::register_to_narrow_band,
            );
        }
        self.stopping_condition.add_total_speed(self.total_speed);
        self.stopping_condition.is_satisfied()
    }
}

pub type LevelSetMethod2d = LevelSetMethod<
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
>;

pub type LevelSetMethod3d = LevelSetMethod<
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
>;
