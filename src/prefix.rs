use crate::unit::Unit;

/// Trait for SI prefixes (kilo, milli, mega, etc.)
///
/// Prefixes modify a unit by a power of 10.
pub trait Prefix: 'static + Copy + Clone + Sized {
    /// The multiplication factor for this prefix
    const FACTOR: f64;

    /// The symbol for this prefix (e.g., "k" for kilo, "m" for milli)
    const SYMBOL: &'static str;

    /// Get the factor at runtime
    fn factor() -> f64 {
        Self::FACTOR
    }

    /// Get the symbol at runtime
    fn symbol() -> &'static str {
        Self::SYMBOL
    }
}

/// Wrapper type for a unit with a prefix
///
/// # Example
///
/// ```rust
/// use units::prelude::*;
/// use units::prefix::{Prefixed, Kilo};
///
/// // Kilometer is Meter with Kilo prefix
/// type Kilometer = Prefixed<Kilo, Meter>;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Prefixed<P: Prefix, U: Unit> {
    _phantom: core::marker::PhantomData<(P, U)>,
}

impl<P: Prefix, U: Unit> Unit for Prefixed<P, U> {
    type BaseQuantity = U::BaseQuantity;

    const SYMBOL: &'static str = const_concat_symbol::<P, U>();

    const TO_SI: f64 = P::FACTOR * U::TO_SI;

    const OFFSET: f64 = U::OFFSET;
}

// Helper function to concatenate prefix and unit symbols at compile time
const fn const_concat_symbol<P: Prefix, U: Unit>() -> &'static str {
    // Note: In real implementation, we'd need a macro for this
    // For now, we'll use a workaround
    U::SYMBOL
}

// Standard SI prefixes

/// Yotta prefix (10^24)
#[derive(Debug, Clone, Copy)]
pub struct Yotta;
impl Prefix for Yotta {
    const FACTOR: f64 = 1e24;
    const SYMBOL: &'static str = "Y";
}

/// Zetta prefix (10^21)
#[derive(Debug, Clone, Copy)]
pub struct Zetta;
impl Prefix for Zetta {
    const FACTOR: f64 = 1e21;
    const SYMBOL: &'static str = "Z";
}

/// Exa prefix (10^18)
#[derive(Debug, Clone, Copy)]
pub struct Exa;
impl Prefix for Exa {
    const FACTOR: f64 = 1e18;
    const SYMBOL: &'static str = "E";
}

/// Peta prefix (10^15)
#[derive(Debug, Clone, Copy)]
pub struct Peta;
impl Prefix for Peta {
    const FACTOR: f64 = 1e15;
    const SYMBOL: &'static str = "P";
}

/// Tera prefix (10^12)
#[derive(Debug, Clone, Copy)]
pub struct Tera;
impl Prefix for Tera {
    const FACTOR: f64 = 1e12;
    const SYMBOL: &'static str = "T";
}

/// Giga prefix (10^9)
#[derive(Debug, Clone, Copy)]
pub struct Giga;
impl Prefix for Giga {
    const FACTOR: f64 = 1e9;
    const SYMBOL: &'static str = "G";
}

/// Mega prefix (10^6)
#[derive(Debug, Clone, Copy)]
pub struct Mega;
impl Prefix for Mega {
    const FACTOR: f64 = 1e6;
    const SYMBOL: &'static str = "M";
}

/// Kilo prefix (10^3)
#[derive(Debug, Clone, Copy)]
pub struct Kilo;
impl Prefix for Kilo {
    const FACTOR: f64 = 1e3;
    const SYMBOL: &'static str = "k";
}

/// Hecto prefix (10^2)
#[derive(Debug, Clone, Copy)]
pub struct Hecto;
impl Prefix for Hecto {
    const FACTOR: f64 = 1e2;
    const SYMBOL: &'static str = "h";
}

/// Deca prefix (10^1)
#[derive(Debug, Clone, Copy)]
pub struct Deca;
impl Prefix for Deca {
    const FACTOR: f64 = 1e1;
    const SYMBOL: &'static str = "da";
}

/// Deci prefix (10^-1)
#[derive(Debug, Clone, Copy)]
pub struct Deci;
impl Prefix for Deci {
    const FACTOR: f64 = 1e-1;
    const SYMBOL: &'static str = "d";
}

/// Centi prefix (10^-2)
#[derive(Debug, Clone, Copy)]
pub struct Centi;
impl Prefix for Centi {
    const FACTOR: f64 = 1e-2;
    const SYMBOL: &'static str = "c";
}

/// Milli prefix (10^-3)
#[derive(Debug, Clone, Copy)]
pub struct Milli;
impl Prefix for Milli {
    const FACTOR: f64 = 1e-3;
    const SYMBOL: &'static str = "m";
}

/// Micro prefix (10^-6)
#[derive(Debug, Clone, Copy)]
pub struct Micro;
impl Prefix for Micro {
    const FACTOR: f64 = 1e-6;
    const SYMBOL: &'static str = "μ";
}

/// Nano prefix (10^-9)
#[derive(Debug, Clone, Copy)]
pub struct Nano;
impl Prefix for Nano {
    const FACTOR: f64 = 1e-9;
    const SYMBOL: &'static str = "n";
}

/// Pico prefix (10^-12)
#[derive(Debug, Clone, Copy)]
pub struct Pico;
impl Prefix for Pico {
    const FACTOR: f64 = 1e-12;
    const SYMBOL: &'static str = "p";
}

/// Femto prefix (10^-15)
#[derive(Debug, Clone, Copy)]
pub struct Femto;
impl Prefix for Femto {
    const FACTOR: f64 = 1e-15;
    const SYMBOL: &'static str = "f";
}

/// Atto prefix (10^-18)
#[derive(Debug, Clone, Copy)]
pub struct Atto;
impl Prefix for Atto {
    const FACTOR: f64 = 1e-18;
    const SYMBOL: &'static str = "a";
}

/// Zepto prefix (10^-21)
#[derive(Debug, Clone, Copy)]
pub struct Zepto;
impl Prefix for Zepto {
    const FACTOR: f64 = 1e-21;
    const SYMBOL: &'static str = "z";
}

/// Yocto prefix (10^-24)
#[derive(Debug, Clone, Copy)]
pub struct Yocto;
impl Prefix for Yocto {
    const FACTOR: f64 = 1e-24;
    const SYMBOL: &'static str = "y";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_factors() {
        assert_eq!(Kilo::factor(), 1e3);
        assert_eq!(Mega::factor(), 1e6);
        assert_eq!(Milli::factor(), 1e-3);
        assert_eq!(Micro::factor(), 1e-6);
    }

    #[test]
    fn test_prefix_symbols() {
        assert_eq!(Kilo::symbol(), "k");
        assert_eq!(Mega::symbol(), "M");
        assert_eq!(Milli::symbol(), "m");
        assert_eq!(Micro::symbol(), "μ");
    }
}
