use crate::core::parameters::Parameters;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parameters() {
        let params = Parameters {
            wband: 1,
            wreset: 2,
            time_step: 3.0,
            gain: 4.0,
            constant_speed: 5.0,
            speed_threshold: 6.0,
        };
        assert_eq!(1, params.wband);
        assert_eq!(2, params.wreset);
        assert_eq!(3.0, params.time_step);
        assert_eq!(4.0, params.gain);
        assert_eq!(5.0, params.constant_speed);
        assert_eq!(6.0, params.speed_threshold);
    }
}
