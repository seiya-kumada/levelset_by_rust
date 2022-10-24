//use crate::core::dimension_types::Dimension;
use crate::core::parameters::Parameters;
//use crate::core::space_size::SpaceSize;

struct LevelSetMethod<SpaceSize, Indexer, Grid> {
    /// input parameters
    params: Parameters,

    /// size of the input image/3Dmodel
    size: SpaceSize,

    /// accessor of the array
    indexer: Indexer,

    /// input front(zero-levelset)
    initial_front: Grid,

    /// auxiliary function
    phi: Vec<f64>,

    /// deviation of auxiliary function
    dphi: Vec<f64>,

    /// velocity function
    speed: Vec<f64>,
}
