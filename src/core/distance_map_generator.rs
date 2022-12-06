use crate::core::dim::{THREE, TWO};
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::neighboring_point::{NEIGHBORING_POINTS2D, NEIGHBORING_POINTS3D};
use crate::core::point::{Point2d, Point3d};
use crate::core::status::Status;
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
pub trait New<T> {
    fn new(wband: i32, indexer: Rc<T>, statuses: Rc<Vec<Status>>) -> Self;
}

#[derive(Eq, PartialEq, Hash)]
pub struct PointInfo2d {
    point: Point2d<i32>,
    label: usize,
}

impl PointInfo2d {
    pub fn new(point: Point2d<i32>, label: usize) -> Self {
        Self { point, label }
    }
}

type DistanceMap2d = MultiMap<i32, PointInfo2d>;

pub struct DistanceMapGenerator2d {
    distance_map: DistanceMap2d,
    table: Table2d,
    wband: i32,
    squared_wband: i32,
    indexer: Rc<Indexer2d>,
    statuses: Rc<Vec<Status>>,
}

impl New<Indexer2d> for DistanceMapGenerator2d {
    fn new(wband: i32, indexer: Rc<Indexer2d>, statuses: Rc<Vec<Status>>) -> Self {
        Self {
            wband,
            indexer: Rc::clone(&indexer),
            statuses: Rc::clone(&statuses),
            table: Table2d::new(),
            distance_map: DistanceMap2d::new(),
            squared_wband: 0,
        }
    }
}
impl DistanceMapGenerator2d {
    //pub fn new(wband: i32, indexer: Rc<Indexer2d>, statuses: Rc<Vec<Status>>) -> Self {
    //    Self {
    //        wband,
    //        indexer: Rc::clone(&indexer),
    //        statuses: Rc::clone(&statuses),
    //        table: Table2d::new(),
    //        distance_map: DistanceMap2d::new(),
    //        squared_wband: 0,
    //    }
    //}

    pub fn get_distance_map(&self) -> &DistanceMap2d {
        &self.distance_map
    }

    fn remove(&self, p: &Point2d<i32>, a: i32, indices: &[i32; 3], labels: &mut Vec<bool>) -> bool {
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

    fn remove_(&self, p: &Point2d<i32>, a: i32, labels: &mut Vec<bool>) {
        let q = p + self.table.point(a);
        let r = self.indexer.get(&q);
        match self.statuses[r as usize] {
            Status::Front => {
                labels[a as usize] = false;
            }
            _ => (),
        }
    }

    fn register_distance(&mut self, p: &Point2d<i32>, d: i32) {
        let index = self.table.index(p);
        self.distance_map
            .insert(d, PointInfo2d::new(p.clone(), index));
    }

    pub fn create_distance_map(&mut self) {
        for x in -self.wband..(1 + self.wband) {
            let sx = x * x;
            for y in -self.wband..(1 + self.wband) {
                let d = sx + y * y;
                if d <= self.squared_wband {
                    let p = Point2d::<i32>::new(x, y);
                    self.register_distance(&p, d);
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct PointInfo3d {
    point: Point3d<i32>,
    label: usize,
}

impl PointInfo3d {
    pub fn new(point: Point3d<i32>, label: usize) -> Self {
        Self { point, label }
    }
}

type DistanceMap3d = MultiMap<i32, PointInfo3d>;

pub struct DistanceMapGenerator3d {
    distance_map: DistanceMap3d,
    table: Table3d,
    wband: i32,
    squared_wband: i32,
    indexer: Rc<Indexer3d>,
    statuses: Rc<Vec<Status>>,
}

impl New<Indexer3d> for DistanceMapGenerator3d {
    fn new(wband: i32, indexer: Rc<Indexer3d>, statuses: Rc<Vec<Status>>) -> Self {
        Self {
            wband,
            indexer: Rc::clone(&indexer),
            statuses: Rc::clone(&statuses),
            table: Table3d::new(),
            distance_map: DistanceMap3d::new(),
            squared_wband: 0,
        }
    }
}

impl DistanceMapGenerator3d {
    //pub fn new(wband: i32, indexer: Rc<Indexer3d>, statuses: Rc<Vec<Status>>) -> Self {
    //    Self {
    //        wband,
    //        indexer: Rc::clone(&indexer),
    //        statuses: Rc::clone(&statuses),
    //        table: Table3d::new(),
    //        distance_map: DistanceMap3d::new(),
    //        squared_wband: 0,
    //    }
    //}

    pub fn get_distance_map(&self) -> &DistanceMap3d {
        &self.distance_map
    }

    fn remove(&self, p: &Point3d<i32>, a: i32, indices: &[i32; 9], labels: &mut Vec<bool>) -> bool {
        let q = p + self.table.point(a);
        let r = self.indexer.get(&q);
        match self.statuses[r as usize] {
            Status::Front => {
                labels[indices[0] as usize] = false;
                labels[indices[1] as usize] = false;
                labels[indices[2] as usize] = false;
                labels[indices[3] as usize] = false;
                labels[indices[4] as usize] = false;
                labels[indices[5] as usize] = false;
                labels[indices[6] as usize] = false;
                labels[indices[7] as usize] = false;
                labels[indices[8] as usize] = false;
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    fn remove_(&self, p: &Point3d<i32>, a: i32, labels: &mut Vec<bool>) {
        let q = p + self.table.point(a);
        let r = self.indexer.get(&q);
        match self.statuses[r as usize] {
            Status::Front => {
                labels[a as usize] = false;
            }
            _ => (),
        }
    }

    fn register_distance(&mut self, p: &Point3d<i32>, d: i32) {
        let index = self.table.index(p);
        self.distance_map
            .insert(d, PointInfo3d::new(p.clone(), index));
    }

    pub fn create_distance_map(&mut self) {
        for x in -self.wband..(1 + self.wband) {
            let sx = x * x;
            for y in -self.wband..(1 + self.wband) {
                let sy = y * y;
                for z in -self.wband..(1 + self.wband) {
                    let d = sx + sy + z * z;
                    if d <= self.squared_wband {
                        let p = Point3d::<i32>::new(x, y, z);
                        self.register_distance(&p, d);
                    }
                }
            }
        }
    }
}
