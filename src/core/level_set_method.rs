use crate::core::dimension_types::Dimension;
use crate::core::parameters::Parameters;
use crate::core::space_size::SpaceSize;

struct LevelSetMethod<D: Dimension> {
    /// input parameters
    params: Parameters,

    /// size of the input image/3Dmodel
    size: SpaceSize<D>,
}
