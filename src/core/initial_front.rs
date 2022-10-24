use crate::core::dimension_types as dim;
use crate::core::point::IntPoint;
pub struct InitialFront<const D: usize> {
    pub vertices: [IntPoint<D>; 2],
}
