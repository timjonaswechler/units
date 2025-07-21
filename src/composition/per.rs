use crate::composition::unit::UnitScale;

// Composition types
pub struct Per<Unit> {
    _unit: std::marker::PhantomData<Unit>,
}

impl<Unit> UnitScale for Per<Unit>
where
    Unit: UnitScale,
{
    fn scale() -> f64 {
        1.0 / Unit::scale()
    }
}
