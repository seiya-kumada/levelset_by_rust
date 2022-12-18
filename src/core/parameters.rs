//use crate::interface::commandline_interface as cli;
#[derive(Clone)]
pub struct Parameters {
    pub wband: i32,
    pub wreset: i32,
    pub time_step: f64,
    pub gain: f64,
    pub constant_speed: f64,
    pub speed_threshold: f64,
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            wband: 0,
            wreset: 0,
            time_step: 0.0,
            gain: 0.0,
            constant_speed: 0.0,
            speed_threshold: 0.0,
        }
    }
}
