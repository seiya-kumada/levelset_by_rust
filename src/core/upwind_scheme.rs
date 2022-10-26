use crate::core::indexer;
use crate::core::types::{Indexer, Position, Type};

use super::types::IntPoint;
pub struct UpwindScheme<'a, D: Type> {
    indexer: &'a Indexer<D>,
    phi: &'a Vec<f64>,

    position: Position<D>,

    /// upwind shceme
    fdxm: f64,
    fdxp: f64,
    fdym: f64,
    fdyp: f64,
    fdzm: f64,
    fdzp: f64,

    /// central difference
    dx: f64,
    dy: f64,
    dz: f64,

    central_difference: f64,
    upwind_scheme_difference: f64,
}

impl<'a, D: Type> UpwindScheme<'a, D> {
    //fn new(indexer: &'a Indexer<D>, phi: &'a Vec<f64>) -> Self {
    //    Self {
    //        indexer: &indexer,
    //        phi: &phi,
    //        position: Position::<D>
    //        fdxm: 0.0,
    //        fdxp: 0.0,
    //        fdym: 0.0,
    //        fdyp: 0.0,
    //        fdzm: 0.0,
    //        fdzp: 0.0,
    //        dx: 0.0,
    //        dy: 0.0,
    //        dz: 0.0,
    //        central_difference: 0.0,
    //        upwind_scheme_difference: 0.0,
    //    }
    //}

    fn calculate(&self, p: &'a IntPoint<D>) {
        self.set_position(p);
    }

    fn set_position(&self, p: &'a IntPoint<D>) {
        //Position::<D>::new(&p, self.indexer);
    }
}
