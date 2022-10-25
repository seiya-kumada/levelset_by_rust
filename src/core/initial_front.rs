use crate::core::types::{IntPoint, Type};
pub struct InitialFront<D: Type> {
    pub vertices: [IntPoint<D>; 2],
}
