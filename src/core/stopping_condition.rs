pub struct StoppingCondition {
    total_speed: [f64; Self::NUM],
    counter: usize,
}

impl StoppingCondition {
    const NUM: usize = 3;
    const EPSILON: f64 = 1.0e-08;

    pub fn new() -> Self {
        Self {
            total_speed: [0.0, 0.0, 0.0],
            counter: 0,
        }
    }

    pub fn add_total_speed(&mut self, speed: f64) {
        self.total_speed[self.counter % Self::NUM] = speed;
        self.counter += 1;
        let a = [0, 9];
    }

    pub fn is_satisfied(&self) -> bool {
        (self.total_speed[0] - self.total_speed[1]).abs() < Self::EPSILON
            && (self.total_speed[1] - self.total_speed[2]).abs() < Self::EPSILON
    }

    pub fn get_counter(&self) -> usize {
        self.counter
    }
}
