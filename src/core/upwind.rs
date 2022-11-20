use crate::core::position::{Position2d, Position3d};
use crate::core::util;
use std::rc::Rc;

pub struct Upwind2d {
    pub fdxm: f64,
    pub fdxp: f64,
    pub fdym: f64,
    pub fdyp: f64,
}

pub struct Upwind3d {
    pub fdxm: f64,
    pub fdxp: f64,
    pub fdym: f64,
    pub fdyp: f64,
    pub fdzm: f64,
    pub fdzp: f64,
}

impl Upwind2d {
    pub fn new() -> Self {
        Self {
            fdxm: 0.0,
            fdxp: 0.0,
            fdym: 0.0,
            fdyp: 0.0,
        }
    }

    pub fn make_upwind_with_positive_speed(&mut self, p: &Position2d, phi: Rc<Vec<f64>>) {
        self.fdxm = util::max(phi[p.me as usize] - phi[p.left as usize], 0.0);
        self.fdxp = util::min(phi[p.right as usize] - phi[p.me as usize], 0.0);
        self.fdym = util::max(phi[p.me as usize] - phi[p.top as usize], 0.0);
        self.fdyp = util::min(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
    }

    pub fn make_upwind_with_negative_speed(&mut self, p: &Position2d, phi: Rc<Vec<f64>>) {
        self.fdxp = util::max(phi[p.right as usize] - phi[p.me as usize], 0.0);
        self.fdxm = util::min(phi[p.me as usize] - phi[p.left as usize], 0.0);
        self.fdyp = util::max(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
        self.fdym = util::min(phi[p.me as usize] - phi[p.top as usize], 0.0);
    }
}

impl Upwind3d {
    pub fn new() -> Self {
        Self {
            fdxm: 0.0,
            fdxp: 0.0,
            fdym: 0.0,
            fdyp: 0.0,
            fdzm: 0.0,
            fdzp: 0.0,
        }
    }

    pub fn make_upwind_with_positive_speed(&mut self, p: &Position3d, phi: Rc<Vec<f64>>) {
        self.fdxm = util::max(phi[p.me as usize] - phi[p.left as usize], 0.0);
        self.fdxp = util::min(phi[p.right as usize] - phi[p.me as usize], 0.0);
        self.fdym = util::max(phi[p.me as usize] - phi[p.top as usize], 0.0);
        self.fdyp = util::min(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
        self.fdzm = util::max(phi[p.me as usize] - phi[p.front as usize], 0.0);
        self.fdzp = util::min(phi[p.back as usize] - phi[p.me as usize], 0.0);
    }

    pub fn make_upwind_with_negative_speed(&mut self, p: &Position3d, phi: Rc<Vec<f64>>) {
        self.fdxm = util::min(phi[p.me as usize] - phi[p.left as usize], 0.0);
        self.fdxp = util::max(phi[p.right as usize] - phi[p.me as usize], 0.0);
        self.fdym = util::min(phi[p.me as usize] - phi[p.top as usize], 0.0);
        self.fdyp = util::max(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
        self.fdzm = util::min(phi[p.me as usize] - phi[p.front as usize], 0.0);
        self.fdzp = util::max(phi[p.back as usize] - phi[p.me as usize], 0.0);
    }
}
