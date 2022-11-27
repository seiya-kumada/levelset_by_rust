use crate::core::neighboring_point::{NEIGHBORING_POINTS2D, NEIGHBORING_POINTS3D};
use crate::core::point::{Point2d, Point3d};
use bimap::BiMap;

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
}
