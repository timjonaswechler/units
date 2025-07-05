//! Metric prefix definitions

/// Trait for metric prefixes
pub trait Prefix {
    /// The multiplication factor for this prefix
    const FACTOR: f64;
    
    /// The symbol for this prefix (e.g., "k" for kilo)
    fn symbol() -> &'static str;
}

// Large prefixes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Yotta;
impl Prefix for Yotta {
    const FACTOR: f64 = 1e24;
    fn symbol() -> &'static str { "Y" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Zetta;
impl Prefix for Zetta {
    const FACTOR: f64 = 1e21;
    fn symbol() -> &'static str { "Z" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Exa;
impl Prefix for Exa {
    const FACTOR: f64 = 1e18;
    fn symbol() -> &'static str { "E" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Peta;
impl Prefix for Peta {
    const FACTOR: f64 = 1e15;
    fn symbol() -> &'static str { "P" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Tera;
impl Prefix for Tera {
    const FACTOR: f64 = 1e12;
    fn symbol() -> &'static str { "T" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Giga;
impl Prefix for Giga {
    const FACTOR: f64 = 1e9;
    fn symbol() -> &'static str { "G" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Mega;
impl Prefix for Mega {
    const FACTOR: f64 = 1e6;
    fn symbol() -> &'static str { "M" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Kilo;
impl Prefix for Kilo {
    const FACTOR: f64 = 1e3;
    fn symbol() -> &'static str { "k" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Hecto;
impl Prefix for Hecto {
    const FACTOR: f64 = 1e2;
    fn symbol() -> &'static str { "h" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Deca;
impl Prefix for Deca {
    const FACTOR: f64 = 1e1;
    fn symbol() -> &'static str { "da" }
}

// Small prefixes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Deci;
impl Prefix for Deci {
    const FACTOR: f64 = 1e-1;
    fn symbol() -> &'static str { "d" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Centi;
impl Prefix for Centi {
    const FACTOR: f64 = 1e-2;
    fn symbol() -> &'static str { "c" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Milli;
impl Prefix for Milli {
    const FACTOR: f64 = 1e-3;
    fn symbol() -> &'static str { "m" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Micro;
impl Prefix for Micro {
    const FACTOR: f64 = 1e-6;
    fn symbol() -> &'static str { "μ" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Nano;
impl Prefix for Nano {
    const FACTOR: f64 = 1e-9;
    fn symbol() -> &'static str { "n" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pico;
impl Prefix for Pico {
    const FACTOR: f64 = 1e-12;
    fn symbol() -> &'static str { "p" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Femto;
impl Prefix for Femto {
    const FACTOR: f64 = 1e-15;
    fn symbol() -> &'static str { "f" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Atto;
impl Prefix for Atto {
    const FACTOR: f64 = 1e-18;
    fn symbol() -> &'static str { "a" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Zepto;
impl Prefix for Zepto {
    const FACTOR: f64 = 1e-21;
    fn symbol() -> &'static str { "z" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Yocto;
impl Prefix for Yocto {
    const FACTOR: f64 = 1e-24;
    fn symbol() -> &'static str { "y" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_factors() {
        assert_eq!(Kilo::FACTOR, 1000.0);
        assert_eq!(Mega::FACTOR, 1_000_000.0);
        assert_eq!(Milli::FACTOR, 0.001);
        assert_eq!(Micro::FACTOR, 0.000_001);
    }

    #[test]
    fn test_prefix_symbols() {
        assert_eq!(Kilo::symbol(), "k");
        assert_eq!(Mega::symbol(), "M");
        assert_eq!(Milli::symbol(), "m");
        assert_eq!(Micro::symbol(), "μ");
    }
}