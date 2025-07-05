//! Atomic and nuclear constants
//!
//! Constants related to atomic and nuclear physics

use crate::prelude::*;

/// Bohr radius: a₀ = 4πε₀ℏ²/(mₑe²) = 5.29177210903×10⁻¹¹ m
pub const BOHR_RADIUS: Distance<Meter> = Distance::new(5.29177210903e-11);

/// Classical electron radius: rₑ = e²/(4πε₀mₑc²) = 2.8179403262×10⁻¹⁵ m
pub const CLASSICAL_ELECTRON_RADIUS: Distance<Meter> = Distance::new(2.8179403262e-15);

/// Compton wavelength of electron: λc = h/(mₑc) = 2.42631023867×10⁻¹² m
pub const ELECTRON_COMPTON_WAVELENGTH: Distance<Meter> = Distance::new(2.42631023867e-12);

/// Reduced Compton wavelength: λ̄c = ℏ/(mₑc) = 3.8615926796×10⁻¹³ m
pub const REDUCED_ELECTRON_COMPTON_WAVELENGTH: Distance<Meter> = Distance::new(3.8615926796e-13);

/// Proton Compton wavelength: λc,p = h/(mₚc) = 1.32140985539×10⁻¹⁵ m
pub const PROTON_COMPTON_WAVELENGTH: Distance<Meter> = Distance::new(1.32140985539e-15);

/// Neutron Compton wavelength: λc,n = h/(mₙc) = 1.31959090581×10⁻¹⁵ m
pub const NEUTRON_COMPTON_WAVELENGTH: Distance<Meter> = Distance::new(1.31959090581e-15);

/// Rydberg constant: R∞ = mₑe⁴/(8ε₀²h³c) = 1.0973731568160×10⁷ m⁻¹
pub const RYDBERG_CONSTANT: f64 = 1.0973731568160e7;

/// Hartree energy: Eₕ = 2R∞hc = 4.3597447222071×10⁻¹⁸ J
pub const HARTREE_ENERGY: Energy<Joule> = Energy::new(4.3597447222071e-18);

/// Electron volt: 1 eV = 1.602176634×10⁻¹⁹ J (exact)
pub const ELECTRON_VOLT: Energy<Joule> = Energy::new(1.602176634e-19);

/// Atomic mass constant: mᵤ = 1.66053906660×10⁻²⁷ kg
pub const ATOMIC_MASS_CONSTANT: Mass<Kilogram> = Mass::new(1.66053906660e-27);

/// Proton-electron mass ratio: mₚ/mₑ = 1836.15267343
pub const PROTON_ELECTRON_MASS_RATIO: f64 = 1836.15267343;

/// Muon mass: mμ = 1.883531627×10⁻²⁸ kg
pub const MUON_MASS: Mass<Kilogram> = Mass::new(1.883531627e-28);

/// Tau mass: mτ = 3.16754×10⁻²⁷ kg
pub const TAU_MASS: Mass<Kilogram> = Mass::new(3.16754e-27);

/// Alpha particle mass: mα = 6.6446573357×10⁻²⁷ kg
pub const ALPHA_PARTICLE_MASS: Mass<Kilogram> = Mass::new(6.6446573357e-27);

/// Deuteron mass: m_d = 3.3435837724×10⁻²⁷ kg
pub const DEUTERON_MASS: Mass<Kilogram> = Mass::new(3.3435837724e-27);

/// Triton mass: m_t = 5.0073567446×10⁻²⁷ kg
pub const TRITON_MASS: Mass<Kilogram> = Mass::new(5.0073567446e-27);

/// Helion mass: m_h = 5.0064127796×10⁻²⁷ kg
pub const HELION_MASS: Mass<Kilogram> = Mass::new(5.0064127796e-27);

/// Electron magnetic moment: μₑ = -9.2847647043×10⁻²⁴ J/T
pub const ELECTRON_MAGNETIC_MOMENT: f64 = -9.2847647043e-24;

/// Proton magnetic moment: μₚ = 1.41606797×10⁻²⁶ J/T
pub const PROTON_MAGNETIC_MOMENT: f64 = 1.41606797e-26;

/// Neutron magnetic moment: μₙ = -9.6623651×10⁻²⁷ J/T
pub const NEUTRON_MAGNETIC_MOMENT: f64 = -9.6623651e-27;

/// Nuclear magneton: μₙ = eℏ/(2mₚ) = 5.0507837461×10⁻²⁷ J/T
pub const NUCLEAR_MAGNETON: f64 = 5.0507837461e-27;

/// Bohr magneton: μB = eℏ/(2mₑ) = 9.2740102×10⁻²⁴ J/T
pub const BOHR_MAGNETON: f64 = 9.2740102e-24;

/// Hyperfine transition frequency of Cs-133: Δν_Cs = 9,192,631,770 Hz (exact)
pub const CESIUM_FREQUENCY: Frequency<Hertz> = Frequency::new(9_192_631_770.0);

/// Hydrogen ground state binding energy: E₁ = 13.605693122994 eV
pub const HYDROGEN_BINDING_ENERGY: Energy<Joule> = Energy::new(2.1798723611035e-18);

/// Thomson scattering cross section: σₜ = (8π/3)rₑ² = 6.6524587321×10⁻²⁹ m²
pub const THOMSON_CROSS_SECTION: Area<SquareMeter> = Area::new(6.6524587321e-29);

/// Weak mixing angle (sin²θw): sin²θw ≈ 0.2229
pub const WEAK_MIXING_ANGLE: f64 = 0.2229;

/// W boson mass: M_W ≈ 80.379 GeV/c²
pub const W_BOSON_MASS_GEV: f64 = 80.379;

/// Z boson mass: M_Z ≈ 91.1876 GeV/c²
pub const Z_BOSON_MASS_GEV: f64 = 91.1876;

/// Higgs boson mass: M_H ≈ 125.25 GeV/c²
pub const HIGGS_BOSON_MASS_GEV: f64 = 125.25;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::fundamental::*;

    #[test]
    fn test_bohr_radius_calculation() {
        // a₀ = 4πε₀ℏ²/(mₑe²)
        let epsilon_0 = VACUUM_PERMITTIVITY;
        let h_bar = REDUCED_PLANCK_CONSTANT;
        let m_e = ELECTRON_MASS.value();
        let e = ELEMENTARY_CHARGE.value();
        
        let calculated = 4.0 * std::f64::consts::PI * epsilon_0 * h_bar.powi(2) / (m_e * e.powi(2));
        assert!((BOHR_RADIUS.value() - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_electron_compton_wavelength() {
        // λc = h/(mₑc)
        let h = PLANCK_CONSTANT;
        let m_e = ELECTRON_MASS.value();
        let c = SPEED_OF_LIGHT.value();
        
        let calculated = h / (m_e * c);
        assert!((ELECTRON_COMPTON_WAVELENGTH.value() - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_reduced_compton_wavelength() {
        // λ̄c = ℏ/(mₑc) = λc/(2π)
        let calculated = ELECTRON_COMPTON_WAVELENGTH.value() / (2.0 * std::f64::consts::PI);
        assert!((REDUCED_ELECTRON_COMPTON_WAVELENGTH.value() - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_hartree_energy() {
        // Eₕ = 2R∞hc
        let r_inf = RYDBERG_CONSTANT;
        let h = PLANCK_CONSTANT;
        let c = SPEED_OF_LIGHT.value();
        
        let calculated = 2.0 * r_inf * h * c;
        assert!((HARTREE_ENERGY.value() - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_proton_electron_mass_ratio() {
        let ratio = PROTON_MASS.value() / ELECTRON_MASS.value();
        assert!((ratio - PROTON_ELECTRON_MASS_RATIO).abs() / PROTON_ELECTRON_MASS_RATIO < 1e-10);
    }

    #[test]
    fn test_hydrogen_binding_energy() {
        // Should be approximately 13.6 eV
        let binding_ev = HYDROGEN_BINDING_ENERGY.value() / ELECTRON_VOLT.value();
        assert!((binding_ev - 13.605693122994).abs() < 1e-10);
    }

    #[test]
    fn test_cesium_frequency() {
        // This is the definition of the second
        assert_eq!(CESIUM_FREQUENCY.value(), 9_192_631_770.0);
    }

    #[test]
    fn test_atomic_mass_unit() {
        // Should be 1/12 of carbon-12 atom mass
        // This is an approximation test
        assert!((ATOMIC_MASS_CONSTANT.value() - 1.66053906660e-27).abs() < 1e-37);
    }

    #[test]
    fn test_nuclear_magneton() {
        // μₙ = eℏ/(2mₚ)
        let e = ELEMENTARY_CHARGE.value();
        let h_bar = REDUCED_PLANCK_CONSTANT;
        let m_p = PROTON_MASS.value();
        
        let calculated = e * h_bar / (2.0 * m_p);
        assert!((NUCLEAR_MAGNETON - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_bohr_magneton() {
        // μB = eℏ/(2mₑ)
        let e = ELEMENTARY_CHARGE.value();
        let h_bar = REDUCED_PLANCK_CONSTANT;
        let m_e = ELECTRON_MASS.value();
        
        let calculated = e * h_bar / (2.0 * m_e);
        assert!((BOHR_MAGNETON - calculated).abs() / calculated < 1e-10);
    }
}