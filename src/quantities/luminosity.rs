#![allow(non_snake_case)]
//! Luminosity units for stellar brightness and energy output calculations

use crate::*;

// Re-export the type alias from core
pub use crate::core::Luminosity;

// Define luminosity units as aliases (same dimensional structure as Power: L²M¹T⁻³)
define_composed_unit!(Watt, "W", 1.0); // SI base unit

// Standard power/luminosity units
define_composed_unit!(Kilowatt, "kW", 1_000.0);
define_composed_unit!(Megawatt, "MW", 1_000_000.0);
define_composed_unit!(Gigawatt, "GW", 1_000_000_000.0);

// Small power units
define_composed_unit!(Milliwatt, "mW", 0.001);
define_composed_unit!(Microwatt, "μW", 0.000_001);
define_composed_unit!(Nanowatt, "nW", 0.000_000_001);

// Astronomical luminosity units
define_composed_unit!(SolarLuminosity, "L☉", 3.828e26); // Solar luminosity in watts
define_composed_unit!(ErgPerSecond, "erg/s", 1e-7); // CGS unit: 1 erg/s = 1e-7 W

// Alternative astronomical notations
define_composed_unit!(SolarLuminosityUnit, "L_sun", 3.828e26); // Alternative notation

// Generate prefixed aliases
define_prefixed_aliases! {
    Watt => [Kilo, Mega, Giga, Tera, Milli, Micro, Nano],
    ErgPerSecond => [Mega, Giga, Tera],
}

// Convenience constructors
impl_quantity_constructors!(
    Luminosity,
    Watt,
    Kilowatt,
    Megawatt,
    Gigawatt,
    Milliwatt,
    Microwatt,
    Nanowatt,
    SolarLuminosity,
    SolarLuminosityUnit,
    ErgPerSecond
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luminosity_units() {
        let lum1 = Luminosity::<Watt>::new(1000.0);
        let lum2 = Luminosity::<(Meter, Meter, Kilogram, Second, Second, Second)>::new(1000.0);

        assert_eq!(lum1.value(), 1000.0);
        assert_eq!(lum2.value(), 1000.0);
    }

    #[test]
    fn test_luminosity_conversions() {
        let lum_w = Luminosity::<Watt>::new(1000.0);
        let lum_kw: Luminosity<Kilowatt> = lum_w.convert_to();

        // 1000 W = 1 kW
        assert_eq!(lum_kw.value(), 1.0);
    }

    #[test]
    fn test_solar_luminosity() {
        let sun = Luminosity::<SolarLuminosity>::new(1.0);
        let sun_watts: Luminosity<Watt> = sun.convert_to();

        // 1 L☉ = 3.828e26 W
        assert_eq!(sun_watts.value(), 3.828e26);
    }

    #[test]
    fn test_stellar_comparisons() {
        // Red dwarf: ~0.0001 L☉
        let red_dwarf = Luminosity::<SolarLuminosity>::new(0.0001);
        let red_dwarf_w: Luminosity<Watt> = red_dwarf.convert_to();

        // Blue giant: ~10,000 L☉
        let blue_giant = Luminosity::<SolarLuminosity>::new(10_000.0);
        let blue_giant_w: Luminosity<Watt> = blue_giant.convert_to();

        assert!((red_dwarf_w.value() - 3.828e22).abs() < 1e19); // 0.0001 * 3.828e26
        assert!((blue_giant_w.value() - 3.828e30).abs() < 1e27); // 10000 * 3.828e26
    }

    #[test]
    fn test_cgs_units() {
        let lum_erg = Luminosity::<ErgPerSecond>::new(1e34); // Typical stellar luminosity
        let lum_w: Luminosity<Watt> = lum_erg.convert_to();

        // 1e34 erg/s = 1e27 W
        assert!((lum_w.value() - 1e27).abs() < 1e24);

        // Convert to solar luminosities
        let lum_solar: Luminosity<SolarLuminosity> = lum_w.convert_to();
        assert!((lum_solar.value() - 2.611).abs() < 0.01); // ~2.6 L☉
    }

    #[test]
    fn test_variable_stars() {
        // Cepheid variable: varies from 1000 to 10000 L☉
        let cepheid_min = Luminosity::<SolarLuminosity>::new(1000.0);
        let cepheid_max = Luminosity::<SolarLuminosity>::new(10_000.0);

        let brightness_ratio = cepheid_max / cepheid_min;
        assert_eq!(brightness_ratio, 10.0); // 10x brightness variation
    }

    #[test]
    fn test_white_dwarf_cooling() {
        // Young white dwarf: ~0.01 L☉
        let young_wd = Luminosity::<SolarLuminosity>::new(0.01);
        let young_w: Luminosity<Watt> = young_wd.convert_to();

        // Old white dwarf: ~0.0001 L☉
        let old_wd = Luminosity::<SolarLuminosity>::new(0.0001);
        let old_w: Luminosity<Watt> = old_wd.convert_to();

        assert!((young_w.value() - 3.828e24).abs() < 1e21);
        assert!((old_w.value() - 3.828e22).abs() < 1e19);

        let cooling_ratio = young_wd / old_wd;
        assert!((cooling_ratio - 100.0).abs() < 1e-10); // 100x dimmer after cooling
    }

    #[test]
    fn test_supergiant_luminosity() {
        // Red supergiant: ~100,000 L☉
        let supergiant = Luminosity::<SolarLuminosity>::new(100_000.0);
        let supergiant_w: Luminosity<Watt> = supergiant.convert_to();

        // Should be enormous: 100,000 * 3.828e26 W
        assert!(supergiant_w.value() > 3e31);
        assert!(supergiant_w.value() < 4e31);
    }

    #[test]
    fn test_mixed_unit_arithmetic() {
        let lum1 = Luminosity::<Watt>::new(1000.0); // 1 kW
        let lum2 = Luminosity::<Kilowatt>::new(2.0); // 2 kW
        let total = lum1 + lum2; // Result in SI units (W)

        assert_eq!(total.value(), 3000.0); // 3000 W = 3 kW
    }

    #[test]
    fn test_dimensionless_ratios() {
        let sun_lum = Luminosity::<SolarLuminosity>::new(1.0);
        let sirius_lum = Luminosity::<SolarLuminosity>::new(25.0); // Sirius is ~25 L☉
        let ratio = sirius_lum / sun_lum;

        assert_eq!(ratio, 25.0); // Sirius is 25x brighter than Sun
    }
}
