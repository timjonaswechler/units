//! Atomic and nuclear physics constants
//!
//! Constants related to atomic structure, nuclear physics, and quantum mechanics.

use crate::core::Quantity;
use crate::core::composition::{Per, Exponent};
use crate::units::base::*;
use crate::units::derived::*;
use crate::units::Kilogram;

/// Bohr radius (a₀)
/// 
/// The most probable distance between the nucleus and electron in a hydrogen atom.
/// 
/// **Value**: 5.29177210903×10⁻¹¹ m
/// **Dimensions**: L¹
pub const BOHR_RADIUS: Quantity<Meter> = Quantity::new(5.29177210903e-11);

/// Classical electron radius (rₑ)
/// 
/// **Value**: 2.8179403262×10⁻¹⁵ m
/// **Dimensions**: L¹
pub const CLASSICAL_ELECTRON_RADIUS: Quantity<Meter> = Quantity::new(2.8179403262e-15);

/// Compton wavelength (λc)
/// 
/// **Value**: 2.42631023867×10⁻¹² m
/// **Dimensions**: L¹
pub const COMPTON_WAVELENGTH: Quantity<Meter> = Quantity::new(2.42631023867e-12);

/// Rydberg constant (R∞)
/// 
/// **Value**: 1.0973731568160×10⁷ m⁻¹
/// **Dimensions**: L⁻¹
pub const RYDBERG_CONSTANT: Quantity<Per<Meter>> = Quantity::new(1.0973731568160e7);

/// Atomic mass unit (u)
/// 
/// One twelfth of the mass of a carbon-12 atom.
/// 
/// **Value**: 1.66053906660×10⁻²⁷ kg
/// **Dimensions**: M¹
pub const ATOMIC_MASS_UNIT: Quantity<Kilogram> = Quantity::new(1.66053906660e-27);

/// Nuclear magneton (μN)
/// 
/// **Value**: 5.0507837461×10⁻²⁷ J/T
/// **Dimensions**: L²M¹T⁻²I⁻¹ (magnetic moment)
pub const NUCLEAR_MAGNETON: Quantity<(Joule, Per<Tesla>)> = Quantity::new(5.0507837461e-27);

/// Bohr magneton (μB)
/// 
/// **Value**: 9.2740100783×10⁻²⁴ J/T
/// **Dimensions**: L²M¹T⁻²I⁻¹ (magnetic moment)
pub const BOHR_MAGNETON: Quantity<(Joule, Per<Tesla>)> = Quantity::new(9.2740100783e-24);