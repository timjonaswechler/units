//! Power units for energy transfer rates and luminosity

use crate::*;

// Re-export the type alias from core
pub use crate::core::Power;

// Re-export units from luminosity module since Power and Luminosity have the same dimension
pub use crate::quantities::luminosity::{Watt, ErgPerSecond, SolarLuminosity, SolarLuminosityUnit};

// Define power-specific units that don't conflict with luminosity module
define_composed_unit!(Horsepower, "hp", 745.699_871_582_27);    // Mechanical horsepower
define_composed_unit!(MetricHorsepower, "PS", 735.498_75);      // European metric horsepower
define_composed_unit!(BTUPerHour, "BTU/h", 0.293_071_07);      // British Thermal Unit per hour
define_composed_unit!(CaloriePerSecond, "cal/s", 4.184);       // 4.184 W
define_composed_unit!(FootPoundPerSecond, "ft⋅lbf/s", 1.355_818); // Imperial power unit

// Electrical power (same dimension, different context)
define_composed_unit!(VoltAmpere, "VA", 1.0);                  // Apparent power unit
define_composed_unit!(VoltAmpereReactive, "VAR", 1.0);         // Reactive power unit

// Generate prefixed aliases for power-specific units
define_prefixed_aliases! {
    Horsepower => [Kilo],
    BTUPerHour => [Kilo, Mega],
}

// Convenience constructors for power-specific units
impl_quantity_constructors!(
    Power,
    Horsepower, MetricHorsepower, BTUPerHour,
    CaloriePerSecond, FootPoundPerSecond,
    VoltAmpere, VoltAmpereReactive
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_units() {
        let power1 = Power::<Watt>::new(1000.0);
        let power2 = Power::<(Meter, Kilogram, Second)>::new(1000.0); // L²M¹T⁻³ dimension
        
        assert_eq!(power1.value(), 1000.0);
        assert_eq!(power2.value(), 1000.0);
    }

    #[test]
    fn test_watt_usage() {
        // Test that we can use Watt from the luminosity module
        let power_w = Power::<Watt>::new(1000.0);
        
        // Verify basic functionality
        assert_eq!(power_w.value(), 1000.0);
    }

    #[test]
    fn test_horsepower_conversion() {
        let power_hp = Power::<Horsepower>::new(1.0);
        let power_w: Power<Watt> = power_hp.convert_to();
        
        // 1 hp ≈ 745.7 W
        assert!((power_w.value() - 745.699_871_582_27).abs() < 0.001);
    }

    #[test]
    fn test_metric_horsepower() {
        let power_ps = Power::<MetricHorsepower>::new(1.0);
        let power_w: Power<Watt> = power_ps.convert_to();
        
        // 1 PS ≈ 735.5 W
        assert!((power_w.value() - 735.498_75).abs() < 0.001);
    }

    #[test]
    fn test_cgs_power() {
        let power_erg = Power::<ErgPerSecond>::new(1e7);
        let power_w: Power<Watt> = power_erg.convert_to();
        
        // 10^7 erg/s = 1 W
        assert!((power_w.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_solar_luminosity() {
        let power_solar = Power::<SolarLuminosity>::new(1.0);
        let power_w: Power<Watt> = power_solar.convert_to();
        
        // 1 L☉ = 3.828×10²⁶ W
        assert!((power_w.value() - 3.828e26).abs() < 1e23);
    }

    #[test]
    fn test_stellar_power_scales() {
        // Red dwarf star: ~0.0001 L☉
        let red_dwarf = Power::<SolarLuminosity>::new(0.0001);
        
        // Blue supergiant: ~100,000 L☉
        let blue_giant = Power::<SolarLuminosity>::new(100_000.0);
        
        let luminosity_ratio = blue_giant / red_dwarf;
        
        // Should be 10^9 ratio
        assert!((luminosity_ratio - 1e9).abs() < 1e6);
    }

    #[test]
    fn test_electrical_power_units() {
        let power_va = Power::<VoltAmpere>::new(1000.0);
        let power_w: Power<Watt> = power_va.convert_to();
        
        // 1 VA = 1 W (same dimension, different electrical context)
        assert!((power_w.value() - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_btu_per_hour() {
        let power_btu = Power::<BTUPerHour>::new(1000.0);
        let power_w: Power<Watt> = power_btu.convert_to();
        
        // 1000 BTU/h ≈ 293 W
        assert!((power_w.value() - 293.071_07).abs() < 0.1);
    }

    #[test]
    fn test_mechanical_power() {
        let power_ftlb = Power::<FootPoundPerSecond>::new(100.0);
        let power_w: Power<Watt> = power_ftlb.convert_to();
        
        // 100 ft⋅lbf/s ≈ 135.6 W
        assert!((power_w.value() - 135.5818).abs() < 0.1);
    }

    #[test]
    fn test_calorie_power() {
        let power_cal = Power::<CaloriePerSecond>::new(10.0);
        let power_w: Power<Watt> = power_cal.convert_to();
        
        // 10 cal/s = 41.84 W
        assert!((power_w.value() - 41.84).abs() < 0.01);
    }

    #[test]
    fn test_mixed_unit_arithmetic() {
        let power1 = Power::<Watt>::new(500.0);
        let power2 = Power::<Horsepower>::new(1.0);  // ~745.7 W
        let total = power1 + power2;  // Result in SI units (Watts)
        
        let expected = 500.0 + 745.699_871_582_27;
        assert!((total.value() - expected).abs() < 0.1);
    }

    #[test]
    fn test_dimensionless_ratios() {
        let solar_power = Power::<SolarLuminosity>::new(1.0);
        let household = Power::<Watt>::new(10_000.0);  // 10 kW house
        
        let ratio = solar_power / household;
        
        // Sun produces vastly more power than household consumption
        assert!(ratio > 1e22);
    }
}