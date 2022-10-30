pub struct Position2d {
    pub left: i32,
    pub right: i32,
    pub me: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Position2d {
    pub fn new(left: i32, right: i32, me: i32, top: i32, bottom: i32) -> Self {
        Self {
            left,
            right,
            me,
            top,
            bottom,
        }
    }
}

pub struct Position3d {
    pub left: i32,
    pub right: i32,
    pub me: i32,
    pub top: i32,
    pub bottom: i32,
    pub front: i32,
    pub back: i32,
}

impl Position3d {
    pub fn new(
        left: i32,
        right: i32,
        me: i32,
        top: i32,
        bottom: i32,
        front: i32,
        back: i32,
    ) -> Self {
        Self {
            left,
            right,
            me,
            top,
            bottom,
            front,
            back,
        }
    }
}
