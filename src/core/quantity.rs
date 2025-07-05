//! Core quantity type with dimensional analysis

use std::marker::PhantomData;
use std::fmt;

/// Generic quantity type with dimensional exponents and unit composition
/// 
/// The dimensional exponents represent the seven SI base dimensions:
/// - L: Length (meter)
/// - M: Mass (kilogram) 
/// - T: Time (second)
/// - K: Temperature (kelvin)
/// - I: Electric current (ampere)
/// - J: Luminous intensity (candela)
/// - N: Amount of substance (mole)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity<Units, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> {
    pub(crate) value: f64,
    _phantom: PhantomData<Units>,
}

impl<Units, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    Quantity<Units, L, M, T, K, I, J, N> 
{
    /// Create a new quantity with the given value
    #[inline]
    pub const fn new(value: f64) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    /// Get the raw value (in whatever units this quantity uses)
    #[inline]
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Create a quantity from a raw value (unsafe - bypasses unit checking)
    #[inline]
    pub const fn from_raw(value: f64) -> Self {
        Self::new(value)
    }
}

impl<Units, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    fmt::Display for Quantity<Units, L, M, T, K, I, J, N>
where
    Units: crate::core::UnitComposition,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use default compact formatting for Display
        let formatter = crate::formatting::QuantityFormatter::compact();
        let formatted = formatter.format_value_and_symbol(self.value, &Units::symbol());
        write!(f, "{}", formatted)
    }
}

// Enhanced formatting implementation
impl<Units, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    crate::formatting::FormattedDisplay for Quantity<Units, L, M, T, K, I, J, N>
where
    Units: crate::core::UnitComposition,
{
    fn format_default(&self) -> String {
        format!("{}", self)
    }
    
    fn format_with(&self, formatter: &crate::formatting::QuantityFormatter) -> String {
        formatter.format_value_and_symbol(self.value, &Units::symbol())
    }
}

// Default implementation
impl<Units, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    Default for Quantity<Units, L, M, T, K, I, J, N> 
{
    #[inline]
    fn default() -> Self {
        Self::new(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantity_creation() {
        struct TestUnit;
        
        let q = Quantity::<TestUnit, 1, 0, 0, 0, 0, 0, 0>::new(42.0);
        assert_eq!(q.value(), 42.0);
    }

    #[test] 
    fn test_quantity_default() {
        struct TestUnit;
        
        let q = Quantity::<TestUnit, 1, 0, 0, 0, 0, 0, 0>::default();
        assert_eq!(q.value(), 0.0);
    }
}