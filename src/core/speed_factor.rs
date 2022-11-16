//use crate::core::differential as df;
//use crate::core::differential::{DifferentialT, DifferentialU8HH};
//use crate::core::types::{DifferentialU8, Indexer, IntPoint, SpaceSize, Type};
use crate::core::dim;
//use crate::core::indexer::Indexer;
use crate::core::indexer::IndexerT;
use crate::core::point::Point;
use crate::core::point::PointT;
use num_traits::Num;
use std::rc::Rc;
//pub trait Type<T: Num>: PointT<T> + IndexerT {}

//pub struct Hoge<T: Num, D: Type<T>> {
//    point: Point<D, T>,
//    indexer: Indexer<D>,
//}

//impl<T: Num, D: Type<T>> Hoge<T, D> {
//    fn hoge(indexer: Rc<Indexer<D>>, p: &Point<D, T>) {}
//}
// https://amagramming.netlify.app/2019/03/25/rust-combine-traits
//pub struct SpeedFactor<D: AllType> {
//    indexer: Rc<Indexer<D>>,
//    differential: DifferentialU8HH<D>,
//    factors: Vec<f64>,
//}

//impl<D: AllType> SpeedFactor<D> {
//    pub fn new(indexer: Rc<Indexer<D>>, gray: Rc<Vec<u8>>) -> Self {
//        Self {
//            indexer: Rc::clone(&indexer),
//            differential: D::make_differential_u8(Rc::clone(&indexer), Rc::clone(&gray)),
//            factors: Vec::<f64>::new(),
//        }
//    }
//
//    pub fn calculate_all(&self, size: &SpaceSize<D>) {}
//    fn calculate(&mut self, p: &IntPoint<D>) -> f64 {
//        D::set_point(&mut self.differential, p);
//
//        1.0
//    }
//
//    pub fn get_value(&self, p: &IntPoint<D>) -> f64 {
//        self.factors[D::get_index(&self.indexer, p) as usize]
//    }
//}
