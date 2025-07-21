use crate::composition::unit::UnitScale;

pub trait PrefixScale {
    fn factor() -> f64;
}

// Prefixed unit wrapper
pub struct Prefixed<Prefix, Unit> {
    _prefix: std::marker::PhantomData<Prefix>,
    _unit: std::marker::PhantomData<Unit>,
}

impl<Prefix, Unit> UnitScale for Prefixed<Prefix, Unit>
where
    Prefix: PrefixScale,
    Unit: UnitScale,
{
    fn scale() -> f64 {
        Prefix::factor() * Unit::scale()
    }
}
