//! Electromagnetic constants
//!
//! Constants related to electromagnetic phenomena

use crate::prelude::*;

/// Electric constant (permittivity of vacuum): ε₀ = 8.8541878128×10⁻¹² F/m
pub const ELECTRIC_CONSTANT: f64 = 8.8541878128e-12;

/// Magnetic constant (permeability of vacuum): μ₀ = 4π×10⁻⁷ H/m (exact)
pub const MAGNETIC_CONSTANT: f64 = 4.0e-7 * std::f64::consts::PI;

/// Impedance of free space: Z₀ = √(μ₀/ε₀) = 376.730313668 Ω
pub const IMPEDANCE_OF_FREE_SPACE: f64 = 376.730313668;

/// Conductance quantum: G₀ = 2e²/h = 7.748091729×10⁻⁵ S
pub const CONDUCTANCE_QUANTUM: f64 = 7.748091729e-5;

/// Resistance quantum: R_K = h/e² = 25812.80745 Ω
pub const RESISTANCE_QUANTUM: f64 = 25812.80745;

/// Magnetic flux quantum: Φ₀ = h/(2e) = 2.067833848×10⁻¹⁵ Wb
pub const MAGNETIC_FLUX_QUANTUM: f64 = 2.067833848e-15;

/// Josephson constant: K_J = 2e/h = 4.835978484×10¹⁴ Hz/V
pub const JOSEPHSON_CONSTANT: f64 = 4.835978484e14;

/// von Klitzing constant: R_K = h/e² = 25812.80745 Ω
pub const VON_KLITZING_CONSTANT: f64 = 25812.80745;

/// Bohr magneton: μB = eℏ/(2mₑ) = 9.2740102×10⁻²⁴ J/T
pub const BOHR_MAGNETON: f64 = 9.2740102e-24;

/// Nuclear magneton: μₙ = eℏ/(2mₚ) = 5.0507837461×10⁻²⁷ J/T
pub const NUCLEAR_MAGNETON: f64 = 5.0507837461e-27;

/// Electron g-factor: gₑ = -2.00231930436256
pub const ELECTRON_G_FACTOR: f64 = -2.00231930436256;

/// Proton g-factor: gₚ = 5.5856946893
pub const PROTON_G_FACTOR: f64 = 5.5856946893;

/// Neutron g-factor: gₙ = -3.82608545
pub const NEUTRON_G_FACTOR: f64 = -3.82608545;

/// Muon g-factor: gμ = -2.0023318418
pub const MUON_G_FACTOR: f64 = -2.0023318418;

/// Electron magnetic moment: μₑ = -9.2847647043×10⁻²⁴ J/T
pub const ELECTRON_MAGNETIC_MOMENT: f64 = -9.2847647043e-24;

/// Proton magnetic moment: μₚ = 1.41606797×10⁻²⁶ J/T
pub const PROTON_MAGNETIC_MOMENT: f64 = 1.41606797e-26;

/// Neutron magnetic moment: μₙ = -9.6623651×10⁻²⁷ J/T
pub const NEUTRON_MAGNETIC_MOMENT: f64 = -9.6623651e-27;

/// Muon magnetic moment: μμ = -4.4904477×10⁻²⁶ J/T
pub const MUON_MAGNETIC_MOMENT: f64 = -4.4904477e-26;

/// Fine structure constant: α = e²/(4πε₀ℏc) = 7.2973525693×10⁻³
pub const FINE_STRUCTURE_CONSTANT: f64 = 7.2973525693e-3;

/// Inverse fine structure constant: α⁻¹ = 137.035999084
pub const INVERSE_FINE_STRUCTURE_CONSTANT: f64 = 137.035999084;

/// Classical electron radius: rₑ = e²/(4πε₀mₑc²) = 2.8179403262×10⁻¹⁵ m
pub const CLASSICAL_ELECTRON_RADIUS: Distance<Meter> = Distance::new(2.8179403262e-15);

/// Thomson scattering cross section: σₜ = (8π/3)rₑ² = 6.6524587321×10⁻²⁹ m²
pub const THOMSON_CROSS_SECTION: Area<SquareMeter> = Area::new(6.6524587321e-29);

/// Electron charge-to-mass ratio: e/mₑ = -1.75882001076×10¹¹ C/kg
pub const ELECTRON_CHARGE_TO_MASS_RATIO: f64 = -1.75882001076e11;

/// Proton charge-to-mass ratio: e/mₚ = 9.5788332×10⁷ C/kg
pub const PROTON_CHARGE_TO_MASS_RATIO: f64 = 9.5788332e7;

/// Electron cyclotron frequency: ωc = eB/mₑ (for B = 1 T) = 1.75882001076×10¹¹ rad/(s⋅T)
pub const ELECTRON_CYCLOTRON_FREQUENCY_PER_TESLA: f64 = 1.75882001076e11;

/// Proton cyclotron frequency: ωc = eB/mₚ (for B = 1 T) = 9.5788332×10⁷ rad/(s⋅T)
pub const PROTON_CYCLOTRON_FREQUENCY_PER_TESLA: f64 = 9.5788332e7;

/// Larmor frequency: ωL = eB/(2m) (for electron, B = 1 T) = 8.794100×10¹⁰ rad/(s⋅T)
pub const ELECTRON_LARMOR_FREQUENCY_PER_TESLA: f64 = 8.794100e10;

/// Compton wavelength shift: Δλ = (h/mₑc)(1 - cos θ) for θ = 90°: 2.42631023867×10⁻¹² m
pub const COMPTON_WAVELENGTH_SHIFT_90_DEG: Distance<Meter> = Distance::new(2.42631023867e-12);

/// Quantum of circulation: h/(2mₑ) = 3.6369475516×10⁻⁴ m²/s
pub const QUANTUM_OF_CIRCULATION: f64 = 3.6369475516e-4;

/// Faraday constant: F = NAe = 96485.33212 C/mol
pub const FARADAY_CONSTANT: f64 = 96485.33212;

/// Elementary charge squared: e² = 2.566969782×10⁻³⁸ C²
pub const ELEMENTARY_CHARGE_SQUARED: f64 = 2.566969782e-38;

/// Vacuum wavelength of sodium D line: λD = 589.29 nm
pub const SODIUM_D_LINE_WAVELENGTH: Distance<Meter> = Distance::new(589.29e-9);

/// Vacuum wavelength of hydrogen Lyman-α: λLyα = 121.567 nm
pub const HYDROGEN_LYMAN_ALPHA_WAVELENGTH: Distance<Meter> = Distance::new(121.567e-9);

/// Vacuum wavelength of hydrogen Balmer-α (Hα): λHα = 656.281 nm
pub const HYDROGEN_BALMER_ALPHA_WAVELENGTH: Distance<Meter> = Distance::new(656.281e-9);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::fundamental::*;

    #[test]
    fn test_impedance_of_free_space() {
        let calculated = (MAGNETIC_CONSTANT / ELECTRIC_CONSTANT).sqrt();
        assert!((IMPEDANCE_OF_FREE_SPACE - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_conductance_quantum() {
        // G₀ = 2e²/h
        let e = ELEMENTARY_CHARGE.value();
        let h = PLANCK_CONSTANT;
        let calculated = 2.0 * e.powi(2) / h;
        assert!((CONDUCTANCE_QUANTUM - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_resistance_quantum() {
        // R_K = h/e²
        let e = ELEMENTARY_CHARGE.value();
        let h = PLANCK_CONSTANT;
        let calculated = h / e.powi(2);
        assert!((RESISTANCE_QUANTUM - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_magnetic_flux_quantum() {
        // Φ₀ = h/(2e)
        let e = ELEMENTARY_CHARGE.value();
        let h = PLANCK_CONSTANT;
        let calculated = h / (2.0 * e);
        assert!((MAGNETIC_FLUX_QUANTUM - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_josephson_constant() {
        // K_J = 2e/h
        let e = ELEMENTARY_CHARGE.value();
        let h = PLANCK_CONSTANT;
        let calculated = 2.0 * e / h;
        assert!((JOSEPHSON_CONSTANT - calculated).abs() / calculated < 1e-10);
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
    fn test_fine_structure_constant() {
        // α = e²/(4πε₀ℏc)
        let e = ELEMENTARY_CHARGE.value();
        let epsilon_0 = ELECTRIC_CONSTANT;
        let h_bar = REDUCED_PLANCK_CONSTANT;
        let c = SPEED_OF_LIGHT.value();
        
        let calculated = e.powi(2) / (4.0 * std::f64::consts::PI * epsilon_0 * h_bar * c);
        assert!((FINE_STRUCTURE_CONSTANT - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_inverse_fine_structure() {
        let product = FINE_STRUCTURE_CONSTANT * INVERSE_FINE_STRUCTURE_CONSTANT;
        assert!((product - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_classical_electron_radius() {
        // rₑ = e²/(4πε₀mₑc²)
        let e = ELEMENTARY_CHARGE.value();
        let epsilon_0 = ELECTRIC_CONSTANT;
        let m_e = ELECTRON_MASS.value();
        let c = SPEED_OF_LIGHT.value();
        
        let calculated = e.powi(2) / (4.0 * std::f64::consts::PI * epsilon_0 * m_e * c.powi(2));
        assert!((CLASSICAL_ELECTRON_RADIUS.value() - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_thomson_cross_section() {
        // σₜ = (8π/3)rₑ²
        let r_e = CLASSICAL_ELECTRON_RADIUS.value();
        let calculated = (8.0 * std::f64::consts::PI / 3.0) * r_e.powi(2);
        assert!((THOMSON_CROSS_SECTION.value() - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_electron_charge_to_mass_ratio() {
        // e/mₑ
        let e = ELEMENTARY_CHARGE.value();
        let m_e = ELECTRON_MASS.value();
        let calculated = -e / m_e; // Negative because electron has negative charge
        assert!((ELECTRON_CHARGE_TO_MASS_RATIO - calculated).abs() / calculated.abs() < 1e-10);
    }

    #[test]
    fn test_faraday_constant() {
        // F = NAe
        let na = AVOGADRO_CONSTANT;
        let e = ELEMENTARY_CHARGE.value();
        let calculated = na * e;
        assert!((FARADAY_CONSTANT - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_quantum_of_circulation() {
        // h/(2mₑ)
        let h = PLANCK_CONSTANT;
        let m_e = ELECTRON_MASS.value();
        let calculated = h / (2.0 * m_e);
        assert!((QUANTUM_OF_CIRCULATION - calculated).abs() / calculated < 1e-10);
    }
}