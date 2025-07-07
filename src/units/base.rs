//! SI base units
//!
//! The seven SI base units that form the foundation of all physical measurements:
//! - Meter (length)
//! - Kilogram (mass)
//! - Second (time)
//! - Kelvin (temperature)
//! - Ampere (electric current)
//! - Candela (luminous intensity)
//! - Mole (amount of substance)

use crate::core::DimensionExtractor;
use crate::arithmetic::conversion::UnitConverter;
use crate::units::{Kilo, Prefixed};
use std::ops::{Mul, Div};

/// Meter - SI base unit of length
///
/// Dimensional signature: L¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meter;

impl DimensionExtractor for Meter {
    const L: i8 = 1;
}

impl UnitConverter for Meter {
    fn to_base_units<V>(value: V) -> V 
    where 
        V: Mul<f64, Output = V>
    {
        value  // Meter is the base unit - no conversion
    }
    
    fn from_base_units<V>(value: V) -> V 
    where 
        V: Div<f64, Output = V>
    {
        value  // Meter is the base unit - no conversion
    }
    
    fn conversion_factor() -> f64 {
        1.0
    }
}

/// Gram - true base unit of mass
///
/// Dimensional signature: M¹
/// Note: In SI, kilogram is the official base unit, but mathematically
/// gram is more fundamental (kilogram = kilo + gram)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gram;

/// Kilogram - SI base unit of mass (1000 grams)
/// This is the official SI base unit, implemented as kilo + gram
pub type Kilogram = Prefixed<Kilo, Gram>;

impl DimensionExtractor for Gram {
    const M: i8 = 1;
}

impl UnitConverter for Gram {
    fn to_base_units<V>(value: V) -> V 
    where 
        V: Mul<f64, Output = V>
    {
        value  // Gram is the base unit - no conversion
    }
    
    fn from_base_units<V>(value: V) -> V 
    where 
        V: Div<f64, Output = V>
    {
        value  // Gram is the base unit - no conversion
    }
    
    fn conversion_factor() -> f64 {
        1.0
    }
}

/// Second - SI base unit of time
///
/// Dimensional signature: T¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Second;

impl DimensionExtractor for Second {
    const T: i8 = 1;
}

impl UnitConverter for Second {
    fn to_base_units<V>(value: V) -> V 
    where 
        V: Mul<f64, Output = V>
    {
        value  // Second is the base unit - no conversion
    }
    
    fn from_base_units<V>(value: V) -> V 
    where 
        V: Div<f64, Output = V>
    {
        value  // Second is the base unit - no conversion
    }
    
    fn conversion_factor() -> f64 {
        1.0
    }
}

/// Kelvin - SI base unit of temperature
///
/// Dimensional signature: Θ¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kelvin;

impl DimensionExtractor for Kelvin {
    const THETA: i8 = 1;
}

impl UnitConverter for Kelvin {
    fn to_base_units<V>(value: V) -> V 
    where 
        V: Mul<f64, Output = V>
    {
        value  // Kelvin is the base unit - no conversion
    }
    
    fn from_base_units<V>(value: V) -> V 
    where 
        V: Div<f64, Output = V>
    {
        value  // Kelvin is the base unit - no conversion
    }
    
    fn conversion_factor() -> f64 {
        1.0
    }
}

/// Ampere - SI base unit of electric current
///
/// Dimensional signature: I¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ampere;

impl DimensionExtractor for Ampere {
    const I: i8 = 1;
}

impl UnitConverter for Ampere {
    fn to_base_units<V>(value: V) -> V 
    where 
        V: Mul<f64, Output = V>
    {
        value  // Ampere is the base unit - no conversion
    }
    
    fn from_base_units<V>(value: V) -> V 
    where 
        V: Div<f64, Output = V>
    {
        value  // Ampere is the base unit - no conversion
    }
    
    fn conversion_factor() -> f64 {
        1.0
    }
}

/// Candela - SI base unit of luminous intensity
///
/// Dimensional signature: J¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Candela;

impl DimensionExtractor for Candela {
    const J: i8 = 1;
}

impl UnitConverter for Candela {
    fn to_base_units<V>(value: V) -> V 
    where 
        V: Mul<f64, Output = V>
    {
        value  // Candela is the base unit - no conversion
    }
    
    fn from_base_units<V>(value: V) -> V 
    where 
        V: Div<f64, Output = V>
    {
        value  // Candela is the base unit - no conversion
    }
    
    fn conversion_factor() -> f64 {
        1.0
    }
}

/// Mole - SI base unit of amount of substance
///
/// Dimensional signature: N¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mole;

impl DimensionExtractor for Mole {
    const N: i8 = 1;
}

impl UnitConverter for Mole {
    fn to_base_units<V>(value: V) -> V 
    where 
        V: Mul<f64, Output = V>
    {
        value  // Mole is the base unit - no conversion
    }
    
    fn from_base_units<V>(value: V) -> V 
    where 
        V: Div<f64, Output = V>
    {
        value  // Mole is the base unit - no conversion
    }
    
    fn conversion_factor() -> f64 {
        1.0
    }
}
