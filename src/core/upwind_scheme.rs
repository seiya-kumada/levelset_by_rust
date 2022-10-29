use std::marker::PhantomData;

use super::types::IntPoint;
use crate::core::indexer;
use crate::core::position as po;
//use crate::core::position::PositionTrait;
use crate::core::types;
use crate::core::types::{Indexer, Position, Type};
pub struct UpwindScheme<D: Type> {
    //indexer: &'a Indexer<D>,
    //phi: &'a Vec<f64>,
    //phantom: PhantomData<T>,
    pub position: Position<D>,
    ///// upwind shceme
    //fdxm: f64,
    //fdxp: f64,
    //fdym: f64,
    //fdyp: f64,
    //fdzm: f64,
    //fdzp: f64,

    ///// central difference
    //dx: f64,
    //dy: f64,
    //dz: f64,

    //central_difference: f64,
    //upwind_scheme_difference: f64,
}

impl<D: Type> UpwindScheme<D> {
    pub fn new(p: &IntPoint<D>, indexer: &Indexer<D>) -> Self {
        Self {
            position: D::make_position(p, indexer),
        }
    }
}
