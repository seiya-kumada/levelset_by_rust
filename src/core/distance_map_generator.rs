use crate::core::dim::{THREE, TWO};
use crate::core::neighboring_point::{NEIGHBORING_POINTS2D, NEIGHBORING_POINTS3D};
use crate::core::point::{Point2d, Point3d};
use crate::core::status::Status;
use crate::core::types::{Indexer, IntPoint, ThreeDim, TwoDim, Type};
use bimap::BiMap;
use multimap::MultiMap;
use std::rc::Rc;

#[inline]
fn is_zero(x: i32) -> bool {
    x == 0
}

#[inline]
fn is_negative(x: i32) -> bool {
    x < 0
}

#[inline]
fn to_symbol(x: i32) -> i32 {
    if is_zero(x) {
        0
    } else {
        if is_negative(x) {
            -1
        } else {
            1
        }
    }
}

//pub const POWER_OF_3: [usize; 4] = [0, 0, 3usize.pow(2), 3usize.pow(3)];
//struct Table<D: Type, const N: usize> {
//    indices: BiMap<IntPoint<D>, usize>,
//    points: [IntPoint<D>; N],
//}
//
//trait TableTrait {
//    type IntPoint;
//    fn initialize(&mut self);
//    fn new() -> Self;
//    //fn to_symbols(p: &Self::IntPoint) -> Self::IntPoint;
//    //fn index(&self, p: &Self::IntPoint) -> usize;
//}

//impl TableTrait for Table<TwoDim, { POWER_OF_3[TwoDim::NUM] }> {
//    type IntPoint = IntPoint<TwoDim>;
//
//    fn initialize(&mut self) {}
//    fn new() -> Self {
//        let mut obj = Self {
//            indices: BiMap::<Point2d<i32>, usize>::new(),
//            points: [Point2d::<i32>::new(0, 0); 9],
//        };
//        obj.initialize();
//        for i in 0..obj.points.len() {
//            obj.points[i] = obj.indices.get_by_right(&i).unwrap().clone();
//        }
//        obj
//    }
//}
struct Table2d {
    indices: BiMap<Point2d<i32>, usize>,
    points: [Point2d<i32>; 9],
}

impl Table2d {
    pub fn new() -> Self {
        let mut obj = Self {
            indices: BiMap::<Point2d<i32>, usize>::new(),
            points: [Point2d::<i32>::new(0, 0); 9],
        };
        obj.initialize();
        for i in 0..obj.points.len() {
            obj.points[i] = obj.indices.get_by_right(&i).unwrap().clone();
        }
        obj
    }

    fn index(&self, p: &Point2d<i32>) -> usize {
        self.indices
            .get_by_left(&Self::to_symbols(&p))
            .unwrap()
            .clone()
    }

    fn to_symbols(p: &Point2d<i32>) -> Point2d<i32> {
        Point2d::<i32>::new(to_symbol(p.x), to_symbol(p.y))
    }

    fn initialize(&mut self) {
        self.indices
            .insert(NEIGHBORING_POINTS2D.get(0, 0).clone(), 0);
        self.indices
            .insert(NEIGHBORING_POINTS2D.get(0, -1).clone(), 1);
        self.indices
            .insert(NEIGHBORING_POINTS2D.get(0, 1).clone(), 2);
        self.indices
            .insert(NEIGHBORING_POINTS2D.get(-1, 0).clone(), 3);
        self.indices
            .insert(NEIGHBORING_POINTS2D.get(-1, -1).clone(), 4);
        self.indices
            .insert(NEIGHBORING_POINTS2D.get(-1, 1).clone(), 5);
        self.indices
            .insert(NEIGHBORING_POINTS2D.get(1, 0).clone(), 6);
        self.indices
            .insert(NEIGHBORING_POINTS2D.get(1, -1).clone(), 7);
        self.indices
            .insert(NEIGHBORING_POINTS2D.get(1, 1).clone(), 8);
    }

    pub fn point(&self, index: i32) -> &Point2d<i32> {
        &self.points[index as usize]
    }
}

struct Table3d {
    indices: BiMap<Point3d<i32>, usize>,
    points: [Point3d<i32>; 27],
}

impl Table3d {
    pub fn new() -> Self {
        let mut obj = Self {
            indices: BiMap::<Point3d<i32>, usize>::new(),
            points: [Point3d::<i32>::new(0, 0, 0); 27],
        };
        obj.initialize();
        for i in 0..obj.points.len() {
            obj.points[i] = obj.indices.get_by_right(&i).unwrap().clone();
        }
        obj
    }

    fn index(&self, p: &Point3d<i32>) -> usize {
        self.indices
            .get_by_left(&Self::to_symbols(&p))
            .unwrap()
            .clone()
    }

    fn to_symbols(p: &Point3d<i32>) -> Point3d<i32> {
        Point3d::<i32>::new(to_symbol(p.x), to_symbol(p.y), to_symbol(p.z))
    }

    fn initialize(&mut self) {
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(0, 0, 0).clone(), 0);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(0, -1, 0).clone(), 1);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(0, 1, 0).clone(), 2);

        self.indices
            .insert(NEIGHBORING_POINTS3D.get(0, 0, -1).clone(), 3);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(0, -1, -1).clone(), 4);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(0, 1, -1).clone(), 5);

        self.indices
            .insert(NEIGHBORING_POINTS3D.get(0, 0, 1).clone(), 6);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(0, -1, 1).clone(), 7);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(0, 1, 1).clone(), 8);

        self.indices
            .insert(NEIGHBORING_POINTS3D.get(-1, 0, 0).clone(), 9);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(-1, -1, 0).clone(), 10);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(-1, 1, 0).clone(), 11);

        self.indices
            .insert(NEIGHBORING_POINTS3D.get(-1, 0, -1).clone(), 12);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(-1, -1, -1).clone(), 13);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(-1, 1, -1).clone(), 14);

        self.indices
            .insert(NEIGHBORING_POINTS3D.get(-1, 0, 1).clone(), 15);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(-1, -1, 1).clone(), 16);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(-1, 1, 1).clone(), 17);

        self.indices
            .insert(NEIGHBORING_POINTS3D.get(1, 0, 0).clone(), 18);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(1, -1, 0).clone(), 19);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(1, 1, 0).clone(), 20);

        self.indices
            .insert(NEIGHBORING_POINTS3D.get(1, 0, -1).clone(), 21);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(1, -1, -1).clone(), 22);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(1, 1, -1).clone(), 23);

        self.indices
            .insert(NEIGHBORING_POINTS3D.get(1, 0, 1).clone(), 24);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(1, -1, 1).clone(), 25);
        self.indices
            .insert(NEIGHBORING_POINTS3D.get(1, 1, 1).clone(), 26);
    }

    pub fn point(&self, index: i32) -> &Point3d<i32> {
        &self.points[index as usize]
    }
}

struct PointInfo<D: Type> {
    point: IntPoint<D>,
    label: i32,
}

type DistanceMap<D> = MultiMap<i32, PointInfo<D>>;

struct DistanceMapGenerator2d {
    distance_map: DistanceMap<TwoDim>,
    table: Table2d,
    wband: i32,
    squared_wband: i32,
    indexer: Rc<Indexer<TwoDim>>,
    statuses: Rc<Vec<Status>>,
}

impl DistanceMapGenerator2d {
    pub fn new(wband: i32, indexer: Rc<Indexer<TwoDim>>, statuses: Rc<Vec<Status>>) -> Self {
        Self {
            wband,
            indexer: Rc::clone(&indexer),
            statuses: Rc::clone(&statuses),
            table: Table2d::new(),
            distance_map: DistanceMap::<TwoDim>::new(),
            squared_wband: 0,
        }
    }

    pub fn get_distance_map(&self) -> &DistanceMap<TwoDim> {
        &self.distance_map
    }

    fn remove(
        &self,
        p: &IntPoint<TwoDim>,
        a: i32,
        indices: &[i32; 3],
        labels: &mut Vec<bool>,
    ) -> bool {
        let q = p + self.table.point(a);
        let r = self.indexer.get(&q);
        match self.statuses[r as usize] {
            Status::Front => {
                labels[indices[0] as usize] = false;
                labels[indices[1] as usize] = false;
                labels[indices[2] as usize] = false;
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    fn remove_(&self, p: &IntPoint<TwoDim>, a: i32, labels: &mut Vec<bool>) {
        let q = p + self.table.point(a);
        let r = self.indexer.get(&q);
        match self.statuses[r as usize] {
            Status::Front => {
                labels[a as usize] = false;
            }
            _ => (),
        }
    }
}
