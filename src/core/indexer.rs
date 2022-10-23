use crate::core::dimension_types::{Dimension, ThreeDimension, TwoDimension};
use crate::core::space_size::SpaceSize;
pub struct Indexer<D: Dimension> {
    size: SpaceSize<D>,
}

impl Indexer<TwoDimension> {
    pub fn new(size: SpaceSize<TwoDimension>) -> Self {
        Self { size }
    }
}
