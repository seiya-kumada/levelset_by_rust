//use crate::core::front::{Front2d, Front3d};
//use crate::core::grid::{Grid2d, Grid3d};
//use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::parameters::Parameters;
//use crate::core::point::{Point2d, Point3d};
//use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::status::Status;
//use crate::core::upwind_scheme::{UpwindScheme2d, UpwindScheme3d};
use super::{types::SpeedFactor, upwind_scheme};
use crate::core::types::{
    DoublePoint, Front, Grid, GridRange, Indexer, IntPoint, SpaceSize, Type, UpwindScheme,
};
use std::rc::Rc;

pub struct LevelSetMethod<D: Type> {
    /// input parameters
    parameters: Parameters,

    /// size of the input image/3Dmodel
    size: SpaceSize<D>,

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
}

impl<D: Type> LevelSetMethod<D> {
    pub fn new(
        parameters: Parameters,
        size: SpaceSize<D>,
        gray: Rc<Vec<u8>>,
        initial_front: Grid<D>,
    ) -> Self {
        let indexer = Rc::new(D::make_indexer(&size));
        let phi = Rc::new(vec![0.0; D::get_total(&size)]);
        Self {
            parameters: parameters,
            size: size,
            indexer: Rc::clone(&indexer),
            initial_front: initial_front,
            phi: Rc::new(vec![0.0; D::get_total(&size)]),
            dphi: Rc::new(vec![0.0; D::get_total(&size)]),
            speed: vec![0.0; D::get_total(&size)],
            statuses: vec![Status::Farway; D::get_total(&size)],
            upwind_scheme: D::make_upwind_scheme(Rc::clone(&indexer), phi),
            speed_factor: D::make_speed_factor(Rc::clone(&indexer), gray),
            grid_range: D::make_grid_range(&size),
            input_object: Rc::clone(&gray),
            front: D::make_int_point_vec(),
            narrow_bands: D::make_int_point_vec(),
            normals: D::make_double_point_vec(),
        }
    }
}
