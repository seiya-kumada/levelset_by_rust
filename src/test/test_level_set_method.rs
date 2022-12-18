use crate::core::grid::Grid3d;
use crate::core::initial_front::InitialFront3d;
use crate::core::level_set_method::LevelSetMethod3d;
use crate::core::parameters::Parameters;
use crate::core::point::Point3d;
use crate::core::space_size::SpaceSize3d;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use crate::core::{initial_front, parameters};

    use super::*;
    #[test]
    fn front_3d() {
        let mut params = Parameters::new();
        params.wband = 3;
        let mut initial_front = InitialFront3d::new();

        initial_front.vertices[0] = Point3d::<i32>::new(0, 0, 0);
        initial_front.vertices[1] = Point3d::<i32>::new(2, 2, 2);

        let size = Rc::new(SpaceSize3d::new(3, 3, 3));
        let gray = RefCell::new(vec![0u8]);
        let grid = Grid3d::new();
        let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), RefCell::clone(&gray), grid);
        lsm.initialize_along_front(&initial_front);

        let front = lsm.get_front();
        assert_eq!(front.len(), 26);
    }
}
