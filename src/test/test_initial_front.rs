use crate::core::dimension_types::dimension_ as dim;
//use crate::core::initial_front::InitialFront2d;
use crate::core::initial_front::InitialFront_;
use crate::core::point::IntPoint;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initial_front_2d() {
        let a: IntPoint<{ dim::TWO }> = [1, 2];
        let b: IntPoint<{ dim::TWO }> = [3, 4];
        let c = InitialFront_::<{ dim::TWO }> { vertices: [a, b] };
        let x0 = &c.vertices[0][0];
        let y0 = &c.vertices[0][1];
        let x1 = &c.vertices[1][0];
        let y1 = &c.vertices[1][1];
        assert_eq!(*x0, 1);
        assert_eq!(*y0, 2);
        assert_eq!(*x1, 3);
        assert_eq!(*y1, 4);
    }
}
