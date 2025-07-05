//! Mixed unit arithmetic for quantities of the same dimension but different units
//!
//! This module implements arithmetic operations between quantities that have the same
//! dimensional exponents but different unit types. For example:
//! - Distance::<Meter> + Distance::<Kilometer>
//! - Mass::<Kilogram> + Mass::<Pound>
//! - Time::<Second> + Time::<Hour>
//!
//! The operations automatically convert both operands to SI base units, perform the
//! operation, and return a result in SI base units.

use crate::core::{Quantity, UnitComposition};
use std::ops::{Add, Sub, AddAssign, SubAssign};

// Addition for same dimension - handles both same and different units
impl<U1, U2, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    Add<Quantity<U2, L, M, T, K, I, J, N>> for Quantity<U1, L, M, T, K, I, J, N>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    // Result is in SI base units (represented by () tuple)
    type Output = Quantity<(), L, M, T, K, I, J, N>;

    fn add(self, rhs: Quantity<U2, L, M, T, K, I, J, N>) -> Self::Output {
        // Convert both operands to SI base units
        let lhs_si = self.value * U1::to_si_factor();
        let rhs_si = rhs.value * U2::to_si_factor();
        
        // Perform addition in SI units
        let result_si = lhs_si + rhs_si;
        
        // Return result in SI base units
        Quantity::new(result_si)
    }
}

// Subtraction for same dimension - handles both same and different units
impl<U1, U2, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    Sub<Quantity<U2, L, M, T, K, I, J, N>> for Quantity<U1, L, M, T, K, I, J, N>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    // Result is in SI base units (represented by () tuple)
    type Output = Quantity<(), L, M, T, K, I, J, N>;

    fn sub(self, rhs: Quantity<U2, L, M, T, K, I, J, N>) -> Self::Output {
        // Convert both operands to SI base units
        let lhs_si = self.value * U1::to_si_factor();
        let rhs_si = rhs.value * U2::to_si_factor();
        
        // Perform subtraction in SI units
        let result_si = lhs_si - rhs_si;
        
        // Return result in SI base units
        Quantity::new(result_si)
    }
}

// AddAssign for same dimension, different units
impl<U1, U2, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    AddAssign<Quantity<U2, L, M, T, K, I, J, N>> for Quantity<U1, L, M, T, K, I, J, N>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    fn add_assign(&mut self, rhs: Quantity<U2, L, M, T, K, I, J, N>) {
        // Convert rhs to this unit system
        let rhs_in_lhs_units = rhs.value * U2::to_si_factor() / U1::to_si_factor();
        self.value += rhs_in_lhs_units;
    }
}

// SubAssign for same dimension, different units
impl<U1, U2, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    SubAssign<Quantity<U2, L, M, T, K, I, J, N>> for Quantity<U1, L, M, T, K, I, J, N>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    fn sub_assign(&mut self, rhs: Quantity<U2, L, M, T, K, I, J, N>) {
        // Convert rhs to this unit system
        let rhs_in_lhs_units = rhs.value * U2::to_si_factor() / U1::to_si_factor();
        self.value -= rhs_in_lhs_units;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantities::*;

    #[test]
    fn test_mixed_distance_addition() {
        let meters = Distance::<Meter>::new(100.0);  // 100 m
        let kilometers = Distance::<Kilometer>::new(1.0);  // 1 km = 1000 m
        
        let sum = meters + kilometers;  // Should be 1100 m in SI units
        assert_eq!(sum.value(), 1100.0);
    }

    #[test]
    fn test_mixed_distance_subtraction() {
        let kilometers = Distance::<Kilometer>::new(2.0);  // 2 km = 2000 m
        let meters = Distance::<Meter>::new(500.0);  // 500 m
        
        let diff = kilometers - meters;  // Should be 1500 m in SI units
        assert_eq!(diff.value(), 1500.0);
    }

    #[test]
    fn test_mixed_mass_addition() {
        let kilograms = Mass::<Kilogram>::new(1.0);  // 1 kg
        let grams = Mass::<Gram>::new(500.0);  // 500 g = 0.5 kg
        
        let sum = kilograms + grams;  // Should be 1.5 kg in SI units
        assert_eq!(sum.value(), 1.5);
    }

    #[test]
    fn test_mixed_time_addition() {
        let hours = Time::<Hour>::new(1.0);  // 1 hour = 3600 s
        let minutes = Time::<Minute>::new(30.0);  // 30 min = 1800 s
        
        let sum = hours + minutes;  // Should be 5400 s in SI units
        assert_eq!(sum.value(), 5400.0);
    }

    #[test]
    fn test_add_assign_mixed_units() {
        let mut distance = Distance::<Meter>::new(100.0);  // 100 m
        distance += Distance::<Kilometer>::new(1.0);  // Add 1 km = 1000 m
        
        // distance should now be 1100 m (in Meter units)
        assert_eq!(distance.value(), 1100.0);
    }

    #[test]
    fn test_sub_assign_mixed_units() {
        let mut distance = Distance::<Kilometer>::new(2.0);  // 2 km
        distance -= Distance::<Meter>::new(500.0);  // Subtract 500 m = 0.5 km
        
        // distance should now be 1.5 km (in Kilometer units)
        assert_eq!(distance.value(), 1.5);
    }

    #[test]
    fn test_imperial_metric_conversion() {
        let meters = Distance::<Meter>::new(100.0);  // 100 m
        let feet = Distance::<Foot>::new(328.084);  // ~100 m in feet
        
        let sum = meters + feet;  // Should be ~200 m in SI units
        assert!((sum.value() - 200.0).abs() < 0.1);
    }

    #[test]
    fn test_astronomical_units() {
        let au = Distance::<AstronomicalUnit>::new(1.0);  // 1 AU
        let km = Distance::<Kilometer>::new(1000.0);  // 1000 km (tiny compared to AU)
        
        let sum = au + km;  // Should be ~149,597,870,700 + 1,000,000 m in SI (meters)
        // 1 AU = 1.495_978_707e11 m, so adding 1,000,000 m is negligible
        let expected = 1.495_978_707e11 + 1_000_000.0;
        assert!((sum.value() - expected).abs() < 1000.0);
    }

    #[test]
    fn test_chain_mixed_operations() {
        let m = Distance::<Meter>::new(100.0);
        let km = Distance::<Kilometer>::new(1.0);
        let cm = Distance::<Centimeter>::new(50.0);
        
        // 100 m + 1000 m + 0.5 m = 1100.5 m
        let result = m + km + cm;
        assert_eq!(result.value(), 1100.5);
    }
}