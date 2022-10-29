use crate::core::types;
use crate::core::upwind_scheme as us;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn upwind_scheme() {
        let p = types::IntPoint::<types::TwoDim>::new(1, 2);
        let space_size = types::SpaceSize::<types::TwoDim>::new(1, 2);
        let indexer = types::Indexer::<types::TwoDim>::new(&space_size);
        let scheme = us::UpwindScheme::<types::TwoDim>::new(&p, &indexer);
        //b.set_position(&c);
        //a.make(&c);
    }
}
