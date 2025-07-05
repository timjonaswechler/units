//! Pressure units for stellar atmospheres, interstellar medium, and fluid dynamics

use crate::*;

// Re-export the type alias from core
pub use crate::core::Pressure;

// Define pressure units as aliases (dimension: L⁻¹M¹T⁻²)
define_composed_unit!(Pascal, "Pa", 1.0);  // SI base unit: N/m² = kg/(m·s²)

// Common pressure units
define_composed_unit!(Kilopascal, "kPa", 1_000.0);
define_composed_unit!(Megapascal, "MPa", 1_000_000.0);
define_composed_unit!(Gigapascal, "GPa", 1_000_000_000.0);

// Small pressure units
define_composed_unit!(Millipascal, "mPa", 0.001);
define_composed_unit!(Micropascal, "μPa", 0.000_001);
define_composed_unit!(Nanopascal, "nPa", 0.000_000_001);

// Atmospheric and meteorological units
define_composed_unit!(Bar, "bar", 100_000.0);                    // 1 bar = 100,000 Pa
define_composed_unit!(Millibar, "mbar", 100.0);                  // 1 mbar = 100 Pa
define_composed_unit!(Atmosphere, "atm", 101_325.0);             // Standard atmosphere
define_composed_unit!(Torr, "Torr", 133.322_387_415);           // 1 Torr = 1 mmHg
define_composed_unit!(MillimeterMercury, "mmHg", 133.322_387_415); // Same as Torr

// CGS units for astrophysics
define_composed_unit!(Dyne, "dyn/cm²", 0.1);                    // 1 dyn/cm² = 0.1 Pa
define_composed_unit!(Barye, "Ba", 0.1);                        // CGS pressure unit (dyn/cm²)

// Astronomical pressure units
define_composed_unit!(SolarWindPressure, "P_sw", 1e-9);         // Typical solar wind pressure ~nPa
define_composed_unit!(InterstellarPressure, "P_ISM", 1e-13);    // Interstellar medium pressure ~0.1 fPa

// High-pressure units for stellar interiors
define_composed_unit!(Terapascal, "TPa", 1e12);                 // For stellar core pressures
define_composed_unit!(Petapascal, "PPa", 1e15);                 // Extreme stellar pressures

// Generate prefixed aliases
define_prefixed_aliases! {
    Pascal => [Kilo, Mega, Giga, Tera, Peta, Milli, Micro, Nano],
    Bar => [Milli, Micro],
    Dyne => [Mega, Giga],
}

// Convenience constructors
impl_quantity_constructors!(
    Pressure,
    Pascal, Kilopascal, Megapascal, Gigapascal, Terapascal, Petapascal,
    Millipascal, Micropascal, Nanopascal,
    Bar, Millibar, Atmosphere, Torr, MillimeterMercury,
    Dyne, Barye, SolarWindPressure, InterstellarPressure
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_units() {
        let p1 = Pressure::<Pascal>::new(100_000.0);
        let p2 = Pressure::<(Meter, Kilogram, Second, Second)>::new(100_000.0); // L⁻¹M¹T⁻² dimension
        
        assert_eq!(p1.value(), 100_000.0);
        assert_eq!(p2.value(), 100_000.0);
    }

    #[test]
    fn test_pressure_conversions() {
        let p_pa = Pressure::<Pascal>::new(100_000.0);
        let p_bar: Pressure<Bar> = p_pa.convert_to();
        
        // 100,000 Pa = 1 bar
        assert_eq!(p_bar.value(), 1.0);
    }

    #[test]
    fn test_atmospheric_pressure() {
        let atm = Pressure::<Atmosphere>::new(1.0);
        let pa: Pressure<Pascal> = atm.convert_to();
        let bar: Pressure<Bar> = atm.convert_to();
        
        // 1 atm = 101,325 Pa = 1.01325 bar
        assert_eq!(pa.value(), 101_325.0);
        assert!((bar.value() - 1.01325).abs() < 1e-5);
    }

    #[test]
    fn test_stellar_atmosphere_pressures() {
        // Solar photosphere: ~100 Pa
        let solar_photosphere = Pressure::<Pascal>::new(100.0);
        let solar_mbar: Pressure<Millibar> = solar_photosphere.convert_to();
        
        // 100 Pa = 1 mbar
        assert_eq!(solar_mbar.value(), 1.0);
        
        // Red giant atmosphere: ~1 Pa
        let red_giant_atm = Pressure::<Pascal>::new(1.0);
        let red_giant_mpa: Pressure<Micropascal> = red_giant_atm.convert_to();
        
        // 1 Pa = 1,000,000 μPa
        assert_eq!(red_giant_mpa.value(), 1_000_000.0);
    }

    #[test]
    fn test_stellar_interior_pressures() {
        // Solar core: ~25 billion Pa
        let solar_core = Pressure::<Gigapascal>::new(25.0);
        let solar_core_pa: Pressure<Pascal> = solar_core.convert_to();
        
        // 25 GPa = 25e9 Pa
        assert_eq!(solar_core_pa.value(), 25e9);
        
        // White dwarf interior: ~10^18 Pa
        let white_dwarf = Pressure::<Petapascal>::new(1000.0);  // 1000 PPa = 10^18 Pa
        let wd_pa: Pressure<Pascal> = white_dwarf.convert_to();
        
        assert_eq!(wd_pa.value(), 1e18);
    }

    #[test]
    fn test_interstellar_medium() {
        // ISM pressure: ~10^-13 Pa
        let ism = Pressure::<InterstellarPressure>::new(1.0);
        let ism_pa: Pressure<Pascal> = ism.convert_to();
        
        assert_eq!(ism_pa.value(), 1e-13);
        
        // Solar wind pressure: ~1 nPa
        let solar_wind = Pressure::<SolarWindPressure>::new(1.0);
        let sw_pa: Pressure<Pascal> = solar_wind.convert_to();
        
        assert_eq!(sw_pa.value(), 1e-9);
    }

    #[test]
    fn test_cgs_pressure_units() {
        // CGS: 1 dyn/cm² = 0.1 Pa
        let cgs_pressure = Pressure::<Dyne>::new(1.0);
        let si_pressure: Pressure<Pascal> = cgs_pressure.convert_to();
        
        assert_eq!(si_pressure.value(), 0.1);
        
        // Barye is same as dyn/cm²
        let barye = Pressure::<Barye>::new(1.0);
        let barye_pa: Pressure<Pascal> = barye.convert_to();
        
        assert_eq!(barye_pa.value(), 0.1);
    }

    #[test]
    fn test_mercury_pressure() {
        let torr = Pressure::<Torr>::new(760.0);  // Standard atmosphere in Torr
        let atm: Pressure<Atmosphere> = torr.convert_to();
        
        // 760 Torr = 1 atm (approximately)
        assert!((atm.value() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_mixed_unit_arithmetic() {
        let p1 = Pressure::<Pascal>::new(50_000.0);      // 50 kPa
        let p2 = Pressure::<Kilopascal>::new(50.0);      // 50 kPa
        let total = p1 + p2;  // Result in SI units (Pa)
        
        assert_eq!(total.value(), 100_000.0);  // 100,000 Pa = 100 kPa
    }

    #[test]
    fn test_dimensionless_ratios() {
        let earth_atm = Pressure::<Atmosphere>::new(1.0);
        let venus_surface = Pressure::<Atmosphere>::new(92.0);  // Venus surface ~92 atm
        let ratio = venus_surface / earth_atm;
        
        assert_eq!(ratio, 92.0);  // Venus is 92x Earth atmospheric pressure
    }
}