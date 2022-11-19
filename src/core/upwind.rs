use crate::core::position::{Position2d, Position3d};
use crate::core::util;

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
    fn make_upwind_with_positive_speed(p: &Position2d, phi: &Vec<f64>) -> Self {
        let fdxm = util::max(phi[p.me as usize] - phi[p.left as usize], 0.0);
        let fdxp = util::min(phi[p.right as usize] - phi[p.me as usize], 0.0);
        let fdym = util::max(phi[p.me as usize] - phi[p.top as usize], 0.0);
        let fdyp = util::min(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
        Self {
            fdxm,
            fdxp,
            fdym,
            fdyp,
        }
    }

    fn make_upwind_with_negative_speed(p: &Position2d, phi: &Vec<f64>) -> Self {
        let fdxp = util::max(phi[p.right as usize] - phi[p.me as usize], 0.0);
        let fdxm = util::min(phi[p.me as usize] - phi[p.left as usize], 0.0);
        let fdyp = util::max(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
        let fdym = util::min(phi[p.me as usize] - phi[p.top as usize], 0.0);
        Self {
            fdxm,
            fdxp,
            fdym,
            fdyp,
        }
    }
}

impl Upwind3d {
    fn make_upwind_with_positive_speed(p: &Position3d, phi: &Vec<f64>) -> Self {
        let fdxm = util::max(phi[p.me as usize] - phi[p.left as usize], 0.0);
        let fdxp = util::min(phi[p.right as usize] - phi[p.me as usize], 0.0);
        let fdym = util::max(phi[p.me as usize] - phi[p.top as usize], 0.0);
        let fdyp = util::min(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
        let fdzm = util::max(phi[p.me as usize] - phi[p.front as usize], 0.0);
        let fdzp = util::min(phi[p.back as usize] - phi[p.me as usize], 0.0);
        Self {
            fdxm,
            fdxp,
            fdym,
            fdyp,
            fdzm,
            fdzp,
        }
    }

    fn make_upwind_with_negative_speed(p: &Position3d, phi: &Vec<f64>) -> Self {
        let fdxm = util::min(phi[p.me as usize] - phi[p.left as usize], 0.0);
        let fdxp = util::max(phi[p.right as usize] - phi[p.me as usize], 0.0);
        let fdym = util::min(phi[p.me as usize] - phi[p.top as usize], 0.0);
        let fdyp = util::max(phi[p.bottom as usize] - phi[p.me as usize], 0.0);
        let fdzm = util::min(phi[p.me as usize] - phi[p.front as usize], 0.0);
        let fdzp = util::max(phi[p.back as usize] - phi[p.me as usize], 0.0);
        Self {
            fdxm,
            fdxp,
            fdym,
            fdyp,
            fdzm,
            fdzp,
        }
    }
}
