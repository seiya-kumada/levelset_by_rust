use crate::core::initial_front::InitialFront2d;
use crate::core::point::Point2d;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initial_front_2d() {
        let a = Point2d { x: 1, y: 2 };
        let b = Point2d { x: 3, y: 4 };
        let c = InitialFront2d { vertices: [a, b] };
        let x0 = &c.vertices[0].x;
        let y0 = &c.vertices[0].y;
        let x1 = &c.vertices[1].x;
        let y1 = &c.vertices[1].y;
        assert_eq!(*x0, 1);
        assert_eq!(*y0, 2);
        assert_eq!(*x1, 3);
        assert_eq!(*y1, 4);
    }

    #[test]
    fn initial_front_2d_new() {
        let c = InitialFront2d::new(1, 2, 3, 4);
        let x0 = &c.vertices[0].x;
        let y0 = &c.vertices[0].y;
        let x1 = &c.vertices[1].x;
        let y1 = &c.vertices[1].y;
        assert_eq!(*x0, 1);
        assert_eq!(*y0, 2);
        assert_eq!(*x1, 3);
        assert_eq!(*y1, 4);
    }
}
