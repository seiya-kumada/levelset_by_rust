use crate::core::grid::{Grid2d, Grid3d, GridMethod};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d, SpaceSizeMethod};
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_space_without_edge_2d() {
        let size = Rc::new(SpaceSize2d::new(101, 143));
        let grid = Grid2d::create_space_without_edge(Rc::clone(&size));

        assert!(grid.left == 0);
        assert!(grid.right == size.width - 1);
        assert!(grid.top == 0);
        assert!(grid.bottom == size.height - 1);
    }
    #[test]
    fn create_space_without_edge_3d() {
        let size = Rc::new(SpaceSize3d::new(101, 143, 3));
        let grid = Grid3d::create_space_without_edge(Rc::clone(&size));

        assert!(grid.left == 0);
        assert!(grid.right == size.width - 1);
        assert!(grid.top == 0);
        assert!(grid.bottom == size.height - 1);
        assert!(grid.front == 0);
        assert!(grid.back == size.depth - 1);
    }
}
