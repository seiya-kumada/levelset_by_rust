use super::types::IntPoint;
use super::upwind;
use crate::core::indexer;
use crate::core::position as po;
use crate::core::speed;
use crate::core::speed as sp;
use crate::core::types;
use crate::core::types::{Indexer, Position, Type, Upwind};

pub struct UpwindScheme<D: Type> {
    pub position: Position<D>,

    /// upwind shceme
    pub upwind: Upwind<D>,
}

impl<D: Type> UpwindScheme<D> {
    pub fn new(p: &IntPoint<D>, indexer: &Indexer<D>, phi: &Vec<f64>, speed: &sp::Speed) -> Self {
        let position = D::make_position(p, indexer);
        let upwind = match speed {
            sp::Speed::Positive => D::make_upwind_with_positive_speed(&position, phi),
            sp::Speed::Negative => D::make_upwind_with_negative_speed(&position, phi),
        };
        Self { position, upwind }
    }
}
