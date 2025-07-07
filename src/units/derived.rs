//! SI derived units
//!
//! Common derived units that can be expressed in terms of the base SI units.

use crate::core::DimensionExtractor;

/// Newton - SI unit of force
/// 
/// 1 N = 1 kg⋅m⋅s⁻²
/// Dimensional signature: L¹M¹T⁻²
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Newton;

impl DimensionExtractor for Newton {
    const L: i8 = 1;  // meter
    const M: i8 = 1;  // kilogram
    const T: i8 = -2; // per second squared
}

/// Joule - SI unit of energy
/// 
/// 1 J = 1 kg⋅m²⋅s⁻²
/// Dimensional signature: L²M¹T⁻²
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Joule;

impl DimensionExtractor for Joule {
    const L: i8 = 2;  // meter squared
    const M: i8 = 1;  // kilogram
    const T: i8 = -2; // per second squared
}

/// Watt - SI unit of power
/// 
/// 1 W = 1 kg⋅m²⋅s⁻³
/// Dimensional signature: L²M¹T⁻³
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Watt;

impl DimensionExtractor for Watt {
    const L: i8 = 2;  // meter squared
    const M: i8 = 1;  // kilogram
    const T: i8 = -3; // per second cubed
}

/// Pascal - SI unit of pressure
/// 
/// 1 Pa = 1 kg⋅m⁻¹⋅s⁻²
/// Dimensional signature: L⁻¹M¹T⁻²
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pascal;

impl DimensionExtractor for Pascal {
    const L: i8 = -1; // per meter
    const M: i8 = 1;  // kilogram
    const T: i8 = -2; // per second squared
}

/// Coulomb - SI unit of electric charge
/// 
/// 1 C = 1 A⋅s
/// Dimensional signature: IT¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coulomb;

impl DimensionExtractor for Coulomb {
    const T: i8 = 1; // second
    const I: i8 = 1; // ampere
}

/// Volt - SI unit of electric potential
/// 
/// 1 V = 1 kg⋅m²⋅s⁻³⋅A⁻¹
/// Dimensional signature: L²M¹T⁻³I⁻¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Volt;

impl DimensionExtractor for Volt {
    const L: i8 = 2;  // meter squared
    const M: i8 = 1;  // kilogram
    const T: i8 = -3; // per second cubed
    const I: i8 = -1; // per ampere
}

/// Ohm - SI unit of electrical resistance
/// 
/// 1 Ω = 1 kg⋅m²⋅s⁻³⋅A⁻²
/// Dimensional signature: L²M¹T⁻³I⁻²
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ohm;

impl DimensionExtractor for Ohm {
    const L: i8 = 2;  // meter squared
    const M: i8 = 1;  // kilogram
    const T: i8 = -3; // per second cubed
    const I: i8 = -2; // per ampere squared
}

/// Farad - SI unit of capacitance
/// 
/// 1 F = 1 kg⁻¹⋅m⁻²⋅s⁴⋅A²
/// Dimensional signature: L⁻²M⁻¹T⁴I²
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Farad;

impl DimensionExtractor for Farad {
    const L: i8 = -2; // per meter squared
    const M: i8 = -1; // per kilogram
    const T: i8 = 4;  // second to the fourth
    const I: i8 = 2;  // ampere squared
}

/// Henry - SI unit of inductance
/// 
/// 1 H = 1 kg⋅m²⋅s⁻²⋅A⁻²
/// Dimensional signature: L²M¹T⁻²I⁻²
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Henry;

impl DimensionExtractor for Henry {
    const L: i8 = 2;  // meter squared
    const M: i8 = 1;  // kilogram
    const T: i8 = -2; // per second squared
    const I: i8 = -2; // per ampere squared
}

/// Weber - SI unit of magnetic flux
/// 
/// 1 Wb = 1 kg⋅m²⋅s⁻²⋅A⁻¹
/// Dimensional signature: L²M¹T⁻²I⁻¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Weber;

impl DimensionExtractor for Weber {
    const L: i8 = 2;  // meter squared
    const M: i8 = 1;  // kilogram
    const T: i8 = -2; // per second squared
    const I: i8 = -1; // per ampere
}

/// Tesla - SI unit of magnetic field strength
/// 
/// 1 T = 1 kg⋅s⁻²⋅A⁻¹
/// Dimensional signature: M¹T⁻²I⁻¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tesla;

impl DimensionExtractor for Tesla {
    const M: i8 = 1;  // kilogram
    const T: i8 = -2; // per second squared
    const I: i8 = -1; // per ampere
}

/// Hertz - SI unit of frequency
/// 
/// 1 Hz = 1 s⁻¹
/// Dimensional signature: T⁻¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hertz;

impl DimensionExtractor for Hertz {
    const T: i8 = -1; // per second
}

/// Lumen - SI unit of luminous flux
/// 
/// 1 lm = 1 cd⋅sr (candela-steradian)
/// Dimensional signature: J¹ (steradians are dimensionless)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lumen;

impl DimensionExtractor for Lumen {
    const J: i8 = 1; // candela
}

/// Lux - SI unit of illuminance
/// 
/// 1 lx = 1 lm⋅m⁻² = 1 cd⋅m⁻²
/// Dimensional signature: L⁻²J¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lux;

impl DimensionExtractor for Lux {
    const L: i8 = -2; // per meter squared
    const J: i8 = 1;  // candela
}

/// Becquerel - SI unit of radioactivity
/// 
/// 1 Bq = 1 s⁻¹
/// Dimensional signature: T⁻¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Becquerel;

impl DimensionExtractor for Becquerel {
    const T: i8 = -1; // per second
}

/// Gray - SI unit of absorbed dose
/// 
/// 1 Gy = 1 J⋅kg⁻¹ = 1 m²⋅s⁻²
/// Dimensional signature: L²T⁻²
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gray;

impl DimensionExtractor for Gray {
    const L: i8 = 2;  // meter squared
    const T: i8 = -2; // per second squared
}

/// Sievert - SI unit of equivalent dose
/// 
/// 1 Sv = 1 J⋅kg⁻¹ = 1 m²⋅s⁻²
/// Dimensional signature: L²T⁻² (same as Gray)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sievert;

impl DimensionExtractor for Sievert {
    const L: i8 = 2;  // meter squared
    const T: i8 = -2; // per second squared
}

/// Katal - SI unit of catalytic activity
/// 
/// 1 kat = 1 mol⋅s⁻¹
/// Dimensional signature: NT⁻¹
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Katal;

impl DimensionExtractor for Katal {
    const N: i8 = 1;  // mole
    const T: i8 = -1; // per second
}