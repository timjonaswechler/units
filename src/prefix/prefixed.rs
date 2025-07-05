//! Prefixed unit wrapper

use crate::core::UnitComposition;
use crate::prefix::Prefix;
use std::marker::PhantomData;

/// Wrapper for prefixed units: Prefixed<Kilo, Meter> = Kilometer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Prefixed<P, U> {
    _phantom_prefix: PhantomData<P>,
    _phantom_unit: PhantomData<U>,
}

impl<P, U> UnitComposition for Prefixed<P, U>
where
    P: Prefix,
    U: UnitComposition,
{
    #[inline]
    fn to_si_factor() -> f64 {
        P::FACTOR * U::to_si_factor()
    }

    #[inline]
    fn from_si_factor() -> f64 {
        U::from_si_factor() / P::FACTOR
    }

    fn symbol() -> String {
        format!("{}{}", P::symbol(), U::symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prefix::{Kilo, Milli};

    struct TestUnit;
    impl UnitComposition for TestUnit {
        fn to_si_factor() -> f64 { 1.0 }
        fn from_si_factor() -> f64 { 1.0 }
        fn symbol() -> String { "T".to_string() }
    }

    #[test]
    fn test_prefixed_composition() {
        type KiloTest = Prefixed<Kilo, TestUnit>;
        
        assert_eq!(KiloTest::to_si_factor(), 1000.0);
        assert_eq!(KiloTest::from_si_factor(), 0.001);
        assert_eq!(KiloTest::symbol(), "kT");
    }

    #[test]
    fn test_prefixed_composition_milli() {
        type MilliTest = Prefixed<Milli, TestUnit>;
        
        assert_eq!(MilliTest::to_si_factor(), 0.001);
        assert_eq!(MilliTest::from_si_factor(), 1000.0);
        assert_eq!(MilliTest::symbol(), "mT");
    }
}