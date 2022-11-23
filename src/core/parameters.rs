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
