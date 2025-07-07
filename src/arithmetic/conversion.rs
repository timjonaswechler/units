//! Unit conversion operations
//!
//! This module handles automatic conversion to SI base units for internal storage.
//! All quantities are normalized to base units internally, ensuring consistent arithmetic.

use std::ops::{Mul, Div};

/// Trait for units that can convert values to/from SI base units
pub trait UnitConverter {
    /// Convert a value from this unit to SI base units
    /// 
    /// For example: Kilogram converts kg → g (multiply by 1000)
    fn to_base_units<V>(value: V) -> V 
    where 
        V: Mul<f64, Output = V>;
    
    /// Convert a value from SI base units to this unit
    /// 
    /// For example: Kilogram converts g → kg (divide by 1000)
    fn from_base_units<V>(value: V) -> V 
    where 
        V: Div<f64, Output = V>;
    
    /// Get the conversion factor for this unit
    /// 
    /// Returns the factor to multiply by to convert to base units
    fn conversion_factor() -> f64;
}

// Base units implement UnitConverter with no conversion (factor = 1.0)
// Each base unit must implement this explicitly to avoid conflicts