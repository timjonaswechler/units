//! Nuclear physics constants
//!
//! Constants related to nuclear physics and radioactivity

use crate::prelude::*;

/// Strong coupling constant: αs ≈ 0.1181 (at MZ scale)
pub const STRONG_COUPLING_CONSTANT: f64 = 0.1181;

/// Weak coupling constant: αw ≈ 0.0331
pub const WEAK_COUPLING_CONSTANT: f64 = 0.0331;

/// QCD scale parameter: ΛQCD ≈ 217 MeV
pub const QCD_SCALE_MEV: f64 = 217.0;

/// Nuclear radius constant: r₀ = 1.2×10⁻¹⁵ m
pub const NUCLEAR_RADIUS_CONSTANT: Distance<Meter> = Distance::new(1.2e-15);

/// Classical nuclear radius: rN = ℏ²/(mNc²) ≈ 2.1×10⁻¹⁶ m
pub const CLASSICAL_NUCLEAR_RADIUS: Distance<Meter> = Distance::new(2.1e-16);

/// Nuclear magneton: μN = eℏ/(2mp) = 5.0507837461×10⁻²⁷ J/T
pub const NUCLEAR_MAGNETON: f64 = 5.0507837461e-27;

/// Fermi coupling constant: GF = 1.1663787×10⁻⁵ GeV⁻²
pub const FERMI_COUPLING_CONSTANT_GEV_INV2: f64 = 1.1663787e-5;

/// W boson mass: MW = 80.379 GeV/c²
pub const W_BOSON_MASS_GEV: f64 = 80.379;

/// Z boson mass: MZ = 91.1876 GeV/c²
pub const Z_BOSON_MASS_GEV: f64 = 91.1876;

/// Higgs boson mass: MH ≈ 125.25 GeV/c²
pub const HIGGS_BOSON_MASS_GEV: f64 = 125.25;

/// Top quark mass: mt ≈ 173.1 GeV/c²
pub const TOP_QUARK_MASS_GEV: f64 = 173.1;

/// Bottom quark mass: mb ≈ 4.18 GeV/c²
pub const BOTTOM_QUARK_MASS_GEV: f64 = 4.18;

/// Charm quark mass: mc ≈ 1.275 GeV/c²
pub const CHARM_QUARK_MASS_GEV: f64 = 1.275;

/// Strange quark mass: ms ≈ 95 MeV/c²
pub const STRANGE_QUARK_MASS_MEV: f64 = 95.0;

/// Down quark mass: md ≈ 4.7 MeV/c²
pub const DOWN_QUARK_MASS_MEV: f64 = 4.7;

/// Up quark mass: mu ≈ 2.2 MeV/c²
pub const UP_QUARK_MASS_MEV: f64 = 2.2;

/// Proton-neutron mass difference: Δm = mn - mp = 1.29333236×10⁻³⁰ kg
pub const PROTON_NEUTRON_MASS_DIFFERENCE: Mass<Kilogram> = Mass::new(1.29333236e-30);

/// Binding energy per nucleon (average): BE/A ≈ 8.5 MeV
pub const AVERAGE_BINDING_ENERGY_PER_NUCLEON_MEV: f64 = 8.5;

/// Alpha decay Q-value (typical): Q ≈ 5 MeV
pub const TYPICAL_ALPHA_DECAY_Q_VALUE_MEV: f64 = 5.0;

/// Beta decay Q-value (typical): Q ≈ 1 MeV
pub const TYPICAL_BETA_DECAY_Q_VALUE_MEV: f64 = 1.0;

/// Neutron lifetime: τn = 879.4 s
pub const NEUTRON_LIFETIME: Time<Second> = Time::new(879.4);

/// Muon lifetime: τμ = 2.1969811×10⁻⁶ s
pub const MUON_LIFETIME: Time<Second> = Time::new(2.1969811e-6);

/// Pion charged lifetime: τπ± = 2.6033×10⁻⁸ s
pub const PION_CHARGED_LIFETIME: Time<Second> = Time::new(2.6033e-8);

/// Pion neutral lifetime: τπ⁰ = 8.52×10⁻¹⁷ s
pub const PION_NEUTRAL_LIFETIME: Time<Second> = Time::new(8.52e-17);

/// Kaon charged lifetime: τK± = 1.2380×10⁻⁸ s
pub const KAON_CHARGED_LIFETIME: Time<Second> = Time::new(1.2380e-8);

/// Kaon short lifetime: τKS = 8.954×10⁻¹¹ s
pub const KAON_SHORT_LIFETIME: Time<Second> = Time::new(8.954e-11);

/// Kaon long lifetime: τKL = 5.116×10⁻⁸ s
pub const KAON_LONG_LIFETIME: Time<Second> = Time::new(5.116e-8);

/// Proton mass in MeV/c²: mp = 938.272088 MeV/c²
pub const PROTON_MASS_MEV: f64 = 938.272088;

/// Neutron mass in MeV/c²: mn = 939.565420 MeV/c²
pub const NEUTRON_MASS_MEV: f64 = 939.565420;

/// Electron mass in MeV/c²: me = 0.51099895000 MeV/c²
pub const ELECTRON_MASS_MEV: f64 = 0.51099895000;

/// Muon mass in MeV/c²: mμ = 105.6583745 MeV/c²
pub const MUON_MASS_MEV: f64 = 105.6583745;

/// Tau mass in MeV/c²: mτ = 1776.86 MeV/c²
pub const TAU_MASS_MEV: f64 = 1776.86;

/// Pion charged mass in MeV/c²: mπ± = 139.57039 MeV/c²
pub const PION_CHARGED_MASS_MEV: f64 = 139.57039;

/// Pion neutral mass in MeV/c²: mπ⁰ = 134.9768 MeV/c²
pub const PION_NEUTRAL_MASS_MEV: f64 = 134.9768;

/// Kaon charged mass in MeV/c²: mK± = 493.677 MeV/c²
pub const KAON_CHARGED_MASS_MEV: f64 = 493.677;

/// Kaon neutral mass in MeV/c²: mK⁰ = 497.611 MeV/c²
pub const KAON_NEUTRAL_MASS_MEV: f64 = 497.611;

/// Deuteron binding energy: BE = 2.224573 MeV
pub const DEUTERON_BINDING_ENERGY_MEV: f64 = 2.224573;

/// Tritium half-life: t₁/₂ = 12.32 years
pub const TRITIUM_HALF_LIFE_YEARS: f64 = 12.32;

/// Carbon-14 half-life: t₁/₂ = 5730 years
pub const CARBON_14_HALF_LIFE_YEARS: f64 = 5730.0;

/// Uranium-235 half-life: t₁/₂ = 703.8×10⁶ years
pub const URANIUM_235_HALF_LIFE_YEARS: f64 = 703.8e6;

/// Uranium-238 half-life: t₁/₂ = 4.468×10⁹ years
pub const URANIUM_238_HALF_LIFE_YEARS: f64 = 4.468e9;

/// Plutonium-239 half-life: t₁/₂ = 24110 years
pub const PLUTONIUM_239_HALF_LIFE_YEARS: f64 = 24110.0;

/// Critical mass of U-235 (bare sphere): Mcrit ≈ 52 kg
pub const URANIUM_235_CRITICAL_MASS: Mass<Kilogram> = Mass::new(52.0);

/// Critical mass of Pu-239 (bare sphere): Mcrit ≈ 10 kg
pub const PLUTONIUM_239_CRITICAL_MASS: Mass<Kilogram> = Mass::new(10.0);

/// Neutron absorption cross section of U-235 (thermal): σa ≈ 681 barns
pub const URANIUM_235_ABSORPTION_CROSS_SECTION_BARNS: f64 = 681.0;

/// Neutron fission cross section of U-235 (thermal): σf ≈ 585 barns
pub const URANIUM_235_FISSION_CROSS_SECTION_BARNS: f64 = 585.0;

/// Barn unit: 1 barn = 10⁻²⁴ cm² = 10⁻²⁸ m²
pub const BARN: Area<SquareMeter> = Area::new(1e-28);

/// Nuclear force range: R ≈ 1.5×10⁻¹⁵ m
pub const NUCLEAR_FORCE_RANGE: Distance<Meter> = Distance::new(1.5e-15);

/// Pion Compton wavelength: λπ = h/(mπc) ≈ 8.9×10⁻¹⁶ m
pub const PION_COMPTON_WAVELENGTH: Distance<Meter> = Distance::new(8.9e-16);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::fundamental::*;

    #[test]
    fn test_proton_neutron_mass_difference() {
        let difference = NEUTRON_MASS.value() - PROTON_MASS.value();
        assert!((difference - PROTON_NEUTRON_MASS_DIFFERENCE.value()).abs() / difference < 1e-10);
    }

    #[test]
    fn test_neutron_lifetime() {
        // Should be around 15 minutes
        assert!(NEUTRON_LIFETIME.value() > 800.0);
        assert!(NEUTRON_LIFETIME.value() < 900.0);
    }

    #[test]
    fn test_muon_lifetime() {
        // Should be microseconds
        assert!(MUON_LIFETIME.value() > 1e-6);
        assert!(MUON_LIFETIME.value() < 3e-6);
    }

    #[test]
    fn test_particle_mass_hierarchy() {
        // Check mass ordering: electron < muon < proton < neutron
        assert!(ELECTRON_MASS_MEV < MUON_MASS_MEV);
        assert!(MUON_MASS_MEV < PROTON_MASS_MEV);
        assert!(PROTON_MASS_MEV < NEUTRON_MASS_MEV);
        
        // Tau should be heaviest lepton
        assert!(TAU_MASS_MEV > MUON_MASS_MEV);
    }

    #[test]
    fn test_quark_mass_hierarchy() {
        // Check quark mass ordering
        assert!(UP_QUARK_MASS_MEV < DOWN_QUARK_MASS_MEV);
        assert!(DOWN_QUARK_MASS_MEV < STRANGE_QUARK_MASS_MEV);
        assert!(STRANGE_QUARK_MASS_MEV < CHARM_QUARK_MASS_GEV * 1000.0);
        assert!(CHARM_QUARK_MASS_GEV < BOTTOM_QUARK_MASS_GEV);
        assert!(BOTTOM_QUARK_MASS_GEV < TOP_QUARK_MASS_GEV);
    }

    #[test]
    fn test_boson_masses() {
        // W and Z should be similar, Higgs a bit higher
        assert!((W_BOSON_MASS_GEV - Z_BOSON_MASS_GEV).abs() < 20.0);
        assert!(HIGGS_BOSON_MASS_GEV > Z_BOSON_MASS_GEV);
        assert!(HIGGS_BOSON_MASS_GEV < 150.0);
    }

    #[test]
    fn test_pion_masses() {
        // Charged pions should be slightly heavier than neutral
        assert!(PION_CHARGED_MASS_MEV > PION_NEUTRAL_MASS_MEV);
        assert!((PION_CHARGED_MASS_MEV - PION_NEUTRAL_MASS_MEV) < 10.0);
    }

    #[test]
    fn test_kaon_masses() {
        // Kaons should be heavier than pions
        assert!(KAON_CHARGED_MASS_MEV > PION_CHARGED_MASS_MEV);
        assert!(KAON_NEUTRAL_MASS_MEV > PION_NEUTRAL_MASS_MEV);
    }

    #[test]
    fn test_particle_lifetimes() {
        // Check lifetime ordering: neutral pion << charged pion < kaon short < muon < neutron
        assert!(PION_NEUTRAL_LIFETIME.value() < PION_CHARGED_LIFETIME.value());
        assert!(KAON_SHORT_LIFETIME.value() < MUON_LIFETIME.value());
        assert!(MUON_LIFETIME.value() < NEUTRON_LIFETIME.value());
    }

    #[test]
    fn test_deuteron_binding_energy() {
        // Should be a few MeV
        assert!(DEUTERON_BINDING_ENERGY_MEV > 2.0);
        assert!(DEUTERON_BINDING_ENERGY_MEV < 3.0);
    }

    #[test]
    fn test_radioactive_half_lives() {
        // Check ordering: tritium < C-14 < Pu-239 < U-235 < U-238
        assert!(TRITIUM_HALF_LIFE_YEARS < CARBON_14_HALF_LIFE_YEARS);
        assert!(CARBON_14_HALF_LIFE_YEARS < PLUTONIUM_239_HALF_LIFE_YEARS);
        assert!(PLUTONIUM_239_HALF_LIFE_YEARS < URANIUM_235_HALF_LIFE_YEARS);
        assert!(URANIUM_235_HALF_LIFE_YEARS < URANIUM_238_HALF_LIFE_YEARS);
    }

    #[test]
    fn test_critical_masses() {
        // Plutonium should have smaller critical mass than uranium
        assert!(PLUTONIUM_239_CRITICAL_MASS.value() < URANIUM_235_CRITICAL_MASS.value());
        
        // Both should be reasonable (kg scale)
        assert!(URANIUM_235_CRITICAL_MASS.value() > 10.0);
        assert!(URANIUM_235_CRITICAL_MASS.value() < 100.0);
    }

    #[test]
    fn test_cross_sections() {
        // Fission cross section should be less than absorption
        assert!(URANIUM_235_FISSION_CROSS_SECTION_BARNS < URANIUM_235_ABSORPTION_CROSS_SECTION_BARNS);
        
        // Both should be reasonable (hundreds of barns)
        assert!(URANIUM_235_ABSORPTION_CROSS_SECTION_BARNS > 500.0);
        assert!(URANIUM_235_FISSION_CROSS_SECTION_BARNS > 400.0);
    }

    #[test]
    fn test_barn_unit() {
        assert_eq!(BARN.value(), 1e-28);
    }

    #[test]
    fn test_nuclear_scales() {
        // Nuclear radius should be femtometer scale
        assert!(NUCLEAR_RADIUS_CONSTANT.value() > 1e-16);
        assert!(NUCLEAR_RADIUS_CONSTANT.value() < 1e-14);
        
        // Nuclear force range should be similar
        assert!(NUCLEAR_FORCE_RANGE.value() > 1e-16);
        assert!(NUCLEAR_FORCE_RANGE.value() < 1e-14);
    }

    #[test]
    fn test_coupling_constants() {
        // Strong coupling should be order 0.1
        assert!(STRONG_COUPLING_CONSTANT > 0.05);
        assert!(STRONG_COUPLING_CONSTANT < 0.2);
        
        // Weak coupling should be smaller
        assert!(WEAK_COUPLING_CONSTANT < STRONG_COUPLING_CONSTANT);
        assert!(WEAK_COUPLING_CONSTANT > 0.01);
    }
}