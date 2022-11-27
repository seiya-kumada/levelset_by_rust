use super::types::InitialFront;
use super::{types::SpeedFactor, upwind_scheme};
use crate::core::parameters::Parameters;
use crate::core::status::Status;
use crate::core::stopping_condition::StoppingCondition;
use crate::core::types::{
    DoublePoint, Front, Grid, GridRange, Indexer, InsideEstimator, IntPoint, SpaceSize, Type,
    UpwindScheme,
};
use std::rc::Rc;

pub struct LevelSetMethod<D: Type> {
    /// input parameters
    parameters: Parameters,

    /// size of the input image/3Dmodel
    size: Rc<SpaceSize<D>>,

    /// accessor of the array
    indexer: Rc<Indexer<D>>,

    /// input front(zero-levelset)
    initial_front: Grid<D>,

    /// auxiliary function
    phi: Rc<Vec<f64>>,

    /// deviation of auxiliary function
    dphi: Rc<Vec<f64>>,

    /// velocity function
    speed: Vec<f64>,

    /// current statuses
    statuses: Vec<Status>,

    /// front
    front: Vec<IntPoint<D>>,

    /// normals
    normals: Vec<DoublePoint<D>>,

    /// narrow band
    narrow_bands: Vec<IntPoint<D>>,

    /// input image(gray image)
    input_object: Rc<Vec<u8>>,

    upwind_scheme: UpwindScheme<D>,
    speed_factor: SpeedFactor<D>,
    grid_range: GridRange<D>,

    inside_estimator_of_space_without_edge: InsideEstimator<D>,
    inside_estimator_of_space_with_edge: InsideEstimator<D>,
    inside_estimator_of_initial_front: InsideEstimator<D>,

    total_speed: f64,

    stopping_condition: StoppingCondition,
    zero_count: i32,
}

impl<D: Type> LevelSetMethod<D> {
    pub fn new(
        parameters: Parameters,
        size: Rc<SpaceSize<D>>,
        gray: Rc<Vec<u8>>,
        initial_front: Grid<D>,
    ) -> Self {
        let indexer = Rc::new(D::make_indexer(&size));
        let phi = Rc::new(vec![0.0; D::get_total(&size)]);
        Self {
            parameters,
            size: Rc::clone(&size),
            indexer: Rc::clone(&indexer),
            initial_front,
            phi: Rc::clone(&phi),
            dphi: Rc::new(vec![0.0; D::get_total(&size)]),
            speed: vec![0.0; D::get_total(&size)],
            statuses: vec![Status::Farway; D::get_total(&size)],
            upwind_scheme: D::make_upwind_scheme(Rc::clone(&indexer), Rc::clone(&phi)),
            speed_factor: D::make_speed_factor(Rc::clone(&indexer), Rc::clone(&gray)),
            grid_range: D::make_grid_range(&size),
            input_object: Rc::clone(&gray),
            front: D::make_int_point_vec(),
            narrow_bands: D::make_int_point_vec(),
            normals: D::make_double_point_vec(),
            inside_estimator_of_space_without_edge: D::create_space_without_edge(Rc::clone(&size)),
            inside_estimator_of_space_with_edge: D::create_space_with_edge(Rc::clone(&size)),
            inside_estimator_of_initial_front: D::initialize_inside_estimator(),
            total_speed: 0.0,
            stopping_condition: StoppingCondition::new(),
            zero_count: 0,
        }
    }

    pub fn initialize_narrow_band(&mut self) {
        self.narrow_bands.clear();
        self.set_speed_function(true);
    }

    fn calculate_speed_factors(&mut self) {
        D::calculate_all(&mut self.speed_factor, &self.size);
    }

    fn initailze_distance_map() {}

    fn clear_speed_within_narrow_band(&self, reset: bool) {}

    fn set_speed_on_front(&mut self) -> f64 {
        let fs = 0.0;
        for p in &self.front {
            if D::is_inside(&self.inside_estimator_of_space_without_edge, p) {
                let i = D::get_index(&self.indexer, p) as usize;
                let speed_factor = D::get_speed_factor(&self.speed_factor, p);
                self.speed[i] = speed_factor.clone();
            }
        }
        0.0
    }

    fn copy_nearest_speed_to_narrow_band(&self, resets: bool) {}

    fn register_to_narrow_band(
        indexer: &Indexer<D>,
        statuses: &Vec<Status>,
        band: &mut Vec<IntPoint<D>>,
        p: IntPoint<D>,
    ) {
        let index = D::get_index(indexer, &p);
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
            D::loop_in_grid_range(
                &self.grid_range,
                &self.indexer,
                &self.statuses,
                &mut self.narrow_bands,
                Self::register_to_narrow_band,
            );
        }
        self.stopping_condition.add_total_speed(self.total_speed);
        self.stopping_condition.is_satisfied()
    }

    pub fn initialize_along_front(&mut self, initial_front: &InitialFront<D>) {
        self.front.clear();
        self.normals.clear();
        D::create_initial_front(initial_front, &mut self.initial_front);
        D::set_grid(
            &self.initial_front,
            &mut self.inside_estimator_of_initial_front,
        );
    }
}
