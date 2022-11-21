use crate::core::front::{Front2d, Front3d};
use crate::core::grid::{Grid2d, Grid3d};
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::parameters::Parameters;
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::status::Status;
use crate::core::upwind_scheme::{UpwindScheme2d, UpwindScheme3d};

pub struct LevelSetMethod2d {
    /// input parameters
    params: Parameters,

    /// size of the input image/3Dmodel
    pub size: SpaceSize2d,

    /// accessor of the array
    pub indexer: Indexer2d,

    /// input front(zero-levelset)
    pub initial_front: Grid2d,

    /// auxiliary function
    phi: Vec<f64>,

    /// deviation of auxiliary function
    dphi: Vec<f64>,

    /// velocity function
    speed: Vec<f64>,

    /// current statuses
    statuses: Vec<Status>,

    /// front
    front: Front2d,

    /// normals
    normals: Vec<Point2d<f64>>,

    /// narrow band
    narrow_bands: Vec<Point2d<i32>>,

    /// input image(gray image)
    input_object: Vec<u8>,

    upwind_scheme: UpwindScheme2d,
}
