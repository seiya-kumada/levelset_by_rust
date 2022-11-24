use crate::core::level_set_method::LevelSetMethod;
use crate::core::parameters::Parameters;
use crate::core::types::{Grid, Indexer, SpaceSize, ThreeDim, TwoDim};
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use crate::core::{initial_front, parameters};

    use super::*;
    #[test]
    fn level_set_method() {
        //    let g2 = Grid::<TwoDim>::new();
        //    let s2 = Rc::new(SpaceSize::<TwoDim>::new(1, 2));
        //    let g: Rc<Vec<u8>> = Rc::new(vec![1, 2, 3]);

        //    let p = Parameters {
        //        wband: 0,
        //        wreset: 0,
        //        time_step: 0.0,
        //        gain: 0.0,
        //        constant_speed: 0.0,
        //        speed_threshold: 0.0,
        //    };

        //    let a = Rc::clone(&g);
        //    let a: Rc<SpaceSize<TwoDim>> = Rc::clone(&s2);

        //    let a = LevelSetMethod::<TwoDim>::new(p, Rc::clone(&s2), Rc::clone(&g), g2);
    }
}
