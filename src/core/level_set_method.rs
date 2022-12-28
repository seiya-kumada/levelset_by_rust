use opencv::prelude::AlgorithmTrait;

use crate::core::curvature_generator::{
    CurvatureGenerator2d, CurvatureGenerator3d, CurvatureGeneratorMethod,
};
use crate::core::distance_map_generator::{
    DistanceMap2d, DistanceMap3d, DistanceMapGenerator2d, DistanceMapGenerator3d,
    DistanceMapGeneratorMethod,
};
use crate::core::grid::{Grid2d, Grid3d, GridMethod};
use crate::core::grid_range::{GridRange2d, GridRange3d, GridRangeMethod};
use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::initial_front::{InitialFront2d, InitialFront3d};
use crate::core::inside_estimator::{InsideEstimator2d, InsideEstimator3d, InsideEstimatorMethod};
use crate::core::parameters::Parameters;
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d, SpaceSizeMethod};
use crate::core::speed::Speed;
use crate::core::speed_factor::{SpeedFactor2d, SpeedFactor3d, SpeedFactorMethod};
use crate::core::status::Status;
use crate::core::stopping_condition::StoppingCondition;
use crate::core::upwind_scheme::{UpwindScheme2d, UpwindScheme3d, UpwindSchemeMethod};
use std::cell::RefCell;
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
    DistanceMap,
    InitialFront,
    Grid,
    InsideEstimator,
    CurvatureGenerator,
> where
    IntPoint: Copy,
    SpaceSize: SpaceSizeMethod,
    Indexer: IndexerMethod<SpaceSize, IntPoint>,
    UpwindScheme: UpwindSchemeMethod<Indexer, IntPoint>,
    SpeedFactor: SpeedFactorMethod<Indexer, IntPoint, SpaceSize>,
    GridRange: GridRangeMethod<SpaceSize, Indexer, IntPoint, Self>,
    DistanceMapGenerator: DistanceMapGeneratorMethod<Indexer, DistanceMap, IntPoint>,
    Grid: GridMethod<InitialFront, SpaceSize, Self, IntPoint>,
    InsideEstimator: InsideEstimatorMethod<Grid, IntPoint>,
    CurvatureGenerator: CurvatureGeneratorMethod<Indexer, IntPoint, DoublePoint>,
{
    phantom_initial_front: PhantomData<InitialFront>,
    phantom_distance_map: PhantomData<DistanceMap>,

    /// input parameters
    parameters: Parameters,

    /// size of the input image/3Dmodel
    size: Rc<SpaceSize>,

    /// accessor of the array
    indexer: Rc<Indexer>,

    /// input front(zero-levelset)
    initial_front: Grid,

    /// auxiliary function
    phi: Rc<RefCell<Vec<f64>>>,

    /// deviation of auxiliary function
    dphi: Rc<RefCell<Vec<f64>>>,

    /// velocity function
    speed: Vec<f64>,

    /// current statuses
    statuses: Rc<RefCell<Vec<Status>>>,

    /// front
    front: Rc<RefCell<Vec<IntPoint>>>,

    /// normals
    normals: Vec<DoublePoint>,

    /// narrow band
    narrow_bands: Vec<IntPoint>,

    /// input image(gray image)
    input_object: Rc<RefCell<Vec<u8>>>,

    upwind_scheme: UpwindScheme,
    speed_factor: SpeedFactor,

    grid_range: GridRange,

    inside_estimator_for_space_without_edge: InsideEstimator,
    inside_estimator_for_space_with_edge: InsideEstimator,
    inside_estimator_for_initial_front: InsideEstimator,

    total_speed: f64,

    stopping_condition: StoppingCondition,
    zero_count: i32,
    distance_map_generator: DistanceMapGenerator,
    curvature_generator: CurvatureGenerator,
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
        DistanceMap,
        InitialFront,
        Grid,
        InsideEstimator,
        CurvatureGenerator,
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
        DistanceMap,
        InitialFront,
        Grid,
        InsideEstimator,
        CurvatureGenerator,
    >
where
    IntPoint: Copy,
    SpaceSize: SpaceSizeMethod,
    Indexer: IndexerMethod<SpaceSize, IntPoint>,
    UpwindScheme: UpwindSchemeMethod<Indexer, IntPoint>,
    SpeedFactor: SpeedFactorMethod<Indexer, IntPoint, SpaceSize>,
    GridRange: GridRangeMethod<SpaceSize, Indexer, IntPoint, Self>,
    DistanceMapGenerator: DistanceMapGeneratorMethod<Indexer, DistanceMap, IntPoint>,
    Grid: GridMethod<InitialFront, SpaceSize, Self, IntPoint>,
    InsideEstimator: InsideEstimatorMethod<Grid, IntPoint>,
    CurvatureGenerator: CurvatureGeneratorMethod<Indexer, IntPoint, DoublePoint>,
{
    pub fn new(parameters: Parameters, size: Rc<SpaceSize>, gray: Rc<RefCell<Vec<u8>>>) -> Self {
        let statuses = RefCell::new(vec![Status::Farway; size.get_total()]);
        let indexer = Rc::new(Indexer::new(&size));
        let phi = Rc::new(RefCell::new(vec![0.0; size.get_total()]));
        let initial_front = Grid::new();
        Self {
            phantom_initial_front: PhantomData,
            phantom_distance_map: PhantomData,
            parameters: parameters.clone(),
            size: Rc::clone(&size),
            indexer: Rc::clone(&indexer),
            initial_front,
            phi: Rc::new(RefCell::clone(&phi)),
            dphi: Rc::new(RefCell::new(vec![0.0; size.get_total()])),
            speed: vec![0.0; size.get_total()],
            statuses: Rc::new(RefCell::clone(&statuses)),
            upwind_scheme: UpwindScheme::new(Rc::clone(&indexer), Rc::clone(&phi)),
            speed_factor: SpeedFactor::new(Rc::clone(&indexer), RefCell::clone(&gray)),
            grid_range: GridRange::new(&size),
            input_object: Rc::clone(&gray),
            front: Rc::new(RefCell::new(Vec::<IntPoint>::new())),
            narrow_bands: Vec::<IntPoint>::new(),
            normals: Vec::<DoublePoint>::new(),
            inside_estimator_for_space_without_edge: InsideEstimator::from_grid(
                Grid::create_space_without_edge(Rc::clone(&size)),
            ),
            inside_estimator_for_space_with_edge: InsideEstimator::from_grid(
                Grid::create_space_with_edge(Rc::clone(&size)),
            ),
            inside_estimator_for_initial_front: InsideEstimator::new(),
            total_speed: 0.0,
            stopping_condition: StoppingCondition::new(),
            zero_count: 0,
            distance_map_generator: DistanceMapGenerator::new(
                parameters.wband,
                Rc::clone(&indexer),
                RefCell::clone(&statuses),
            ),
            curvature_generator: CurvatureGenerator::new(Rc::clone(&indexer), RefCell::clone(&phi)),
        }
    }

    pub fn get_speed(&self) -> &Vec<f64> {
        &self.speed
    }

    pub fn initialize_narrow_band(&mut self) {
        self.narrow_bands.clear();
        self.set_speed_function(true);
    }

    pub fn calculate_speed_factors(&mut self) {
        self.speed_factor.calculate_all(&self.size);
    }

    pub fn initailze_distance_map(&mut self) {
        self.distance_map_generator.create_distance_map();
    }

    pub fn get_size(&self) -> Rc<SpaceSize> {
        Rc::clone(&self.size)
    }

    pub fn initialize_along_front(&mut self, initial_front: &InitialFront) {
        self.front.borrow_mut().clear();
        self.normals.clear();
        self.initial_front.create_initial_front(initial_front);
        self.inside_estimator_for_initial_front
            .set_grid(&self.initial_front);

        self.initial_front.initialize_along_front(&self);
    }

    pub fn initialize_point_on_front(&self, p: &IntPoint) {
        let index = self.indexer.get(p) as usize;
        self.phi.borrow_mut()[index] = 0.0;
        self.statuses.borrow_mut()[index] = Status::Front;
        self.front.borrow_mut().push(p.clone());
    }

    pub fn initailze_over_all(&self, initial_front: &InitialFront) {
        self.grid_range.foreach_phi(&self);
    }

    pub fn get_phi(&self) -> RefCell<Vec<f64>> {
        RefCell::clone(&self.phi)
    }

    pub fn register_to_phi(&self, p: &IntPoint) {
        let index = self.indexer.get(&p);
        match self.statuses.borrow()[index as usize] {
            Status::Front => (),
            _ => {
                self.phi.borrow_mut()[index as usize] =
                    if self.inside_estimator_for_initial_front.is_inside(&p) {
                        -self.parameters.wband as f64
                    } else {
                        self.parameters.wband as f64
                    }
            }
        }
    }

    pub fn get_statuses(&self) -> RefCell<Vec<Status>> {
        RefCell::clone(&self.statuses)
    }

    pub fn get_front(&self) -> RefCell<Vec<IntPoint>> {
        RefCell::clone(&self.front)
    }

    pub fn get_grid(&self) -> &Grid {
        &self.initial_front
    }

    pub fn get_indexer(&self) -> Rc<Indexer> {
        Rc::clone(&self.indexer)
    }

    pub fn get_normals(&self) -> &Vec<DoublePoint> {
        &self.normals
    }

    pub fn print_verbose_description(&self) {
        // print something
    }

    fn clear_speed_within_narrow_band(&mut self, resets: bool) {
        for p in &self.narrow_bands {
            let index = self.indexer.get(p) as usize;
            self.speed[index] = 0.0;
            self.dphi.borrow_mut()[index] = 0.0;
            if resets {
                match self.statuses.borrow()[index] {
                    Status::Front => (),
                    _ => {
                        self.statuses.borrow_mut()[index] = Status::Farway;
                    }
                }
            }
        }
    }

    pub fn set_speed_on_front(&mut self) -> f64 {
        let mut fs = 0.0;
        self.zero_count = 0;

        for p in self.front.borrow().iter() {
            if self.inside_estimator_for_space_without_edge.is_inside(p) {
                let i = self.indexer.get(&p) as usize;
                let mut speed = self.speed_factor.get_value(p);
                let kappa = self.curvature_generator.generate(p);
                speed *= (self.parameters.constant_speed - self.parameters.gain * kappa);
                if speed.abs() < self.parameters.speed_threshold {
                    speed = 0.0;
                    self.zero_count += 1;
                } else {
                    //
                }
                fs += speed.abs();
                self.speed[i] = speed;
            }
        }
        fs
    }

    fn copy_nearest_speed_to_narrow_band(&self, resets: bool) {
        let distance_map = self.distance_map_generator.get_distance_map();
        let mut is_considerable = Vec::<Vec<bool>>::new();
        is_considerable.reserve(self.front.borrow().len());

        for p in self.front.borrow().iter() {
            let a = self.distance_map_generator.select_labels(p);
            is_considerable.push(a);
        }

        //something to do
    }

    fn register_to_narrow_band(
        indexer: &Indexer,
        statuses: RefCell<Vec<Status>>,
        band: &mut Vec<IntPoint>,
        p: IntPoint,
    ) {
        let index = indexer.get(&p);
        match statuses.borrow()[index as usize] {
            Status::Farway => band.push(p),
            _ => (),
        }
    }

    pub fn set_speed_function(&mut self, resets: bool) -> bool {
        self.clear_speed_within_narrow_band(resets);
        self.total_speed = self.set_speed_on_front();
        self.copy_nearest_speed_to_narrow_band(resets);
        if resets {
            self.narrow_bands.clear();
            self.grid_range.foreach_band(
                &self.indexer,
                RefCell::clone(&self.statuses),
                &mut self.narrow_bands,
                Self::register_to_narrow_band,
            );
        }
        self.stopping_condition.add_total_speed(self.total_speed);
        self.stopping_condition.is_satisfied()
    }

    pub fn propagate_front(&mut self) {
        for p in &self.narrow_bands {
            if self.inside_estimator_for_space_without_edge.is_inside(p) {
                let index = self.indexer.get(p) as usize;
                let speed = self.speed[index];
                let mut upwind_scheme = 0.0;
                if speed > 0.0 {
                    upwind_scheme = self.upwind_scheme.calculate(p, Speed::Positive);
                } else if speed < 0.0 {
                    upwind_scheme = self.upwind_scheme.calculate(p, Speed::Negative);
                }
                self.dphi.borrow_mut()[index] = speed * upwind_scheme * self.parameters.time_step;
            }
        }

        for p in &self.narrow_bands {
            let index = self.indexer.get(p) as usize;
            self.phi.borrow_mut()[index] -= self.dphi.borrow()[index];
        }
    }

    pub fn calculate_normals(&mut self) {
        self.normals.clear();
        for p in self.front.borrow().iter() {
            let n = self.curvature_generator.calculate_normal(p);
            self.normals.push(n);
        }
    }

    pub fn create_labels(&mut self) -> bool {
        let mut resets = false;
        self.front.borrow_mut().clear();
        for p in &self.narrow_bands {
            if self.inside_estimator_for_space_without_edge.is_inside(p) {
                let index = self.indexer.get(p);
            }
        }
        return resets;
    }

    pub fn get_input_object(&self) -> RefCell<Vec<u8>> {
        RefCell::clone(&self.input_object)
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
    DistanceMap2d,
    InitialFront2d,
    Grid2d,
    InsideEstimator2d,
    CurvatureGenerator2d,
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
    DistanceMap3d,
    InitialFront3d,
    Grid3d,
    InsideEstimator3d,
    CurvatureGenerator3d,
>;
