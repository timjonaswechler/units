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

/// Meter - SI base unit of length
/// 
/// Dimensional signature: L¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meter;

impl DimensionExtractor for Meter {
    const L: i8 = 1;
}

/// Kilogram - SI base unit of mass
/// 
/// Dimensional signature: M¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gram;

impl DimensionExtractor for Kilogram {
    const M: i8 = 1;
}

/// Second - SI base unit of time
/// 
/// Dimensional signature: T¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Second;

impl DimensionExtractor for Second {
    const T: i8 = 1;
}

/// Kelvin - SI base unit of temperature
/// 
/// Dimensional signature: Θ¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kelvin;

impl DimensionExtractor for Kelvin {
    const THETA: i8 = 1;
}

/// Ampere - SI base unit of electric current
/// 
/// Dimensional signature: I¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ampere;

impl DimensionExtractor for Ampere {
    const I: i8 = 1;
}

/// Candela - SI base unit of luminous intensity
/// 
/// Dimensional signature: J¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Candela;

impl DimensionExtractor for Candela {
    const J: i8 = 1;
}

/// Mole - SI base unit of amount of substance
/// 
/// Dimensional signature: N¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mole;

impl DimensionExtractor for Mole {
    const N: i8 = 1;
}