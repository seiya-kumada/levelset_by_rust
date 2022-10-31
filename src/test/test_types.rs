use crate::core::types;

#[cfg(test)]
mod tests {

    use crate::core::types::Type;

    use super::*;
    #[test]
    fn test_types() {
        let a = std::any::type_name::<types::Grid<types::TwoDim>>();
        assert_eq!(a, "levelset_by_rust::core::grid::Grid2d");

        let a = std::any::type_name::<types::Grid<types::ThreeDim>>();
        assert_eq!(a, "levelset_by_rust::core::grid::Grid3d");

        let a = std::any::type_name::<types::SpaceSize<types::TwoDim>>();
        assert_eq!(a, "levelset_by_rust::core::space_size::SpaceSize2d");

        let a = std::any::type_name::<types::SpaceSize<types::ThreeDim>>();
        assert_eq!(a, "levelset_by_rust::core::space_size::SpaceSize3d");

        let a = std::any::type_name::<types::Indexer<types::TwoDim>>();
        assert_eq!(a, "levelset_by_rust::core::indexer::Indexer2d");

        let a = std::any::type_name::<types::Indexer<types::ThreeDim>>();
        assert_eq!(a, "levelset_by_rust::core::indexer::Indexer3d");

        let a = std::any::type_name::<types::IntPoint<types::TwoDim>>();
        assert_eq!(a, "levelset_by_rust::core::point::Point2d<i32>");

        let a = std::any::type_name::<types::IntPoint<types::ThreeDim>>();
        assert_eq!(a, "levelset_by_rust::core::point::Point3d<i32>");

        let a = std::any::type_name::<types::DoublePoint<types::TwoDim>>();
        assert_eq!(a, "levelset_by_rust::core::point::Point2d<f64>");

        let a = std::any::type_name::<types::DoublePoint<types::ThreeDim>>();
        assert_eq!(a, "levelset_by_rust::core::point::Point3d<f64>");

        let a = std::any::type_name::<types::Position<types::TwoDim>>();
        assert_eq!(a, "levelset_by_rust::core::position::Position2d");

        let a = std::any::type_name::<types::Position<types::ThreeDim>>();
        assert_eq!(a, "levelset_by_rust::core::position::Position3d");

        let space_size = types::SpaceSize::<types::TwoDim>::new(1, 2);
        let indexer = types::Indexer::<types::TwoDim>::new(&space_size);
        let p = types::IntPoint::<types::TwoDim>::new(1, 2);
        let q = indexer.get(&p);

        let r = types::TwoDim::make_position(&p, &indexer);
        assert_eq!(r.left, 2);
        assert_eq!(r.right, 4);
        assert_eq!(r.me, 3);
        assert_eq!(r.top, 2);
        assert_eq!(r.bottom, 4);

        let space_size = types::SpaceSize::<types::ThreeDim>::new(1, 1, 1);
        let indexer = types::Indexer::<types::ThreeDim>::new(&space_size);
        let p = types::IntPoint::<types::ThreeDim>::new(1, 1, 1);
        let q = indexer.get(&p);

        let r = types::ThreeDim::make_position(&p, &indexer);
        assert_eq!(r.left, 2);
        assert_eq!(r.right, 4);
        assert_eq!(r.me, 3);
        assert_eq!(r.top, 2);
        assert_eq!(r.bottom, 4);
        assert_eq!(r.front, 2);
        assert_eq!(r.back, 4);
    }
}