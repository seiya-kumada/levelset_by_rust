use opencv::prelude::AlgorithmTrait;

use crate::core::curvature_generator::{
    CurvatureGenerator2d, CurvatureGenerator3d, CurvatureGeneratorMethod,
};
use crate::core::distance_map_generator::{
    DistanceMap2d, DistanceMap3d, DistanceMapGenerator2d, DistanceMapGenerator3d,
    DistanceMapGeneratorMethod, DistanceMapMethod, PointInfo2d, PointInfo3d, PointInfoMethod,
};
use crate::core::grid::{Grid2d, Grid3d, GridMethod};
use crate::core::grid_range::{GridRange2d, GridRange3d, GridRangeMethod};
use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::initial_front::{InitialFront2d, InitialFront3d};
use crate::core::inside_estimator::{InsideEstimator2d, InsideEstimator3d, InsideEstimatorMethod};
use crate::core::parameters::Parameters;
use crate::core::point::{Point2d, Point3d, PointMethod};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d, SpaceSizeMethod};
use crate::core::speed::Speed;
use crate::core::speed_factor::{SpeedFactor2d, SpeedFactor3d, SpeedFactorMethod};
use crate::core::status::Status;
use crate::core::stopping_condition::StoppingCondition;
use crate::core::upwind_scheme::{UpwindScheme2d, UpwindScheme3d, UpwindSchemeMethod};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use super::upwind;

pub struct LevelSetMethod<
    SpaceSize,
    Indexer,
    UpwindScheme,
    SpeedFactor,
    GridRange,
    IntPoint,
    PointInfo,
    DoublePoint,
    DistanceMapGenerator,
    DistanceMap,
    InitialFront,
    Grid,
    InsideEstimator,
    CurvatureGenerator,
> where
    IntPoint: Copy + PointMethod<Type = IntPoint>,
    PointInfo: PointInfoMethod<IntPoint>,
    SpaceSize: SpaceSizeMethod,
    Indexer: IndexerMethod<SpaceSize, IntPoint>,
    UpwindScheme: UpwindSchemeMethod<Indexer, IntPoint>,
    SpeedFactor: SpeedFactorMethod<Indexer, IntPoint, SpaceSize>,
    GridRange: GridRangeMethod<SpaceSize, Indexer, IntPoint, Self>,
    DistanceMapGenerator: DistanceMapGeneratorMethod<Indexer, DistanceMap, IntPoint, Self>,
    Grid: GridMethod<InitialFront, SpaceSize, Self, IntPoint>,
    InsideEstimator: InsideEstimatorMethod<Grid, IntPoint>,
    CurvatureGenerator: CurvatureGeneratorMethod<Indexer, IntPoint, DoublePoint>,
{
    phantom_initial_front: PhantomData<InitialFront>,
    phantom_distance_map: PhantomData<DistanceMap>,
    phantom_point_info: PhantomData<PointInfo>,

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
    speed: Rc<RefCell<Vec<f64>>>,

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

    upper_distance: i32,
}
impl<
        SpaceSize,
        Indexer,
        UpwindScheme,
        SpeedFactor,
        GridRange,
        IntPoint,
        PointInfo,
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
        PointInfo,
        DoublePoint,
        DistanceMapGenerator,
        DistanceMap,
        InitialFront,
        Grid,
        InsideEstimator,
        CurvatureGenerator,
    >
where
    IntPoint: Copy + PointMethod<Type = IntPoint>,
    PointInfo: PointInfoMethod<IntPoint>,
    SpaceSize: SpaceSizeMethod,
    Indexer: IndexerMethod<SpaceSize, IntPoint>,
    UpwindScheme: UpwindSchemeMethod<Indexer, IntPoint>,
    SpeedFactor: SpeedFactorMethod<Indexer, IntPoint, SpaceSize>,
    GridRange: GridRangeMethod<SpaceSize, Indexer, IntPoint, Self>,
    DistanceMapGenerator: DistanceMapGeneratorMethod<Indexer, DistanceMap, IntPoint, Self>,
    DistanceMap: DistanceMapMethod<PointInfo>,
    Grid: GridMethod<InitialFront, SpaceSize, Self, IntPoint>,
    InsideEstimator: InsideEstimatorMethod<Grid, IntPoint>,
    CurvatureGenerator: CurvatureGeneratorMethod<Indexer, IntPoint, DoublePoint>,
{
    pub fn new(parameters: Parameters, size: Rc<SpaceSize>, gray: Rc<RefCell<Vec<u8>>>) -> Self {
        let statuses = Rc::new(RefCell::new(vec![Status::Farway; size.get_total()]));
        let indexer = Rc::new(Indexer::new(&size));
        let phi = Rc::new(RefCell::new(vec![0.0; size.get_total()]));
        let initial_front = Grid::new();
        Self {
            phantom_initial_front: PhantomData,
            phantom_distance_map: PhantomData,
            phantom_point_info: PhantomData,
            parameters: parameters.clone(),
            size: Rc::clone(&size),
            indexer: Rc::clone(&indexer),
            initial_front,
            phi: Rc::clone(&phi),
            dphi: Rc::new(RefCell::new(vec![0.0; size.get_total()])),
            speed: Rc::new(RefCell::new(vec![0.0; size.get_total()])),
            statuses: Rc::clone(&statuses),
            upwind_scheme: UpwindScheme::new(Rc::clone(&indexer), Rc::clone(&phi)),
            speed_factor: SpeedFactor::new(Rc::clone(&indexer), Rc::clone(&gray)),
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
                Rc::clone(&statuses),
            ),
            curvature_generator: CurvatureGenerator::new(Rc::clone(&indexer), Rc::clone(&phi)),
            upper_distance: (parameters.wband - parameters.wreset).pow(2),
        }
    }

    pub fn get_speed(&self) -> Rc<RefCell<Vec<f64>>> {
        Rc::clone(&self.speed)
    }

    pub fn get_grid_range(&self) -> &GridRange {
        &self.grid_range
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

    pub fn initialize_over_all(&self, initial_front: &InitialFront) {
        self.grid_range.foreach_phi(&self);
    }

    pub fn get_phi(&self) -> Rc<RefCell<Vec<f64>>> {
        Rc::clone(&self.phi)
    }

    pub fn initialize_distance_map(&mut self) {
        self.distance_map_generator.create_distance_map();
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

    pub fn get_statuses(&self) -> Rc<RefCell<Vec<Status>>> {
        Rc::clone(&self.statuses)
    }

    pub fn get_front(&self) -> Rc<RefCell<Vec<IntPoint>>> {
        Rc::clone(&self.front)
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

    pub fn get_dphi(&self) -> Rc<RefCell<Vec<f64>>> {
        Rc::clone(&self.dphi)
    }

    pub fn print_verbose_description(&self) {
        // print something
    }

    pub fn get_narrow_bands(&mut self) -> &mut Vec<IntPoint> {
        &mut self.narrow_bands
    }

    pub fn clear_speed_within_narrow_band(&mut self, resets: bool) {
        for p in &self.narrow_bands {
            let index = self.indexer.get(p) as usize;
            self.speed.borrow_mut()[index] = 0.0;
            self.dphi.borrow_mut()[index] = 0.0;
            let status = self.statuses.borrow()[index];
            if resets {
                match status {
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
                //p.print();
                speed *= (self.parameters.constant_speed - self.parameters.gain * kappa);
                //println!("s:{}", speed);
                if speed.abs() < self.parameters.speed_threshold {
                    speed = 0.0;
                    self.zero_count += 1;
                } else {
                    //
                }
                fs += speed.abs();
                self.speed.borrow_mut()[i] = speed;
            }
        }
        fs
    }

    pub fn copy_nearest_speed_to_narrow_band(&self, resets: bool) {
        let mut is_considerable = Vec::<Vec<bool>>::new();
        is_considerable.reserve(self.front.borrow().len());

        for p in self.front.borrow().iter() {
            let a = self.distance_map_generator.select_labels(p);
            is_considerable.push(a);
        }

        self.distance_map_generator
            .foreach(&self, resets, &is_considerable);
    }

    pub fn copy_nearest_speed_to_narrow_band_core(
        &self,
        resets: bool,
        is_considerable: &Vec<Vec<bool>>,
        distance: &i32,
        //range: &Vec<PointInfo>,
    ) {
        let mut k = 0usize;
        let distance_map = self.distance_map_generator.get_distance_map();
        let range = distance_map.get_vec(distance);

        //for p in self.front.borrow().iter() {
        //    let index = self.indexer.get(p) as usize;
        //    if resets {
        //        self.phi.borrow_mut()[index] = 0.0;
        //    }
        //    let center_speed = self.speed.borrow()[index];
        //    self.copy_nearest_speed_to_narrow_band_core_core(
        //        &is_considerable[k],
        //        range,
        //        p,
        //        resets,
        //        distance,
        //        center_speed,
        //    );
        //    k += 1;
        //}
    }

    fn copy_nearest_speed_to_narrow_band_core_core(
        &self,
        is_considerable: &Vec<bool>,
        range: &Vec<PointInfo>,
        center: &IntPoint,
        resets: bool,
        distance: &i32,
        center_speed: f64,
    ) {
        for info in range {
            if is_considerable[info.get_label()] {
                let p = center.add(info.get_point());
                if self.inside_estimator_for_space_with_edge.is_inside(&p) {
                    let index = self.indexer.get(&p) as usize;
                    if self.statuses.borrow()[index] != Status::Front {
                        if resets {
                            if *distance > self.upper_distance {
                                self.statuses.borrow_mut()[index] = Status::ResetBand;
                            } else {
                                self.statuses.borrow_mut()[index] = Status::Band;
                            }
                        }
                        self.speed.borrow_mut()[index] = center_speed;
                    }
                }
            }
        }
    }

    pub fn register_to_narrow_band(
        indexer: &Indexer,
        statuses: Rc<RefCell<Vec<Status>>>,
        band: &mut Vec<IntPoint>,
        p: IntPoint,
    ) {
        let index = indexer.get(&p);
        match statuses.borrow()[index as usize] {
            Status::Farway => (),
            _ => band.push(p),
        }
    }

    pub fn see_statues(&self, log: &str) {
        use std::collections::HashMap;
        let mut status_map: HashMap<Status, usize> = HashMap::new();
        status_map.insert(Status::Farway, 0);
        status_map.insert(Status::Band, 1);
        status_map.insert(Status::ResetBand, 2);
        status_map.insert(Status::Front, 3);

        println!("{} {}", log, status_map[&self.statuses.borrow()[0]]);
        println!("{} {}", log, status_map[&self.statuses.borrow()[1]]);
        println!("{} {}", log, status_map[&self.statuses.borrow()[2]]);
        println!("{} {}", log, status_map[&self.statuses.borrow()[3]]);
    }
    pub fn register_to_narrow_band_(&mut self) {
        self.grid_range.foreach_band(
            &self.indexer,
            Rc::clone(&self.statuses),
            &mut self.narrow_bands,
            Self::register_to_narrow_band,
        )
    }

    pub fn set_speed_function(&mut self, resets: bool) -> bool {
        self.clear_speed_within_narrow_band(resets);
        self.total_speed = self.set_speed_on_front();
        self.copy_nearest_speed_to_narrow_band(resets);
        if resets {
            self.narrow_bands.clear();
            self.grid_range.foreach_band(
                &self.indexer,
                Rc::clone(&self.statuses),
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
                let speed = self.speed.borrow()[index];
                let mut upwind_scheme = 0.0;
                if speed > 0.0 {
                    upwind_scheme = self.upwind_scheme.calculate(p, Speed::Positive);
                } else if speed < 0.0 {
                    upwind_scheme = self.upwind_scheme.calculate(p, Speed::Negative);
                } else {
                    upwind_scheme = 0.0;
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

    pub fn get_input_object(&self) -> Rc<RefCell<Vec<u8>>> {
        Rc::clone(&self.input_object)
    }
}

pub type LevelSetMethod2d = LevelSetMethod<
    SpaceSize2d,
    Indexer2d,
    UpwindScheme2d,
    SpeedFactor2d,
    GridRange2d,
    Point2d<i32>,
    PointInfo2d,
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
    PointInfo3d,
    Point3d<f64>,
    DistanceMapGenerator3d,
    DistanceMap3d,
    InitialFront3d,
    Grid3d,
    InsideEstimator3d,
    CurvatureGenerator3d,
>;
