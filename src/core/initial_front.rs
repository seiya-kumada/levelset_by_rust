use crate::core::dimension_types as dim;
use crate::core::point::{NumDim, Point};
pub struct InitialFront<T, const D: usize> {
    pub vertices: [Point<NumDim<T, D>>; 2],
}
