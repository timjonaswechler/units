#![allow(non_snake_case)]
//! Density units for stellar matter, interstellar medium, and material properties

use crate::*;

// Re-export the type alias from core
pub use crate::core::Density;

// Define density units as aliases (dimension: L⁻³M¹)
define_composed_unit!(KilogramPerCubicMeter, "kg/m³", 1.0); // SI base unit

// Common SI density units
define_composed_unit!(GramPerCubicCentimeter, "g/cm³", 1_000.0); // 1 g/cm³ = 1000 kg/m³
define_composed_unit!(GramPerCubicMeter, "g/m³", 0.001); // 1 g/m³ = 0.001 kg/m³
define_composed_unit!(TonnePerCubicMeter, "t/m³", 1_000.0); // 1 t/m³ = 1000 kg/m³

// CGS units for astrophysics
define_composed_unit!(GramPerCubicCentimeterCGS, "g·cm⁻³", 1_000.0); // Standard CGS notation

// Astronomical density units
define_composed_unit!(SolarDensity, "ρ☉", 1_408.0); // Solar mean density: ~1.408 g/cm³
define_composed_unit!(WaterDensity, "ρ_H₂O", 1_000.0); // Water density: 1000 kg/m³

// Extreme astronomical densities
define_composed_unit!(WhiteDwarfDensity, "ρ_WD", 1e9); // ~10⁶ g/cm³ = 10⁹ kg/m³
define_composed_unit!(NeutronStarDensity, "ρ_NS", 5e17); // ~5×10¹⁴ g/cm³ = 5×10¹⁷ kg/m³
define_composed_unit!(NuclearDensity, "ρ_nuc", 2.3e17); // Nuclear density: ~2.3×10¹⁴ g/cm³

// Interstellar medium densities
define_composed_unit!(InterstellarDensity, "ρ_ISM", 1e-18); // ~10⁻²¹ g/cm³ = 10⁻¹⁸ kg/m³
define_composed_unit!(MolecularCloudDensity, "ρ_MC", 1e-15); // ~10⁻¹⁸ g/cm³ = 10⁻¹⁵ kg/m³

// Planetary and atmospheric densities
define_composed_unit!(EarthDensity, "ρ_⊕", 5_515.0); // Earth mean density: ~5.515 g/cm³
define_composed_unit!(AirDensity, "ρ_air", 1.225); // Sea level air: ~1.225 kg/m³

// Generate prefixed aliases for SI units
define_prefixed_aliases! {
    KilogramPerCubicMeter => [Mega, Giga],
    GramPerCubicMeter => [Kilo, Mega],
}

// Convenience constructors
impl_quantity_constructors!(
    Density,
    KilogramPerCubicMeter,
    GramPerCubicCentimeter,
    GramPerCubicMeter,
    TonnePerCubicMeter,
    GramPerCubicCentimeterCGS,
    SolarDensity,
    WaterDensity,
    WhiteDwarfDensity,
    NeutronStarDensity,
    NuclearDensity,
    InterstellarDensity,
    MolecularCloudDensity,
    EarthDensity,
    AirDensity
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_units() {
        let d1 = Density::<KilogramPerCubicMeter>::new(1000.0);
        let d2 = Density::<(Meter, Meter, Meter, Kilogram)>::new(1000.0); // L⁻³M¹ dimension

        assert_eq!(d1.value(), 1000.0);
        assert_eq!(d2.value(), 1000.0);
    }

    #[test]
    fn test_density_conversions() {
        let d_kg_m3 = Density::<KilogramPerCubicMeter>::new(1000.0);
        let d_g_cm3: Density<GramPerCubicCentimeter> = d_kg_m3.convert_to();

        // 1000 kg/m³ = 1 g/cm³
        assert_eq!(d_g_cm3.value(), 1.0);
    }

    #[test]
    fn test_water_density() {
        let water = Density::<WaterDensity>::new(1.0);
        let water_si: Density<KilogramPerCubicMeter> = water.convert_to();
        let water_cgs: Density<GramPerCubicCentimeter> = water.convert_to();

        // Water: 1000 kg/m³ = 1 g/cm³
        assert_eq!(water_si.value(), 1000.0);
        assert_eq!(water_cgs.value(), 1.0);
    }

    #[test]
    fn test_stellar_densities() {
        // Solar mean density
        let sun = Density::<SolarDensity>::new(1.0);
        let sun_si: Density<KilogramPerCubicMeter> = sun.convert_to();
        let sun_cgs: Density<GramPerCubicCentimeter> = sun.convert_to();

        assert_eq!(sun_si.value(), 1408.0);
        assert!((sun_cgs.value() - 1.408).abs() < 1e-10);

        // Earth density comparison
        let earth = Density::<EarthDensity>::new(1.0);
        let earth_si: Density<KilogramPerCubicMeter> = earth.convert_to();

        assert_eq!(earth_si.value(), 5515.0);

        // Earth is denser than Sun
        let density_ratio = earth / sun;
        assert!((density_ratio - 3.916).abs() < 0.01); // Earth ~3.9x denser than Sun
    }

    #[test]
    fn test_white_dwarf_density() {
        let wd = Density::<WhiteDwarfDensity>::new(1.0);
        let wd_si: Density<KilogramPerCubicMeter> = wd.convert_to();
        let wd_cgs: Density<GramPerCubicCentimeter> = wd.convert_to();

        // White dwarf: ~10⁶ g/cm³ = 10⁹ kg/m³
        assert_eq!(wd_si.value(), 1e9);
        assert_eq!(wd_cgs.value(), 1e6);

        // Compare to water
        let water = Density::<WaterDensity>::new(1.0);
        let ratio = wd / water;
        assert_eq!(ratio, 1e6); // White dwarf is million times denser than water
    }

    #[test]
    fn test_neutron_star_density() {
        let ns = Density::<NeutronStarDensity>::new(1.0);
        let ns_si: Density<KilogramPerCubicMeter> = ns.convert_to();

        // Neutron star: ~5×10¹⁷ kg/m³
        assert_eq!(ns_si.value(), 5e17);

        // Compare to nuclear density
        let nuclear = Density::<NuclearDensity>::new(1.0);
        let ratio = ns / nuclear;
        assert!((ratio - 2.17).abs() < 0.1); // NS ~2x nuclear density
    }

    #[test]
    fn test_interstellar_medium() {
        // Typical ISM density
        let ism = Density::<InterstellarDensity>::new(1.0);
        let ism_si: Density<KilogramPerCubicMeter> = ism.convert_to();
        let ism_cgs: Density<GramPerCubicCentimeter> = ism.convert_to();

        // ISM: ~10⁻²¹ g/cm³ = 10⁻¹⁸ kg/m³
        assert_eq!(ism_si.value(), 1e-18);
        assert!((ism_cgs.value() - 1e-21).abs() < 1e-24);

        // Molecular cloud (denser)
        let mc = Density::<MolecularCloudDensity>::new(1.0);
        let mc_si: Density<KilogramPerCubicMeter> = mc.convert_to();

        assert_eq!(mc_si.value(), 1e-15);

        // Molecular cloud is 1000x denser than ISM
        let ratio = mc / ism;
        assert_eq!(ratio, 1000.0);
    }

    #[test]
    fn test_atmospheric_density() {
        let air = Density::<AirDensity>::new(1.0);
        let air_si: Density<KilogramPerCubicMeter> = air.convert_to();
        let air_cgs: Density<GramPerCubicCentimeter> = air.convert_to();

        // Air at sea level: ~1.225 kg/m³
        assert_eq!(air_si.value(), 1.225);
        assert!((air_cgs.value() - 0.001225).abs() < 1e-10);

        // Compare to water
        let water = Density::<WaterDensity>::new(1.0);
        let ratio = water / air;
        assert!((ratio - 816.3).abs() < 1.0); // Water ~816x denser than air
    }

    #[test]
    fn test_cgs_units() {
        let cgs_density = Density::<GramPerCubicCentimeterCGS>::new(2.5);
        let si_density: Density<KilogramPerCubicMeter> = cgs_density.convert_to();

        // 2.5 g/cm³ = 2500 kg/m³
        assert_eq!(si_density.value(), 2500.0);
    }

    #[test]
    fn test_mixed_unit_arithmetic() {
        let d1 = Density::<KilogramPerCubicMeter>::new(500.0); // 500 kg/m³
        let d2 = Density::<GramPerCubicCentimeter>::new(0.5); // 0.5 g/cm³ = 500 kg/m³
        let total = d1 + d2; // Result in SI units (kg/m³)

        assert_eq!(total.value(), 1000.0); // 1000 kg/m³ = 1 g/cm³
    }

    #[test]
    fn test_dimensionless_ratios() {
        let lead_density = Density::<KilogramPerCubicMeter>::new(11_340.0); // Lead
        let aluminum_density = Density::<KilogramPerCubicMeter>::new(2_700.0); // Aluminum
        let ratio = lead_density / aluminum_density;

        assert!((ratio - 4.2).abs() < 0.1); // Lead is ~4.2x denser than aluminum
    }

    #[test]
    fn test_extreme_density_scale() {
        let vacuum = Density::<InterstellarDensity>::new(1.0); // ISM density
        let neutron_star = Density::<NeutronStarDensity>::new(1.0); // NS density

        let extreme_ratio = neutron_star / vacuum;
        assert!((extreme_ratio - 5e35).abs() < 1e32); // Neutron star is 5×10³⁵ times denser than ISM
    }
}
