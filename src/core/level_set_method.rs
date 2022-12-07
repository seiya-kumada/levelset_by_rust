use crate::core::distance_map_generator::DistanceMapGeneratorMethod;
use crate::core::grid::GridMethod;
use crate::core::grid_range::GridRangeMethod;
use crate::core::indexer::IndexerMethod;
use crate::core::inside_estimator::InsideEstimatorMethod;
use crate::core::parameters::Parameters;
use crate::core::speed_factor::SpeedFactorMethod;
use crate::core::status::Status;
use crate::core::stopping_condition::StoppingCondition;
use crate::core::types;
use crate::core::upwind_scheme::New;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct LevelSetMethod<
    //SpaceSize,
    //Indexer: IndexerMethod<SpaceSize, IntPoint>,
    //UpwindScheme: New<Indexer>,
    //SpeedFactor: SpeedFactorMethod<Indexer, IntPoint, SpaceSize>,
    //GridRange: GridRangeMethod<SpaceSize, Indexer, IntPoint>,
    //IntPoint,
    //DoublePoint,
    //DistanceMapGenerator: DistanceMapGeneratorMethod<Indexer>,
    //InitialFront,
    //Grid: GridMethod<InitialFront, SpaceSize>,
    //InsideEstimator: InsideEstimatorMethod<Grid, IntPoint>,
    D: types::Type, //+ types::Method<
                    //    SpaceSize,
                    //    Indexer,
                    //    UpwindScheme,
                    //    SpeedFactor,
                    //    GridRange,
                    //    IntPoint,
                    //    DoublePoint,
                    //    DistanceMapGenerator,
                    //    InitialFront,
                    //    Grid,
                    //    InsideEstimator,
                    //>
> {
    /// input parameters
    parameters: Parameters,

    /// size of the input image/3Dmodel
    size: Rc<types::SpaceSize<D>>,

    /// accessor of the array
    indexer: Rc<types::Indexer<D>>,

    /// input front(zero-levelset)
    initial_front: types::Grid<D>,

    /// auxiliary function
    phi: Rc<Vec<f64>>,

    /// deviation of auxiliary function
    dphi: Rc<Vec<f64>>,

    /// velocity function
    speed: Vec<f64>,

    /// current statuses
    statuses: Rc<Vec<Status>>,

    /// front
    front: Vec<types::IntPoint<D>>,

    /// normals
    normals: Vec<types::DoublePoint<D>>,

    /// narrow band
    narrow_bands: Vec<types::IntPoint<D>>,

    /// input image(gray image)
    input_object: Rc<Vec<u8>>,

    upwind_scheme: types::UpwindScheme<D>,
    speed_factor: types::SpeedFactor<D>,
    grid_range: types::GridRange<D>,

    inside_estimator_of_space_without_edge: types::InsideEstimator<D>,
    inside_estimator_of_space_with_edge: types::InsideEstimator<D>,
    inside_estimator_of_initial_front: types::InsideEstimator<D>,

    total_speed: f64,

    stopping_condition: StoppingCondition,
    zero_count: i32,

    distance_map_generator: types::DistanceMapGenerator<D>,
}

//impl<D: types::Type> LevelSetMethod<D> {
//pub fn new(
//    parameters: Parameters,
//    size: Rc<SpaceSize<D>>,
//    gray: Rc<Vec<u8>>,
//    initial_front: Grid<D>,
//) -> Self {
//    let statuses = Rc::new(vec![Status::Farway; D::get_total(&size)]);
//    let indexer = Rc::new(D::make_indexer(&size));
//    let phi = Rc::new(vec![0.0; D::get_total(&size)]);
//    Self {
//        parameters: parameters.clone(),
//        size: Rc::clone(&size),
//        indexer: Rc::clone(&indexer),
//        initial_front,
//        phi: Rc::clone(&phi),
//        dphi: Rc::new(vec![0.0; D::get_total(&size)]),
//        speed: vec![0.0; D::get_total(&size)],
//        statuses: Rc::clone(&statuses),
//        upwind_scheme: D::make_upwind_scheme(Rc::clone(&indexer), Rc::clone(&phi)),
//        speed_factor: D::make_speed_factor(Rc::clone(&indexer), Rc::clone(&gray)),
//        grid_range: D::make_grid_range(&size),
//        input_object: Rc::clone(&gray),
//        front: D::make_int_point_vec(),
//        narrow_bands: D::make_int_point_vec(),
//        normals: D::make_double_point_vec(),
//        inside_estimator_of_space_without_edge: D::create_space_without_edge(Rc::clone(&size)),
//        inside_estimator_of_space_with_edge: D::create_space_with_edge(Rc::clone(&size)),
//        inside_estimator_of_initial_front: D::initialize_inside_estimator(),
//        total_speed: 0.0,
//        stopping_condition: StoppingCondition::new(),
//        zero_count: 0,
//        distance_map_generator: D::make_distance_map_generator(
//            parameters.wband,
//            Rc::clone(&indexer),
//            Rc::clone(&statuses),
//        ),
//    }
//}

//pub fn initialize_narrow_band(&mut self) {
//    self.narrow_bands.clear();
//    self.set_speed_function(true);
//}

//fn calculate_speed_factors(&mut self) {
//    D::calculate_all(&mut self.speed_factor, &self.size);
//}

//fn initailze_distance_map(&mut self) {
//    D::create_distance_map(&mut self.distance_map_generator);
//}

//fn get_size(&self) -> Rc<SpaceSize<D>> {
//    Rc::clone(&self.size)
//}

//pub fn initialize_along_front(&mut self, initial_front: &InitialFront<D>) {
//    self.front.clear();
//    self.normals.clear();
//    D::create_initial_front(initial_front, &mut self.initial_front);
//    D::set_grid(
//        &self.initial_front,
//        &mut self.inside_estimator_of_initial_front,
//    );
//}

//fn initailze_over_all(&self, initial_front: &InitialFront<D>) {
//    D::loop_in_grid_range_with_phi(
//        &self.grid_range,
//        &self.indexer,
//        &self.statuses,
//        &mut Rc::clone(&self.phi),
//        Self::register_to_phi,
//    );
//}
//fn register_to_phi(
//    indexer: &Indexer<D>,
//    statuses: &Vec<Status>,
//    phi: &mut Rc<Vec<f64>>,
//    p: IntPoint<D>,
//) {
//    let index = D::get_index(indexer, &p);
//    match statuses[index as usize] {
//        Status::Front => {
//            //phi[index as usize] = 1.0;
//        }
//        _ => (),
//    }
//}

//fn clear_speed_within_narrow_band(&self, reset: bool) {}

//fn set_speed_on_front(&mut self) -> f64 {
//    let fs = 0.0;
//    for p in &self.front {
//        if D::is_inside(&self.inside_estimator_of_space_without_edge, p) {
//            let i = D::get_index(&self.indexer, p) as usize;
//            let speed_factor = D::get_speed_factor(&self.speed_factor, p);
//            self.speed[i] = speed_factor.clone();
//        }
//    }
//    0.0
//}

//fn copy_nearest_speed_to_narrow_band(&self, resets: bool) {}

//fn register_to_narrow_band(
//    indexer: &Indexer<D>,
//    statuses: &Vec<Status>,
//    band: &mut Vec<IntPoint<D>>,
//    p: IntPoint<D>,
//) {
//    let index = D::get_index(indexer, &p);
//    match statuses[index as usize] {
//        Status::Farway => band.push(p),
//        _ => (),
//    }
//}

//fn set_speed_function(&mut self, resets: bool) -> bool {
//    self.clear_speed_within_narrow_band(resets);
//    self.total_speed = self.set_speed_on_front();
//    self.copy_nearest_speed_to_narrow_band(resets);
//    if resets {
//        self.narrow_bands.clear();
//        D::loop_in_grid_range(
//            &self.grid_range,
//            &self.indexer,
//            &self.statuses,
//            &mut self.narrow_bands,
//            Self::register_to_narrow_band,
//        );
//    }
//    self.stopping_condition.add_total_speed(self.total_speed);
//    self.stopping_condition.is_satisfied()
//}
//}
