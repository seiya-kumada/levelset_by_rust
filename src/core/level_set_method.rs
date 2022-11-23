use super::types::InitialFront;
use super::{types::SpeedFactor, upwind_scheme};
use crate::core::parameters::Parameters;
use crate::core::status::Status;
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
    is_inside_initial_front: InsideEstimator<D>,
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
            is_inside_initial_front: D::initialize_inside_estimator(),
        }
    }

    pub fn initialize_along_front(&mut self, initial_front: &InitialFront<D>) {
        self.front.clear();
        self.normals.clear();
        D::create_initial_front(initial_front, &mut self.initial_front);
        D::set_grid(&self.initial_front, &mut self.is_inside_initial_front);
    }
}
