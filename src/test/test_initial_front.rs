use crate::core::types::{InitialFront, IntPoint, TwoDim};
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initial_front_2d() {
        let a = IntPoint::<TwoDim>::new(1, 2);
        let b = IntPoint::<TwoDim>::new(3, 4);
        let c = InitialFront::<TwoDim> { vertices: [a, b] };
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
