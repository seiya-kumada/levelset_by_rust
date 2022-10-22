use crate::core::dimension_types::Dimension;
use crate::core::parameters::Parameters;
use crate::core::space_size::SpaceSize;

struct LevelSetMethod<D: Dimension> {
    params: Parameters,
    size: SpaceSize<D>,
}
