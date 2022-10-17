use point::IntPoint;

pub struct InitialFront<const D: i32> {
    vertices: [IntPoint<D>; 2],
}
