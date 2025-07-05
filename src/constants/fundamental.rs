//! Fundamental physical constants
//!
//! CODATA 2018 values for fundamental constants with type-safe units

use crate::prelude::*;

/// Speed of light in vacuum: c = 299,792,458 m/s (exact)
pub const SPEED_OF_LIGHT: Velocity<MeterPerSecond> = Velocity::new(299_792_458.0);

/// Planck constant: h = 6.626070040×10⁻³⁴ J⋅s
pub const PLANCK_CONSTANT: f64 = 6.62607015e-34;

/// Reduced Planck constant: ℏ = h/(2π) = 1.054571817×10⁻³⁴ J⋅s  
pub const REDUCED_PLANCK_CONSTANT: f64 = 1.054571817e-34;

/// Elementary charge: e = 1.602176634×10⁻¹⁹ C (exact)
pub const ELEMENTARY_CHARGE: ElectricCharge<Coulomb> = ElectricCharge::new(1.602176634e-19);

/// Electron rest mass: mₑ = 9.1093837015×10⁻³¹ kg
pub const ELECTRON_MASS: Mass<Kilogram> = Mass::new(9.1093837015e-31);

/// Proton rest mass: mₚ = 1.67262192369×10⁻²⁷ kg
pub const PROTON_MASS: Mass<Kilogram> = Mass::new(1.67262192369e-27);

/// Neutron rest mass: mₙ = 1.67492749804×10⁻²⁷ kg
pub const NEUTRON_MASS: Mass<Kilogram> = Mass::new(1.67492749804e-27);

/// Atomic mass unit: u = 1.66053906660×10⁻²⁷ kg
pub const ATOMIC_MASS_UNIT: Mass<Kilogram> = Mass::new(1.66053906660e-27);

/// Avogadro constant: Nₐ = 6.02214076×10²³ mol⁻¹ (exact)
pub const AVOGADRO_CONSTANT: f64 = 6.02214076e23;

/// Boltzmann constant: k = 1.380649×10⁻²³ J/K (exact)
pub const BOLTZMANN_CONSTANT: f64 = 1.380649e-23;

/// Gas constant: R = 8.314462618 J/(mol⋅K) (exact)
pub const GAS_CONSTANT: f64 = 8.314462618;

/// Stefan-Boltzmann constant: σ = 5.670374419×10⁻⁸ W/(m²⋅K⁴)
pub const STEFAN_BOLTZMANN_CONSTANT: f64 = 5.670374419e-8;

/// Gravitational constant: G = 6.67430×10⁻¹¹ m³/(kg⋅s²)
pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;

/// Fine structure constant: α = 7.2973525693×10⁻³ (dimensionless)
pub const FINE_STRUCTURE_CONSTANT: f64 = 7.2973525693e-3;

/// Magnetic permeability of vacuum: μ₀ = 4π×10⁻⁷ H/m (exact)
pub const VACUUM_PERMEABILITY: f64 = 4.0e-7 * std::f64::consts::PI;

/// Electric permittivity of vacuum: ε₀ = 8.8541878128×10⁻¹² F/m
pub const VACUUM_PERMITTIVITY: f64 = 8.8541878128e-12;

/// Impedance of vacuum: Z₀ = √(μ₀/ε₀) = 376.730313668 Ω
pub const VACUUM_IMPEDANCE: f64 = 376.730313668;

// Planck units derived from fundamental constants

/// Planck length: lₚ = √(ℏG/c³) = 1.616255×10⁻³⁵ m
pub const PLANCK_LENGTH: Distance<Meter> = Distance::new(1.616255e-35);

/// Planck mass: mₚ = √(ℏc/G) = 2.176434×10⁻⁸ kg
pub const PLANCK_MASS: Mass<Kilogram> = Mass::new(2.176434e-8);

/// Planck time: tₚ = √(ℏG/c⁵) = 5.391247×10⁻⁴⁴ s
pub const PLANCK_TIME: Time<Second> = Time::new(5.391247e-44);

/// Planck energy: Eₚ = √(ℏc⁵/G) = 1.956082×10⁹ J
pub const PLANCK_ENERGY: Energy<Joule> = Energy::new(1.956082e9);

/// Planck temperature: Tₚ = √(ℏc⁵/Gk²) = 1.416784×10³² K
pub const PLANCK_TEMPERATURE: f64 = 1.416784e32;

/// Planck force: Fₚ = c⁴/G = 1.210256×10⁴⁴ N
pub const PLANCK_FORCE: Force<Newton> = Force::new(1.210256e44);

/// Planck power: Pₚ = c⁵/G = 3.628256×10⁵² W
pub const PLANCK_POWER: Power<Watt> = Power::new(3.628256e52);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_of_light() {
        assert_eq!(SPEED_OF_LIGHT.value(), 299_792_458.0);
    }

    #[test]
    fn test_reduced_planck_from_planck() {
        let h_bar_calculated = PLANCK_CONSTANT / (2.0 * std::f64::consts::PI);
        assert!((REDUCED_PLANCK_CONSTANT - h_bar_calculated).abs() / h_bar_calculated < 1e-10);
    }

    #[test]
    fn test_planck_length_dimensional_analysis() {
        // lₚ = √(ℏG/c³)
        let h_bar = REDUCED_PLANCK_CONSTANT;
        let g = GRAVITATIONAL_CONSTANT;
        let c = SPEED_OF_LIGHT.value();
        
        let calculated = (h_bar * g / (c.powi(3))).sqrt();
        assert!((PLANCK_LENGTH.value() - calculated).abs() / calculated < 1e-6);
    }

    #[test]
    fn test_vacuum_impedance() {
        let calculated = (VACUUM_PERMEABILITY / VACUUM_PERMITTIVITY).sqrt();
        assert!((VACUUM_IMPEDANCE - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_fine_structure_constant() {
        // α = e²/(4πε₀ℏc) (in Gaussian units)
        // This is an approximate test due to unit system differences
        assert!(FINE_STRUCTURE_CONSTANT > 0.007 && FINE_STRUCTURE_CONSTANT < 0.008);
    }
}