#![allow(non_snake_case)]
//! Momentum units for particle physics, celestial mechanics, and collisions

use crate::*;

// Re-export the type alias from core
pub use crate::core::Momentum;

// Define momentum units as aliases (dimension: L¹M¹T⁻¹)
define_composed_unit!(KilogramMeterPerSecond, "kg⋅m/s", 1.0); // SI base unit

// Alternative notation for SI unit
define_composed_unit!(NewtonSecond, "N⋅s", 1.0); // N⋅s = kg⋅m/s (impulse unit)

// Common momentum units with prefixes
define_composed_unit!(GramCentimeterPerSecond, "g⋅cm/s", 0.001 * 0.01); // CGS unit: 0.00001 kg⋅m/s
define_composed_unit!(GramMeterPerSecond, "g⋅m/s", 0.001); // 0.001 kg⋅m/s

// Large momentum units
define_composed_unit!(TonneMeterPerSecond, "t⋅m/s", 1000.0); // 1000 kg⋅m/s
define_composed_unit!(KilogramKilometerPerSecond, "kg⋅km/s", 1000.0); // 1000 kg⋅m/s

// Particle physics units (using natural units and electron volt system)
define_composed_unit!(ElectronVoltPerSpeedOfLight, "eV/c", 5.344286e-28); // eV/c in kg⋅m/s
define_composed_unit!(MegaElectronVoltPerSpeedOfLight, "MeV/c", 5.344286e-22); // MeV/c
define_composed_unit!(GigaElectronVoltPerSpeedOfLight, "GeV/c", 5.344286e-19); // GeV/c
define_composed_unit!(TeraElectronVoltPerSpeedOfLight, "TeV/c", 5.344286e-16); // TeV/c

// Astronomical momentum units
define_composed_unit!(
    SolarMassAUPerYear,
    "M☉⋅AU/yr",
    1.989e30 * 1.496e11 / 31_557_600.0
); // ~9.42e33 kg⋅m/s
define_composed_unit!(EarthMassKilometerPerSecond, "M_⊕⋅km/s", 5.972e24 * 1000.0); // Earth mass × km/s
define_composed_unit!(LunarMassKilometerPerSecond, "M_☽⋅km/s", 7.342e22 * 1000.0); // Lunar mass × km/s

// Atomic and molecular scales
define_composed_unit!(AtomicMassUnitMeterPerSecond, "u⋅m/s", 1.66054e-27); // Atomic mass unit momentum
define_composed_unit!(
    ElectronMassSpeedOfLight,
    "m_e⋅c",
    9.1094e-31 * 299_792_458.0
); // Electron rest momentum

// Spacecraft and engineering scales
define_composed_unit!(KilogramKilometerPerHour, "kg⋅km/h", 1000.0 / 3600.0); // ~0.278 kg⋅m/s

// Generate prefixed aliases
define_prefixed_aliases! {
    KilogramMeterPerSecond => [Kilo, Mega, Giga, Milli, Micro, Nano],
    NewtonSecond => [Kilo, Mega, Milli, Micro],
}

// Convenience constructors
impl_quantity_constructors!(
    Momentum,
    KilogramMeterPerSecond,
    NewtonSecond,
    GramCentimeterPerSecond,
    GramMeterPerSecond,
    TonneMeterPerSecond,
    KilogramKilometerPerSecond,
    ElectronVoltPerSpeedOfLight,
    MegaElectronVoltPerSpeedOfLight,
    GigaElectronVoltPerSpeedOfLight,
    TeraElectronVoltPerSpeedOfLight,
    SolarMassAUPerYear,
    EarthMassKilometerPerSecond,
    LunarMassKilometerPerSecond,
    AtomicMassUnitMeterPerSecond,
    ElectronMassSpeedOfLight,
    KilogramKilometerPerHour
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_momentum_units() {
        let p1 = Momentum::<KilogramMeterPerSecond>::new(100.0);
        let p2 = Momentum::<(Meter, Kilogram, Second)>::new(100.0); // L¹M¹T⁻¹ dimension

        assert_eq!(p1.value(), 100.0);
        assert_eq!(p2.value(), 100.0);
    }

    #[test]
    fn test_impulse_equivalence() {
        let momentum = Momentum::<KilogramMeterPerSecond>::new(50.0);
        let impulse: Momentum<NewtonSecond> = momentum.convert_to();

        // kg⋅m/s = N⋅s (same unit, different notation)
        assert_eq!(impulse.value(), 50.0);
    }

    #[test]
    fn test_cgs_momentum() {
        let p_cgs = Momentum::<GramCentimeterPerSecond>::new(1000.0);
        let p_si: Momentum<KilogramMeterPerSecond> = p_cgs.convert_to();

        // 1000 g⋅cm/s = 0.01 kg⋅m/s
        assert!((p_si.value() - 0.01).abs() < 1e-10);
    }

    #[test]
    fn test_particle_physics_momentum() {
        let electron_rest = Momentum::<ElectronMassSpeedOfLight>::new(1.0);
        let electron_si: Momentum<KilogramMeterPerSecond> = electron_rest.convert_to();

        // Electron rest momentum: m_e × c
        let expected = 9.1094e-31 * 299_792_458.0;
        assert!((electron_si.value() - expected).abs() < 1e-35);
    }

    #[test]
    fn test_relativistic_particle_momentum() {
        // High-energy electron: 1 GeV/c
        let electron_gev = Momentum::<GigaElectronVoltPerSpeedOfLight>::new(1.0);
        let electron_si: Momentum<KilogramMeterPerSecond> = electron_gev.convert_to();

        // 1 GeV/c should be much larger than electron rest momentum
        assert!(electron_si.value() > 1e-22);
        assert!(electron_si.value() < 1e-18);
    }

    #[test]
    fn test_astronomical_momentum() {
        // Earth's orbital momentum (approximate)
        let earth_orbit = Momentum::<EarthMassKilometerPerSecond>::new(30.0); // ~30 km/s orbital speed
        let earth_si: Momentum<KilogramMeterPerSecond> = earth_orbit.convert_to();

        // Should be enormous: ~1.8×10^29 kg⋅m/s
        assert!(earth_si.value() > 1e28);
        assert!(earth_si.value() < 2e29);
    }

    #[test]
    fn test_solar_system_momentum() {
        let solar_momentum = Momentum::<SolarMassAUPerYear>::new(1.0);
        let solar_si: Momentum<KilogramMeterPerSecond> = solar_momentum.convert_to();

        // Very large momentum scale
        assert!(solar_si.value() > 1e33);
        assert!(solar_si.value() < 1e35);
    }

    #[test]
    fn test_atomic_scale_momentum() {
        let atomic_momentum = Momentum::<AtomicMassUnitMeterPerSecond>::new(1000.0); // 1 km/s
        let atomic_si: Momentum<KilogramMeterPerSecond> = atomic_momentum.convert_to();

        // Atomic scale: very small
        assert!(atomic_si.value() > 1e-24);
        assert!(atomic_si.value() < 1e-23);
    }

    #[test]
    fn test_spacecraft_momentum() {
        // ISS momentum (approximate: 400 tonnes at 7.7 km/s)
        let iss_mass_kg = 400_000.0;
        let iss_velocity_ms = 7700.0;
        let iss_momentum = Momentum::<KilogramMeterPerSecond>::new(iss_mass_kg * iss_velocity_ms);

        let iss_tonne_kms: Momentum<TonneMeterPerSecond> = iss_momentum.convert_to();

        // Should convert correctly
        let expected_tonne_ms = (iss_mass_kg * iss_velocity_ms) / 1000.0;
        assert!((iss_tonne_kms.value() - expected_tonne_ms).abs() < 1.0);
    }

    #[test]
    fn test_momentum_conservation() {
        // Conservation in collision: p1 + p2 = p_total
        let p1 = Momentum::<KilogramMeterPerSecond>::new(100.0);
        let p2 = Momentum::<GramMeterPerSecond>::new(50_000.0); // 50 kg⋅m/s
        let total = p1 + p2; // Result in SI units

        // 100 + 50 = 150 kg⋅m/s
        assert!((total.value() - 150.0).abs() < 1e-10);
    }

    #[test]
    fn test_impulse_momentum_theorem() {
        // Impulse = change in momentum
        let initial_momentum = Momentum::<KilogramMeterPerSecond>::new(20.0);
        let final_momentum = Momentum::<KilogramMeterPerSecond>::new(80.0);
        let impulse = final_momentum - initial_momentum;

        let impulse_ns: Momentum<NewtonSecond> = impulse.convert_to();

        // Change in momentum = 60 kg⋅m/s = 60 N⋅s
        assert!((impulse_ns.value() - 60.0).abs() < 1e-10);
    }

    #[test]
    fn test_dimensionless_ratios() {
        let photon_momentum = Momentum::<GigaElectronVoltPerSpeedOfLight>::new(1.0);
        let electron_rest = Momentum::<ElectronMassSpeedOfLight>::new(1.0);

        // Ratio should be dimensionless
        let ratio = photon_momentum / electron_rest;

        // 1 GeV/c vs electron rest momentum (~0.511 MeV/c)
        assert!(ratio > 1000.0); // Should be much larger
    }

    #[test]
    fn test_mixed_unit_arithmetic() {
        let p1 = Momentum::<KilogramMeterPerSecond>::new(50.0);
        let p2 = Momentum::<NewtonSecond>::new(30.0);
        let total = p1 + p2; // Result in SI units (kg⋅m/s)

        assert!((total.value() - 80.0).abs() < 1e-10);
    }

    #[test]
    fn test_large_scale_momentum_comparison() {
        let asteroid = Momentum::<TonneMeterPerSecond>::new(1e6); // 1000 tonnes at 1 km/s
        let planet = Momentum::<EarthMassKilometerPerSecond>::new(30.0); // Earth at orbital speed

        let ratio = planet / asteroid;

        // Earth's orbital momentum should be vastly larger
        assert!(ratio > 1e20);
    }

    #[test]
    fn test_quantum_classical_momentum_scales() {
        let quantum = Momentum::<ElectronVoltPerSpeedOfLight>::new(1e6); // 1 MeV/c
        let classical = Momentum::<GramMeterPerSecond>::new(1.0); // 1 g⋅m/s

        let scale_ratio = classical / quantum;

        // Classical should be much larger than single particle quantum scale
        assert!(scale_ratio > 1e15);
    }
}
