use std::marker::PhantomData;

/// A static scale that converts values of one unit to canonical SI values.
///
/// Each unit belongs to exactly one [`Quantity`](Unit::Quantity).
pub trait Unit {
    /// The physical quantity this unit measures.
    type Quantity;

    /// Multiplier from this unit to the quantity's canonical SI scale.
    const SCALE_TO_SI: f64;
}

/// Opt-in marker for units that may be combined with an SI prefix.
pub trait PrefixableUnit: Unit {}

/// A static SI prefix such as kilo or milli.
pub trait Prefix {
    /// Multiplier represented by this prefix.
    const FACTOR: f64;
}

/// A prefix applied to a prefixable unit.
///
/// `Prefixed` deliberately does not implement [`PrefixableUnit`], preventing
/// nested prefixes such as `Prefixed<Kilo, Kilogram>`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Prefixed<P, U>(PhantomData<(P, U)>);

impl<P, U> Unit for Prefixed<P, U>
where
    P: Prefix,
    U: PrefixableUnit,
{
    type Quantity = U::Quantity;

    const SCALE_TO_SI: f64 = P::FACTOR * U::SCALE_TO_SI;
}
