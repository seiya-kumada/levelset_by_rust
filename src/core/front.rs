use crate::core::types::{IntPoint, ThreeDim, TwoDim, Type};

pub type Front<D> = Vec<IntPoint<D>>;
pub type Front2d = Front<TwoDim>;
pub type Front3d = Front<ThreeDim>;
