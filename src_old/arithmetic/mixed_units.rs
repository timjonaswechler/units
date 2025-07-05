//! Arithmetic operations between quantities with different units but same dimensions.
//!
//! This module provides automatic operations for quantities that have the same physical
//! dimensions but different units, automatically handling conversions through
//! the hub-and-spoke system.
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Distance: Meter + Kilometer (automatic conversion)
//! let distance_m = Distance::<Meter>::new(1000.0);              // 1000 m
//! let distance_km = Distance::<Prefixed<Kilo, Meter>>::new(2.0); // 2 km = 2000 m
//! let total = distance_m + distance_km;                          // Result: 3000 m
//!
//! // Velocity: m/s + km/h (automatic conversion)
//! let velocity_ms = Velocity::<Meter, Second>::new(10.0);                    // 10 m/s
//! let velocity_kmh = Velocity::<Prefixed<Kilo, Meter>, Hour>::new(36.0);     // 36 km/h = 10 m/s
//! let total_velocity = velocity_ms + velocity_kmh;                           // Result: 20 m/s
//!
//! // The result always takes the type of the left operand
//! let result_in_kmh = velocity_kmh + velocity_ms;                            // Result: 72 km/h
//! ```

use crate::core::*;
use std::ops::{Add, Sub};

/// Add two quantities with different units but same dimensions.
///
/// This trait enables operations like:
/// ```rust
/// let distance_au = Distance::<AstronomicalUnit>::new(1.0);
/// let distance_km = Distance::<Kilometer>::new(1000.0);
/// let total = distance_au.add_different_unit(distance_km);
/// ```
pub trait AddDifferentUnit<Other> {
    type Output;
    fn add_different_unit(self, other: Other) -> Self::Output;
}

/// Subtract two quantities with different units but same dimensions.
pub trait SubDifferentUnit<Other> {
    type Output;
    fn sub_different_unit(self, other: Other) -> Self::Output;
}

impl<
    Unit1,
    Unit2,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> AddDifferentUnit<Quantity<Unit2, L, M, T, K, I, J, N>> for Quantity<Unit1, L, M, T, K, I, J, N>
where
    Self: ToSI,
    Quantity<Unit2, L, M, T, K, I, J, N>: ToSI,
    Quantity<Unit1, L, M, T, K, I, J, N>: FromSI,
{
    type Output = Quantity<Unit1, L, M, T, K, I, J, N>;

    fn add_different_unit(self, other: Quantity<Unit2, L, M, T, K, I, J, N>) -> Self::Output {
        let self_si = self.to_si();
        let other_si = other.to_si();
        let result_si = self_si + other_si;
        Self::Output::from_si(result_si)
    }
}

impl<
    Unit1,
    Unit2,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> SubDifferentUnit<Quantity<Unit2, L, M, T, K, I, J, N>> for Quantity<Unit1, L, M, T, K, I, J, N>
where
    Self: ToSI,
    Quantity<Unit2, L, M, T, K, I, J, N>: ToSI,
    Quantity<Unit1, L, M, T, K, I, J, N>: FromSI,
{
    type Output = Quantity<Unit1, L, M, T, K, I, J, N>;

    fn sub_different_unit(self, other: Quantity<Unit2, L, M, T, K, I, J, N>) -> Self::Output {
        let self_si = self.to_si();
        let other_si = other.to_si();
        let result_si = self_si - other_si;
        Self::Output::from_si(result_si)
    }
}

// ================================================================================================
// MIXED UNIT ARITHMETIC EXTENSION TRAIT
// ================================================================================================

/// Extension trait for mixed unit arithmetic operations.
///
/// This trait provides convenient methods for arithmetic between quantities
/// with different units but same dimensions, automatically handling conversions.
/// 
/// # Examples
///
/// ```rust,no_run
/// use star_sim::physics::units::*;
/// use star_sim::physics::units::arithmetic::mixed_units::MixedUnitArithmetic;
///
/// let distance_m = Distance::<Meter>::new(1000.0);
/// let distance_km = Distance::<Prefixed<Kilo, Meter>>::new(2.0);
/// let total = distance_m.add_mixed_unit(distance_km); // Result: 3000 m
/// ```
pub trait MixedUnitArithmetic<Other> {
    type Output;
    
    /// Add a quantity with different units but same dimensions.
    fn add_mixed_unit(self, other: Other) -> Self::Output;
    
    /// Subtract a quantity with different units but same dimensions.
    fn sub_mixed_unit(self, other: Other) -> Self::Output;
}

impl<
    Unit1,
    Unit2,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> MixedUnitArithmetic<Quantity<Unit2, L, M, T, K, I, J, N>> for Quantity<Unit1, L, M, T, K, I, J, N>
where
    Self: ToSI + FromSI,
    Quantity<Unit2, L, M, T, K, I, J, N>: ToSI,
{
    type Output = Quantity<Unit1, L, M, T, K, I, J, N>;

    fn add_mixed_unit(self, other: Quantity<Unit2, L, M, T, K, I, J, N>) -> Self::Output {
        let self_si = self.to_si();
        let other_si = other.to_si();
        let result_si = self_si + other_si;
        Self::Output::from_si(result_si)
    }

    fn sub_mixed_unit(self, other: Quantity<Unit2, L, M, T, K, I, J, N>) -> Self::Output {
        let self_si = self.to_si();
        let other_si = other.to_si();
        let result_si = self_si - other_si;
        Self::Output::from_si(result_si)
    }
}

// ================================================================================================
// COMPREHENSIVE TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prefix::*;
    use crate::quantities::*;
    use crate::variadic_syntax::{Velocity, Area, Volume};

    #[test]
    fn test_distance_mixed_units() {
        // Test Distance: Meter + Kilometer using MixedUnitArithmetic trait
        let distance_m = Distance::<Meter>::new(1000.0);        // 1000 m
        let distance_km = Distance::<Prefixed<Kilo, Meter>>::new(2.0); // 2 km = 2000 m
        
        // Test both directions using new trait methods
        let total_m = distance_m.add_mixed_unit(distance_km);     // Should be 3000 m
        let total_km = distance_km.add_mixed_unit(distance_m);    // Should be 3 km
        
        assert!((total_m.value - 3000.0).abs() < 1e-10);
        assert!((total_km.value - 3.0).abs() < 1e-10);
        
        // Test subtraction
        let diff_m = distance_km.sub_mixed_unit(distance_m);      // 2000m - 1000m = 1000m, in km = 1.0
        let diff_km = distance_m.sub_mixed_unit(distance_km);     // 1000m - 2000m = -1000m, in m = -1000.0
        
        assert!((diff_m.value - 1.0).abs() < 1e-10);
        assert!((diff_km.value - (-1000.0)).abs() < 1e-10);
    }

    #[test]
    fn test_velocity_mixed_units() {
        // Test Velocity: m/s + km/h using MixedUnitArithmetic trait
        let velocity_ms = Velocity::<Meter, Second>::new(10.0);                    // 10 m/s
        let velocity_kmh = Velocity::<Prefixed<Kilo, Meter>, Hour>::new(36.0);     // 36 km/h = 10 m/s
        
        let total_ms = velocity_ms.add_mixed_unit(velocity_kmh);   // Should be 20 m/s
        let total_kmh = velocity_kmh.add_mixed_unit(velocity_ms);  // Should be 72 km/h
        
        assert!((total_ms.value - 20.0).abs() < 1e-10);
        assert!((total_kmh.value - 72.0).abs() < 1e-10);
    }

    #[test]
    fn test_area_mixed_units() {
        // Test Area: m² + km² using MixedUnitArithmetic trait
        let area_m2 = Area::<Meter, Meter>::new(1000000.0);                                   // 1,000,000 m²
        let area_km2 = Area::<Prefixed<Kilo, Meter>, Prefixed<Kilo, Meter>>::new(1.0);      // 1 km² = 1,000,000 m²
        
        let total_m2 = area_m2.add_mixed_unit(area_km2);   // Should be 2,000,000 m²
        let total_km2 = area_km2.add_mixed_unit(area_m2);  // Should be 2 km²
        
        assert!((total_m2.value - 2000000.0).abs() < 1e-6);
        assert!((total_km2.value - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_volume_mixed_units() {
        // Test Volume: m³ + km³ using MixedUnitArithmetic trait
        let volume_m3 = Volume::<Meter, Meter, Meter>::new(1000000000.0);  // 1 billion m³
        let volume_km3 = Volume::<Prefixed<Kilo, Meter>, Prefixed<Kilo, Meter>, Prefixed<Kilo, Meter>>::new(1.0); // 1 km³
        
        let total_m3 = volume_m3.add_mixed_unit(volume_km3);   // Should be 2 billion m³
        let total_km3 = volume_km3.add_mixed_unit(volume_m3);  // Should be 2 km³
        
        assert!((total_m3.value - 2000000000.0).abs() < 1e-3);
        assert!((total_km3.value - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_astronomical_distances() {
        // Test Distance: AU + Parsec
        let distance_au = Distance::<AstronomicalUnit>::new(1.0);     // 1 AU
        let distance_pc = Distance::<Parsec>::new(1.0);               // 1 parsec
        
        // 1 parsec ≈ 206,265 AU, so total should be ≈ 206,266 AU
        let total_au = distance_au + distance_pc;
        assert!((total_au.value - 206266.0).abs() < 1.0); // Allow some floating point error
        
        // Total in parsecs should be ≈ 1.0000048 pc
        let total_pc = distance_pc + distance_au;
        assert!((total_pc.value - 1.0000048).abs() < 1e-6);
    }

    #[test]
    fn test_prefix_chains() {
        // Test multiple prefixes: mm + m + km
        let distance_mm = Distance::<Prefixed<Milli, Meter>>::new(5000.0);    // 5000 mm = 5 m
        let distance_m = Distance::<Meter>::new(10.0);                         // 10 m
        let distance_km = Distance::<Prefixed<Kilo, Meter>>::new(0.015);      // 0.015 km = 15 m
        
        // Total: 5 + 10 + 15 = 30 m
        let total = distance_mm + distance_m + distance_km;
        assert!((total.value - 30000.0).abs() < 1e-10); // Result in mm: 30,000 mm
    }
}