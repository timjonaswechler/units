use crate::composition::unit::UnitScale;

// Tuple for compound units
impl<U1, U2> UnitScale for (U1, U2)
where
    U1: UnitScale,
    U2: UnitScale,
{
    fn scale() -> f64 {
        U1::scale() * U2::scale()
    }
}
