use crate::core::differential as df;
use crate::core::types::{DifferentialU8, Indexer, Type};
use std::rc::Rc;
pub struct SpeedFactor<D: Type> {
    indexer: Rc<Indexer<D>>,
    differential: DifferentialU8<D>,
}

impl<D: Type> SpeedFactor<D> {
    fn new(indexer: Rc<Indexer<D>>, gray: Rc<Vec<u8>>) -> Self {
        Self {
            indexer: Rc::clone(&indexer),
            differential: D::make_differential_u8(Rc::clone(&indexer), Rc::clone(&gray)),
        }
    }
}
