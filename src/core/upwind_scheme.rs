use super::upwind::{Upwind2d, Upwind3d};
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::point::{Point2d, Point3d};
use crate::core::position::{Position2d, Position3d};
use crate::core::speed::Speed;
use crate::core::util;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

pub trait UpwindSchemeMethod<T, P> {
    fn new(t: Rc<T>, phi: Rc<RefCell<Vec<f64>>>) -> Self;
    fn calculate(&mut self, p: &P, speed: Speed) -> f64;
}

pub struct UpwindScheme2d {
    pub position: Position2d,
    pub upwind: Upwind2d,
    pub phi: RefCell<Vec<f64>>,
    pub indexer: Rc<Indexer2d>,
}

impl UpwindSchemeMethod<Indexer2d, Point2d<i32>> for UpwindScheme2d {
    fn new(indexer: Rc<Indexer2d>, phi: Rc<RefCell<Vec<f64>>>) -> Self {
        Self {
            position: Position2d::new(),
            upwind: Upwind2d::new(),
            phi: RefCell::clone(&phi),
            indexer: Rc::clone(&indexer),
        }
    }

    fn calculate(&mut self, p: &Point2d<i32>, speed: Speed) -> f64 {
        self.position.set_position(p, Rc::clone(&self.indexer));
        match speed {
            Speed::Positive => self.calculate_with_positive_speed(),
            Speed::Negative => self.calculate_with_negative_speed(),
        }
        self.upwind.fdxm.powf(2.0)
            + self.upwind.fdxp.powf(2.0)
            + self.upwind.fdym.powf(2.0)
            + self.upwind.fdyp.powf(2.0)
    }
}

impl UpwindScheme2d {
    //pub fn calculate(&mut self, p: &Point2d<i32>, speed: &Speed) -> f64 {
    //    self.position.set_position(p, Rc::clone(&self.indexer));
    //    match speed {
    //        Speed::Positive => self.calculate_with_positive_speed(),
    //        Speed::Negative => self.calculate_with_negative_speed(),
    //    }
    //    self.upwind.fdxm.powf(2.0)
    //        + self.upwind.fdxp.powf(2.0)
    //        + self.upwind.fdym.powf(2.0)
    //        + self.upwind.fdyp.powf(2.0)
    //}

    // test ok
    pub fn calculate_with_positive_speed(&mut self) {
        self.upwind.fdxm = util::max(
            self.phi.borrow()[(self.position.me) as usize]
                - self.phi.borrow()[(self.position.left) as usize],
            0.0,
        );
        self.upwind.fdxp = util::min(
            self.phi.borrow()[(self.position.right) as usize]
                - self.phi.borrow()[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdym = util::max(
            self.phi.borrow()[(self.position.me) as usize]
                - self.phi.borrow()[(self.position.top) as usize],
            0.0,
        );
        self.upwind.fdyp = util::min(
            self.phi.borrow()[(self.position.bottom) as usize]
                - self.phi.borrow()[(self.position.me) as usize],
            0.0,
        );
    }

    //test ok
    pub fn calculate_with_negative_speed(&mut self) {
        self.upwind.fdxp = util::max(
            self.phi.borrow()[(self.position.right) as usize]
                - self.phi.borrow()[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdxm = util::min(
            self.phi.borrow()[(self.position.me) as usize]
                - self.phi.borrow()[(self.position.left) as usize],
            0.0,
        );
        self.upwind.fdyp = util::max(
            self.phi.borrow()[(self.position.bottom) as usize]
                - self.phi.borrow()[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdym = util::min(
            self.phi.borrow()[(self.position.me) as usize]
                - self.phi.borrow()[(self.position.top) as usize],
            0.0,
        );
    }
}
pub struct UpwindScheme3d {
    pub position: Position3d,
    pub upwind: Upwind3d,
    pub phi: RefCell<Vec<f64>>,
    pub indexer: Rc<Indexer3d>,
}

impl UpwindSchemeMethod<Indexer3d, Point3d<i32>> for UpwindScheme3d {
    fn new(indexer: Rc<Indexer3d>, phi: Rc<RefCell<Vec<f64>>>) -> Self {
        Self {
            position: Position3d::new(),
            upwind: Upwind3d::new(),
            phi: RefCell::clone(&phi),
            indexer: Rc::clone(&indexer),
        }
    }

    fn calculate(&mut self, p: &Point3d<i32>, speed: Speed) -> f64 {
        self.position.set_position(p, Rc::clone(&self.indexer));
        match speed {
            Speed::Positive => self.calculate_with_positive_speed(),
            Speed::Negative => self.calculate_with_negative_speed(),
        }
        self.upwind.fdxm.powf(2.0)
            + self.upwind.fdxp.powf(2.0)
            + self.upwind.fdym.powf(2.0)
            + self.upwind.fdyp.powf(2.0)
            + self.upwind.fdzm.powf(2.0)
            + self.upwind.fdzp.powf(2.0)
    }
}

impl UpwindScheme3d {
    //pub fn calculate(&mut self, p: &Point3d<i32>, speed: &Speed) -> f64 {
    //    self.position.set_position(p, Rc::clone(&self.indexer));
    //    match speed {
    //        Speed::Positive => self.calculate_with_positive_speed(),
    //        Speed::Negative => self.calculate_with_negative_speed(),
    //    }
    //    self.upwind.fdxm.powf(2.0)
    //        + self.upwind.fdxp.powf(2.0)
    //        + self.upwind.fdym.powf(2.0)
    //        + self.upwind.fdyp.powf(2.0)
    //        + self.upwind.fdzm.powf(2.0)
    //        + self.upwind.fdzp.powf(2.0)
    //}

    pub fn calculate_with_positive_speed(&mut self) {
        self.upwind.fdxm = util::max(
            self.phi.borrow()[(self.position.me) as usize]
                - self.phi.borrow()[(self.position.left) as usize],
            0.0,
        );
        self.upwind.fdxp = util::min(
            self.phi.borrow()[(self.position.right) as usize]
                - self.phi.borrow()[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdym = util::max(
            self.phi.borrow()[(self.position.me) as usize]
                - self.phi.borrow()[(self.position.top) as usize],
            0.0,
        );
        self.upwind.fdyp = util::min(
            self.phi.borrow()[(self.position.bottom) as usize]
                - self.phi.borrow()[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdzm = util::max(
            self.phi.borrow()[(self.position.me) as usize]
                - self.phi.borrow()[(self.position.front) as usize],
            0.0,
        );
        self.upwind.fdzp = util::min(
            self.phi.borrow()[(self.position.back) as usize]
                - self.phi.borrow()[(self.position.me) as usize],
            0.0,
        );
    }

    pub fn calculate_with_negative_speed(&mut self) {
        self.upwind.fdxp = util::max(
            self.phi.borrow()[(self.position.right) as usize]
                - self.phi.borrow()[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdxm = util::min(
            self.phi.borrow()[(self.position.me) as usize]
                - self.phi.borrow()[(self.position.left) as usize],
            0.0,
        );
        self.upwind.fdyp = util::max(
            self.phi.borrow()[(self.position.bottom) as usize]
                - self.phi.borrow()[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdym = util::min(
            self.phi.borrow()[(self.position.me) as usize]
                - self.phi.borrow()[(self.position.top) as usize],
            0.0,
        );
        self.upwind.fdzp = util::max(
            self.phi.borrow()[(self.position.back) as usize]
                - self.phi.borrow()[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdzm = util::min(
            self.phi.borrow()[(self.position.me) as usize]
                - self.phi.borrow()[(self.position.front) as usize],
            0.0,
        );
    }
}
