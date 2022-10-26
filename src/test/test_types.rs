use crate::core::types;

#[cfg(test)]
mod tests {

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

        types::SpaceSize::<types::TwoDim>::new(1, 2);
    }
}
