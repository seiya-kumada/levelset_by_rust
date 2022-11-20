use crate::core::front::Front;
use crate::core::parameters::Parameters;
use crate::core::status::Status;
use crate::core::types::{DoublePoint, Grid, Indexer, IntPoint, SpaceSize, Type};
//use crate::core::upwind_scheme::UpwindScheme;

pub struct LevelSetMethod<D: Type> {
    /// input parameters
    params: Parameters,

    /// size of the input image/3Dmodel
    pub size: SpaceSize<D>,

    /// accessor of the array
    pub indexer: Indexer<D>,

    /// input front(zero-levelset)
    pub initial_front: Grid<D>,

    /// auxiliary function
    phi: Vec<f64>,

    /// deviation of auxiliary function
    dphi: Vec<f64>,

    /// velocity function
    speed: Vec<f64>,

    /// current statuses
    statuses: Vec<Status>,

    /// front
    front: Front<D>,

    /// normals
    normals: Vec<DoublePoint<D>>,

    /// narrow band
    narrow_bands: Vec<IntPoint<D>>,

    /// input image(gray image)
    input_object: Vec<u8>,
    //upwind_scheme: UpwindScheme<D>,
}
