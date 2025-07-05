//! Unit conversion system

use crate::core::{Quantity, UnitComposition};

/// Trait for converting between compatible units
pub trait ConvertTo<Target> {
    fn convert_to(self) -> Target;
}

/// Auto-conversion between units of the same dimension
impl<U1, U2, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    ConvertTo<Quantity<U2, L, M, T, K, I, J, N>> for Quantity<U1, L, M, T, K, I, J, N>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    #[inline]
    fn convert_to(self) -> Quantity<U2, L, M, T, K, I, J, N> {
        // Convert through SI: self_units -> SI -> target_units
        let si_value = self.value * U1::to_si_factor();
        let target_value = si_value * U2::from_si_factor();
        Quantity::new(target_value)
    }
}

// Note: Removed conflicting From implementation - use convert_to() explicitly

/// Extension trait for convenient conversions
pub trait Convert<U1> {
    fn convert<U2>(self) -> Quantity<U2, 0, 0, 0, 0, 0, 0, 0>
    where
        U2: UnitComposition;
}

// Helper trait for getting SI value
pub trait ToSI {
    fn to_si(&self) -> f64;
}

impl<U, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    ToSI for Quantity<U, L, M, T, K, I, J, N>
where
    U: UnitComposition,
{
    #[inline]
    fn to_si(&self) -> f64 {
        self.value * U::to_si_factor()
    }
}

// Helper trait for creating from SI value
pub trait FromSI<U> {
    fn from_si(value: f64) -> Self;
}

impl<U, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    FromSI<U> for Quantity<U, L, M, T, K, I, J, N>
where
    U: UnitComposition,
{
    #[inline]
    fn from_si(value: f64) -> Self {
        Self::new(value * U::from_si_factor())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Unit1;
    impl UnitComposition for Unit1 {
        fn to_si_factor() -> f64 { 1.0 }
        fn from_si_factor() -> f64 { 1.0 }
        fn symbol() -> String { "U1".to_string() }
    }

    struct Unit2;
    impl UnitComposition for Unit2 {
        fn to_si_factor() -> f64 { 1000.0 }  // 1000 U2 = 1 SI
        fn from_si_factor() -> f64 { 0.001 } // 1 SI = 0.001 U2
        fn symbol() -> String { "U2".to_string() }
    }

    type TestQuantity1 = Quantity<Unit1, 1, 0, 0, 0, 0, 0, 0>;
    type TestQuantity2 = Quantity<Unit2, 1, 0, 0, 0, 0, 0, 0>;

    #[test]
    fn test_conversion() {
        let q1 = TestQuantity1::new(1000.0);
        let q2: TestQuantity2 = q1.convert_to();
        
        // 1000 U1 * 1.0 = 1000 SI
        // 1000 SI * 0.001 = 1.0 U2
        assert_eq!(q2.value(), 1.0);
    }

    #[test]
    fn test_explicit_conversion() {
        let q1 = TestQuantity1::new(2000.0);
        let q2: TestQuantity2 = q1.convert_to();
        
        assert_eq!(q2.value(), 2.0);
    }

    #[test]
    fn test_to_si() {
        let q1 = TestQuantity1::new(5.0);
        assert_eq!(q1.to_si(), 5.0);

        let q2 = TestQuantity2::new(3.0);
        assert_eq!(q2.to_si(), 3000.0);
    }

    #[test]
    fn test_from_si() {
        let q1 = TestQuantity1::from_si(10.0);
        assert_eq!(q1.value(), 10.0);

        let q2 = TestQuantity2::from_si(5000.0);
        assert_eq!(q2.value(), 5.0);
    }
}