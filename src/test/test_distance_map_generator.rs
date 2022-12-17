use crate::core::distance_map_generator::{
    DistanceMapGenerator2d, DistanceMapGenerator3d, DistanceMapGeneratorMethod, PointInfo2d,
    PointInfo3d,
};
use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::status::Status;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::*;

    struct CheckerSelector2d {
        pub sizes: HashMap<usize, usize>,
        pub points: HashMap<usize, Vec<PointInfo2d>>,
    }

    impl CheckerSelector2d {
        const COUNT: usize = 37;
        pub fn new() -> Self {
            let points = HashMap::<usize, Vec<Point2d<i32>>>::new();
            let a = vec![
                PointInfo2d::new(Point2d::<i32>::new(-3, -1), 4),
                PointInfo2d::new(Point2d::<i32>::new(-3, 1), 5),
                PointInfo2d::new(Point2d::<i32>::new(-1, -3), 4),
                PointInfo2d::new(Point2d::<i32>::new(-1, 3), 5),
                PointInfo2d::new(Point2d::<i32>::new(1, -3), 7),
                PointInfo2d::new(Point2d::<i32>::new(1, 3), 8),
                PointInfo2d::new(Point2d::<i32>::new(3, -1), 7),
                PointInfo2d::new(Point2d::<i32>::new(3, 1), 8),
            ];

            let b = vec![
                PointInfo2d::new(Point2d::<i32>::new(-3, 0), 3),
                PointInfo2d::new(Point2d::<i32>::new(0, -3), 1),
                PointInfo2d::new(Point2d::<i32>::new(0, 3), 2),
                PointInfo2d::new(Point2d::<i32>::new(3, 0), 6),
            ];

            let c = vec![
                PointInfo2d::new(Point2d::<i32>::new(-2, -2), 4),
                PointInfo2d::new(Point2d::<i32>::new(-2, 2), 5),
                PointInfo2d::new(Point2d::<i32>::new(2, -2), 7),
                PointInfo2d::new(Point2d::<i32>::new(2, 2), 8),
            ];

            let d = vec![
                PointInfo2d::new(Point2d::<i32>::new(-2, -1), 4),
                PointInfo2d::new(Point2d::<i32>::new(-2, 1), 5),
                PointInfo2d::new(Point2d::<i32>::new(-1, -2), 4),
                PointInfo2d::new(Point2d::<i32>::new(-1, 2), 5),
                PointInfo2d::new(Point2d::<i32>::new(1, -2), 7),
                PointInfo2d::new(Point2d::<i32>::new(1, 2), 8),
                PointInfo2d::new(Point2d::<i32>::new(2, -1), 7),
                PointInfo2d::new(Point2d::<i32>::new(2, 1), 8),
            ];

            let e = vec![
                PointInfo2d::new(Point2d::<i32>::new(-2, 0), 3),
                PointInfo2d::new(Point2d::<i32>::new(0, -2), 1),
                PointInfo2d::new(Point2d::<i32>::new(0, 2), 2),
                PointInfo2d::new(Point2d::<i32>::new(2, 0), 6),
            ];

            let f = vec![
                PointInfo2d::new(Point2d::<i32>::new(-1, -1), 4),
                PointInfo2d::new(Point2d::<i32>::new(-1, 1), 5),
                PointInfo2d::new(Point2d::<i32>::new(1, -1), 7),
                PointInfo2d::new(Point2d::<i32>::new(1, 1), 8),
            ];

            let g = vec![
                PointInfo2d::new(Point2d::<i32>::new(-1, 0), 3),
                PointInfo2d::new(Point2d::<i32>::new(0, -1), 1),
                PointInfo2d::new(Point2d::<i32>::new(0, 1), 2),
                PointInfo2d::new(Point2d::<i32>::new(1, 0), 6),
            ];

            let h = vec![PointInfo2d::new(Point2d::<i32>::new(0, 0), 0)];

            Self {
                sizes: HashMap::<usize, usize>::from([
                    (10, 8),
                    (9, 4),
                    (8, 4),
                    (5, 8),
                    (4, 4),
                    (2, 4),
                    (1, 4),
                    (0, 1),
                ]),
                points: HashMap::<usize, Vec<PointInfo2d>>::from([
                    (10, a),
                    (9, b),
                    (8, c),
                    (5, d),
                    (4, e),
                    (2, f),
                    (1, g),
                    (0, h),
                ]),
            }
        }
    }

    struct CheckerSelector3d {
        pub sizes: HashMap<usize, usize>,
        pub points: HashMap<usize, Vec<PointInfo3d>>,
    }

    impl CheckerSelector3d {
        pub fn new() -> Self {
            let a = vec![PointInfo3d::new(Point3d::<i32>::new(0, 0, 0), 0)];
            let b = vec![
                PointInfo3d::new(Point3d::<i32>::new(-1, 0, 0), 9),
                PointInfo3d::new(Point3d::<i32>::new(0, -1, 0), 1),
                PointInfo3d::new(Point3d::<i32>::new(0, 0, -1), 3),
                PointInfo3d::new(Point3d::<i32>::new(0, 0, 1), 6),
                PointInfo3d::new(Point3d::<i32>::new(0, 1, 0), 2),
                PointInfo3d::new(Point3d::<i32>::new(1, 0, 0), 18),
            ];
            let c = vec![
                PointInfo3d::new(Point3d::<i32>::new(-1, -1, 0), 10),
                PointInfo3d::new(Point3d::<i32>::new(-1, 0, -1), 12),
                PointInfo3d::new(Point3d::<i32>::new(-1, 0, 1), 15),
                PointInfo3d::new(Point3d::<i32>::new(-1, 1, 0), 11),
                PointInfo3d::new(Point3d::<i32>::new(0, -1, -1), 4),
                PointInfo3d::new(Point3d::<i32>::new(0, -1, 1), 7),
                PointInfo3d::new(Point3d::<i32>::new(0, 1, -1), 5),
                PointInfo3d::new(Point3d::<i32>::new(0, 1, 1), 8),
                PointInfo3d::new(Point3d::<i32>::new(1, -1, 0), 19),
                PointInfo3d::new(Point3d::<i32>::new(1, 0, -1), 21),
                PointInfo3d::new(Point3d::<i32>::new(1, 0, 1), 24),
                PointInfo3d::new(Point3d::<i32>::new(1, 1, 0), 20),
            ];

            Self {
                sizes: HashMap::<usize, usize>::from([(2, 12), (1, 6), (0, 1)]),
                points: HashMap::<usize, Vec<PointInfo3d>>::from([(0, a), (1, b), (2, c)]),
            }
        }
    }
    impl CheckerSelector3d {
        const COUNT: usize = 19;
    }

    #[test]
    fn initialize_distance_map_2d() {
        let size = SpaceSize2d::new(3, 3);
        let statuses = RefCell::new(Vec::<Status>::new());
        let wband = 3;
        let indexer = Rc::new(Indexer2d::new(&size));
        let mut generator =
            DistanceMapGenerator2d::new(wband, Rc::clone(&indexer), RefCell::clone(&statuses));
        generator.create_distance_map();
        let map = generator.get_distance_map();
        let mut c = 0;
        for key in map.keys() {
            c += map.get_vec(key).unwrap().len();
        }
        assert_eq!(CheckerSelector2d::COUNT, c);
        let checker = CheckerSelector2d::new();
        for (i, key) in map.keys().enumerate() {
            let v = map.get_vec(key).unwrap();
            let size = v.len();
            let k = (*key) as usize;
            assert_eq!(size, checker.sizes[&k]);

            let answers = &checker.points[&k];
            assert_eq!(answers.len(), v.len());
            for i in 0..v.len() {
                let a = &v[i];
                let ap = a.point;
                let al = a.label;

                let b = &answers[i];
                let bp = b.point;
                let bl = b.label;

                assert_eq!(al, bl);
                assert!(ap == bp);
            }
        }
    }

    #[test]
    fn initialize_distance_map_3d() {
        let size = SpaceSize3d::new(3, 3, 3);
        let statuses = RefCell::new(Vec::<Status>::new());
        let wband = 1;
        let indexer = Rc::new(Indexer3d::new(&size));
        let mut generator =
            DistanceMapGenerator3d::new(wband, Rc::clone(&indexer), RefCell::clone(&statuses));
        generator.create_distance_map();
        let map = generator.get_distance_map();
        let mut c = 0;
        for key in map.keys() {
            c += map.get_vec(key).unwrap().len();
        }
        assert_eq!(CheckerSelector3d::COUNT, c);

        let checker = CheckerSelector3d::new();
        for (i, key) in map.keys().enumerate() {
            let v = map.get_vec(key).unwrap();
            let size = v.len();
            let k = (*key) as usize;
            assert_eq!(size, checker.sizes[&k]);

            let answers = &checker.points[&k];
            assert_eq!(answers.len(), v.len());
            for i in 0..v.len() {
                let a = &v[i];
                let ap = a.point;
                let al = a.label;

                let b = &answers[i];
                let bp = b.point;
                let bl = b.label;

                assert_eq!(al, bl);
                assert!(ap == bp);
            }
        }
    }
}
