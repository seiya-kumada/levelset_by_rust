//use crate::core::parameters::Parameters;
use crate::core::types;

pub struct LevelSetMethod<D: types::Type> {
    ///// input parameters
    //params: Parameters,
    /// size of the input image/3Dmodel
    pub size: types::SpaceSize<D>,

    /// accessor of the array
    pub indexer: types::Indexer<D>,

    /// input front(zero-levelset)
    pub initial_front: types::Grid<D>,
    ///// auxiliary function
    //phi: Vec<f64>,

    ///// deviation of auxiliary function
    //dphi: Vec<f64>,

    ///// velocity function
    //speed: Vec<f64>,
}
