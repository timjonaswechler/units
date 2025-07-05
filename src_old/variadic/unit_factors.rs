//! Unit factor calculation system for variadic unit types.
//!
//! This module provides the `UnitFactor` trait system that enables correct
//! unit conversion calculations for variadic types like `Velocity<Meter, Second>`.
//!
//! # Overview
//!
//! The problem: Variadic types like `Velocity<Meter, Second>` need to calculate
//! their conversion factor to SI units by combining the factors of their component units.
//! For velocity (m/s), this means: `meter_factor / second_factor`.
//!
//! # Design
//!
//! 1. **UnitFactor trait**: Each unit type implements this to provide its conversion factor
//! 2. **Composition**: Variadic types calculate their factor by combining component factors
//! 3. **Prefix support**: `Prefixed<P, U>` automatically handles prefix multiplication
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Base units have factor 1.0 (they are SI base units)
//! assert_eq!(Meter::factor(), 1.0);
//! assert_eq!(Second::factor(), 1.0);
//!
//! // Prefixed units multiply by prefix factor
//! assert_eq!(Prefixed::<Kilo, Meter>::factor(), 1000.0);
//! assert_eq!(Prefixed::<Milli, Second>::factor(), 0.001);
//!
//! // Non-SI units have their conversion factors
//! assert_eq!(AstronomicalUnit::factor(), METERS_PER_AU);
//! ```

use crate::prefix::*;
use crate::constants::*;
use crate::quantities::*;

/// Trait for units that can provide a conversion factor to SI base units.
///
/// This trait is implemented by all unit types to provide their conversion
/// factor relative to the corresponding SI base unit.
///
/// # Examples
///
/// ```rust,no_run
/// use star_sim::physics::units::*;
///
/// // SI base units have factor 1.0
/// assert_eq!(Meter::factor(), 1.0);
/// assert_eq!(Kilogram::factor(), 1.0);
///
/// // Non-SI units have their specific conversion factors
/// assert_eq!(AstronomicalUnit::factor(), METERS_PER_AU);
/// ```
pub trait UnitFactor {
    /// Returns the conversion factor from this unit to the corresponding SI base unit.
    fn factor() -> f64;
}

// ================================================================================================
// BASE UNIT IMPLEMENTATIONS
// ================================================================================================

// SI base units - these are the reference, so factor = 1.0
impl UnitFactor for Meter {
    fn factor() -> f64 { 1.0 }
}

// Note: Kilogram is actually Prefixed<Kilo, Gram>, so handled by generic implementation

impl UnitFactor for Second {
    fn factor() -> f64 { 1.0 }
}

impl UnitFactor for Kelvin {
    fn factor() -> f64 { 1.0 }
}

impl UnitFactor for Ampere {
    fn factor() -> f64 { 1.0 }
}

// Note: Candela and Mole not currently defined in the quantities module

// ================================================================================================
// DISTANCE UNITS
// ================================================================================================

impl UnitFactor for AstronomicalUnit {
    fn factor() -> f64 { METERS_PER_AU }
}

impl UnitFactor for EarthRadius {
    fn factor() -> f64 { METERS_PER_EARTH_RADIUS }
}

impl UnitFactor for SunRadius {
    fn factor() -> f64 { METERS_PER_SUN_RADIUS }
}

impl UnitFactor for LightYear {
    fn factor() -> f64 { METERS_PER_LIGHT_YEAR }
}

impl UnitFactor for Parsec {
    fn factor() -> f64 { METERS_PER_PARSEC }
}

// ================================================================================================
// MASS UNITS
// ================================================================================================

impl UnitFactor for Gram {
    fn factor() -> f64 { 0.001 } // 1 gram = 0.001 kg
}

impl UnitFactor for SolarMass {
    fn factor() -> f64 { KG_PER_SOLAR_MASS }
}

impl UnitFactor for EarthMass {
    fn factor() -> f64 { KG_PER_EARTH_MASS }
}

// Note: JupiterMass not currently defined in the quantities module

// ================================================================================================
// TIME UNITS
// ================================================================================================

impl UnitFactor for Minute {
    fn factor() -> f64 { 60.0 }
}

impl UnitFactor for Hour {
    fn factor() -> f64 { 3600.0 }
}

impl UnitFactor for Day {
    fn factor() -> f64 { SECONDS_PER_DAY }
}

impl UnitFactor for Year {
    fn factor() -> f64 { SECONDS_PER_YEAR }
}

// ================================================================================================
// TEMPERATURE UNITS
// ================================================================================================

// Note: Celsius not currently defined in the quantities module

// ================================================================================================
// ANGLE UNITS  
// ================================================================================================

impl UnitFactor for Radian {
    fn factor() -> f64 { 1.0 }
}

impl UnitFactor for Degree {
    fn factor() -> f64 { std::f64::consts::PI / 180.0 }
}

impl UnitFactor for Arcsecond {
    fn factor() -> f64 { RADIANS_PER_DEGREE / 3600.0 }
}

// ================================================================================================
// PREFIXED UNIT IMPLEMENTATION
// ================================================================================================

/// Automatic UnitFactor implementation for prefixed units.
///
/// This implementation automatically combines the prefix factor with the base unit factor:
/// `factor = prefix_factor * base_unit_factor`
///
/// # Examples
///
/// ```rust,no_run
/// use star_sim::physics::units::*;
///
/// // Kilometer = 1000 * Meter
/// assert_eq!(Prefixed::<Kilo, Meter>::factor(), 1000.0 * 1.0);
///
/// // Millimeter = 0.001 * Meter  
/// assert_eq!(Prefixed::<Milli, Meter>::factor(), 0.001 * 1.0);
/// ```
impl<P: Prefix, U: UnitFactor> UnitFactor for Prefixed<P, U> {
    fn factor() -> f64 {
        P::FACTOR * U::factor()
    }
}

// ================================================================================================
// UNIT TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_si_base_units() {
        assert_eq!(Meter::factor(), 1.0);
        assert_eq!(Kilogram::factor(), 1.0);
        assert_eq!(Second::factor(), 1.0);
        assert_eq!(Kelvin::factor(), 1.0);
    }

    #[test]
    fn test_prefixed_units() {
        // Test kilometer
        assert_eq!(Prefixed::<Kilo, Meter>::factor(), 1000.0);
        
        // Test millimeter
        assert_eq!(Prefixed::<Milli, Meter>::factor(), 0.001);
        
        // Test kilogram (note: this is actually the base unit, not prefixed)
        assert_eq!(Prefixed::<Kilo, Gram>::factor(), 1.0);
    }

    #[test]
    fn test_distance_units() {
        assert!(AstronomicalUnit::factor() > 1e11); // AU is very large in meters
        assert!(Parsec::factor() > LightYear::factor()); // Parsec > light year
    }

    #[test]
    fn test_time_units() {
        assert_eq!(Minute::factor(), 60.0);
        assert_eq!(Hour::factor(), 3600.0);
        assert_eq!(Day::factor(), 24.0 * 3600.0);
    }
}