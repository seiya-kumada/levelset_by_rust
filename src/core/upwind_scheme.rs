use super::upwind::{Upwind2d, Upwind3d};
use crate::core::indexer::{Indexer2d, Indexer3d};
use crate::core::point::{Point2d, Point3d};
use crate::core::position::{Position2d, Position3d};
use crate::core::speed::Speed;
use crate::core::util;
use std::cmp;
use std::rc::Rc;

pub struct UpwindScheme2d {
    pub position: Position2d,
    pub upwind: Upwind2d,
    pub phi: Rc<Vec<f64>>,
    pub indexer: Rc<Indexer2d>,
}

impl UpwindScheme2d {
    pub fn new(indexer: Rc<Indexer2d>, phi: Rc<Vec<f64>>) -> Self {
        Self {
            position: Position2d::new(),
            upwind: Upwind2d::new(),
            phi: Rc::clone(&phi),
            indexer: Rc::clone(&indexer),
        }
    }

    pub fn calculate(&mut self, p: &Point2d<i32>, speed: &Speed) -> f64 {
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

    // test ok
    pub fn calculate_with_positive_speed(&mut self) {
        self.upwind.fdxm = util::max(
            self.phi[(self.position.me) as usize] - self.phi[(self.position.left) as usize],
            0.0,
        );
        self.upwind.fdxp = util::min(
            self.phi[(self.position.right) as usize] - self.phi[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdym = util::max(
            self.phi[(self.position.me) as usize] - self.phi[(self.position.top) as usize],
            0.0,
        );
        self.upwind.fdyp = util::min(
            self.phi[(self.position.bottom) as usize] - self.phi[(self.position.me) as usize],
            0.0,
        );
    }

    pub fn calculate_with_negative_speed(&mut self) {
        self.upwind.fdxp = util::max(
            self.phi[(self.position.right) as usize] - self.phi[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdxm = util::min(
            self.phi[(self.position.me) as usize] - self.phi[(self.position.left) as usize],
            0.0,
        );
        self.upwind.fdyp = util::max(
            self.phi[(self.position.bottom) as usize] - self.phi[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdym = util::min(
            self.phi[(self.position.me) as usize] - self.phi[(self.position.top) as usize],
            0.0,
        );
    }
}
pub struct UpwindScheme3d {
    pub position: Position3d,
    pub upwind: Upwind3d,
    pub phi: Rc<Vec<f64>>,
    pub indexer: Rc<Indexer3d>,
}

impl UpwindScheme3d {
    pub fn new(indexer: Rc<Indexer3d>, phi: Rc<Vec<f64>>) -> Self {
        Self {
            position: Position3d::new(),
            upwind: Upwind3d::new(),
            phi: Rc::clone(&phi),
            indexer: Rc::clone(&indexer),
        }
    }

    pub fn calculate(&mut self, p: &Point3d<i32>, speed: &Speed) -> f64 {
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

    // test ok
    pub fn calculate_with_positive_speed(&mut self) {
        self.upwind.fdxm = util::max(
            self.phi[(self.position.me) as usize] - self.phi[(self.position.left) as usize],
            0.0,
        );
        self.upwind.fdxp = util::min(
            self.phi[(self.position.right) as usize] - self.phi[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdym = util::max(
            self.phi[(self.position.me) as usize] - self.phi[(self.position.top) as usize],
            0.0,
        );
        self.upwind.fdyp = util::min(
            self.phi[(self.position.bottom) as usize] - self.phi[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdzm = util::max(
            self.phi[(self.position.me) as usize] - self.phi[(self.position.front) as usize],
            0.0,
        );
        self.upwind.fdzp = util::min(
            self.phi[(self.position.back) as usize] - self.phi[(self.position.me) as usize],
            0.0,
        );
    }

    pub fn calculate_with_negative_speed(&mut self) {
        self.upwind.fdxp = util::max(
            self.phi[(self.position.right) as usize] - self.phi[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdxm = util::min(
            self.phi[(self.position.me) as usize] - self.phi[(self.position.left) as usize],
            0.0,
        );
        self.upwind.fdyp = util::max(
            self.phi[(self.position.bottom) as usize] - self.phi[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdym = util::min(
            self.phi[(self.position.me) as usize] - self.phi[(self.position.top) as usize],
            0.0,
        );
        self.upwind.fdzp = util::max(
            self.phi[(self.position.back) as usize] - self.phi[(self.position.me) as usize],
            0.0,
        );
        self.upwind.fdzm = util::min(
            self.phi[(self.position.me) as usize] - self.phi[(self.position.front) as usize],
            0.0,
        );
    }
}
