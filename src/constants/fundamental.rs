//! Fundamental physical constants
//!
//! Core constants of physics with proper dimensional types using the new compositional syntax.
//! All constants are defined with exact CODATA 2018 values where applicable.

use crate::core::{Quantity, DimensionExtractor};
use crate::core::composition::{Per, Exponent, DimensionlessUnit};
use crate::units::base::*;
use crate::units::derived::*;
use crate::units::Kilogram;

// ================================================================================================
// FUNDAMENTAL CONSTANTS - The pillars of physics
// ================================================================================================

/// Planck constant (h)
/// 
/// The quantum of electromagnetic action, relating energy and frequency.
/// 
/// **Value**: 6.62607015×10⁻³⁴ J⋅s (exact, defining constant)
/// **Dimensions**: L²M¹T⁻¹ (action)
/// 
/// # Usage in Physics
/// - Quantum mechanics: E = hν
/// - Photon energy: E = hc/λ
/// - Uncertainty principle: ΔE⋅Δt ≥ ℏ/2
pub const PLANCK_CONSTANT: Quantity<(Joule, Second)> = Quantity::new(6.62607015e-34);

/// Reduced Planck constant (ℏ = h/2π)
/// 
/// **Value**: 1.054571817×10⁻³⁴ J⋅s
/// **Dimensions**: L²M¹T⁻¹ (action)
pub const REDUCED_PLANCK_CONSTANT: Quantity<(Joule, Second)> = Quantity::new(1.054571817e-34);

/// Speed of light in vacuum (c)
/// 
/// The fundamental constant of spacetime, defining the relationship between space and time.
/// 
/// **Value**: 299,792,458 m/s (exact, defining constant)
/// **Dimensions**: L¹T⁻¹ (velocity)
/// 
/// # Usage in Physics
/// - Relativity: E = mc²
/// - Electromagnetic waves: c = λν
/// - Spacetime geometry
pub const SPEED_OF_LIGHT: Quantity<(Meter, Per<Second>)> = Quantity::new(299_792_458.0);

/// Elementary charge (e)
/// 
/// The electric charge carried by a single proton.
/// 
/// **Value**: 1.602176634×10⁻¹⁹ C (exact, defining constant)
/// **Dimensions**: IT¹ (charge)
pub const ELEMENTARY_CHARGE: Quantity<Coulomb> = Quantity::new(1.602176634e-19);

/// Electron rest mass (mₑ)
/// 
/// **Value**: 9.1093837015×10⁻³¹ kg
/// **Dimensions**: M¹ (mass)
pub const ELECTRON_MASS: Quantity<Kilogram> = Quantity::new(9.1093837015e-31);

/// Proton rest mass (mₚ)
/// 
/// **Value**: 1.67262192369×10⁻²⁷ kg  
/// **Dimensions**: M¹ (mass)
pub const PROTON_MASS: Quantity<Kilogram> = Quantity::new(1.67262192369e-27);

/// Neutron rest mass (mₙ)
/// 
/// **Value**: 1.67492749804×10⁻²⁷ kg
/// **Dimensions**: M¹ (mass)
pub const NEUTRON_MASS: Quantity<Kilogram> = Quantity::new(1.67492749804e-27);

// ================================================================================================
// THERMODYNAMIC CONSTANTS
// ================================================================================================

/// Boltzmann constant (k)
/// 
/// Relates the average kinetic energy of particles to temperature.
/// 
/// **Value**: 1.380649×10⁻²³ J/K (exact, defining constant)
/// **Dimensions**: L²M¹T⁻²Θ⁻¹ (energy per temperature)
/// 
/// # Usage in Physics
/// - Kinetic theory: ⟨E⟩ = (3/2)kT
/// - Entropy: S = k ln(Ω)
/// - Maxwell-Boltzmann distribution
pub const BOLTZMANN_CONSTANT: Quantity<(Joule, Per<Kelvin>)> = Quantity::new(1.380649e-23);

/// Stefan-Boltzmann constant (σ)
/// 
/// **The constant that was impossible with the old system!**
/// Relates the power radiated by a black body to its temperature.
/// 
/// **Value**: 5.670374419×10⁻⁸ W⋅m⁻²⋅K⁻⁴
/// **Dimensions**: M¹T⁻³Θ⁻⁴ (power per area per temperature⁴)
/// 
/// # Usage in Physics
/// - Stefan-Boltzmann law: P = σAT⁴
/// - Black body radiation
/// - Stellar luminosity calculations
pub const STEFAN_BOLTZMANN_CONSTANT: Quantity<(Watt, Per<Exponent<Meter, 2>>, Per<Exponent<Kelvin, 4>>)> = 
    Quantity::new(5.670374419e-8);

/// Avogadro constant (Nₐ)
/// 
/// The number of constituent particles in one mole.
/// 
/// **Value**: 6.02214076×10²³ mol⁻¹ (exact, defining constant)
/// **Dimensions**: N⁻¹ (per amount of substance)
pub const AVOGADRO_CONSTANT: Quantity<Per<Mole>> = Quantity::new(6.02214076e23);

/// Gas constant (R)
/// 
/// **Value**: 8.314462618 J⋅mol⁻¹⋅K⁻¹
/// **Dimensions**: L²M¹T⁻²N⁻¹Θ⁻¹
pub const GAS_CONSTANT: Quantity<(Joule, Per<Mole>, Per<Kelvin>)> = Quantity::new(8.314462618);

// ================================================================================================
// ELECTROMAGNETIC CONSTANTS
// ================================================================================================

/// Vacuum permeability (μ₀)
/// 
/// **Value**: 1.25663706212×10⁻⁶ H/m  
/// **Dimensions**: L¹M¹T⁻²I⁻² (inductance per length)
pub const VACUUM_PERMEABILITY: Quantity<(Henry, Per<Meter>)> = Quantity::new(1.25663706212e-6);

/// Vacuum permittivity (ε₀)
/// 
/// **Value**: 8.8541878128×10⁻¹² F/m
/// **Dimensions**: L⁻³M⁻¹T⁴I² (capacitance per length)
pub const VACUUM_PERMITTIVITY: Quantity<(Farad, Per<Meter>)> = Quantity::new(8.8541878128e-12);

/// Fine structure constant (α)
/// 
/// The dimensionless coupling constant characterizing the strength of electromagnetic interaction.
/// 
/// **Value**: 7.2973525693×10⁻³ (dimensionless)
/// **Dimensions**: dimensionless
pub const FINE_STRUCTURE_CONSTANT: Quantity<DimensionlessUnit> = Quantity::new(7.2973525693e-3);

/// Magnetic flux quantum (Φ₀)
/// 
/// **Value**: 2.067833848×10⁻¹⁵ Wb
/// **Dimensions**: L²M¹T⁻²I⁻¹ (magnetic flux)
pub const MAGNETIC_FLUX_QUANTUM: Quantity<Weber> = Quantity::new(2.067833848e-15);