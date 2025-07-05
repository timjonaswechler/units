//! Volume units for three-dimensional measurements in stellar systems

use crate::*;

// Re-export the type alias from core
pub use crate::core::Volume;

// Define volume units as aliases  
define_composed_unit!(CubicMeter, "m³", 1.0);  // SI base unit

// Metric volume units
define_composed_unit!(CubicKilometer, "km³", 1_000_000_000.0);  // 1 km³ = 10⁹ m³
define_composed_unit!(CubicCentimeter, "cm³", 0.000_001);       // 1 cm³ = 10⁻⁶ m³
define_composed_unit!(CubicMillimeter, "mm³", 0.000_000_001);   // 1 mm³ = 10⁻⁹ m³

// Liquid volume units
define_composed_unit!(Liter, "L", 0.001);                       // 1 L = 10⁻³ m³
define_composed_unit!(Milliliter, "mL", 0.000_001);             // 1 mL = 1 cm³

// Imperial volume units
define_composed_unit!(CubicInch, "in³", 0.000_016_387_064);     // 1 in³ ≈ 16.387 cm³
define_composed_unit!(CubicFoot, "ft³", 0.028_316_846_592);     // 1 ft³ ≈ 28.317 L
define_composed_unit!(CubicYard, "yd³", 0.764_554_857_984);     // 1 yd³ ≈ 0.765 m³
define_composed_unit!(CubicMile, "mi³", 4_168_181_825_440.58);  // 1 mi³ in m³

// US liquid volume units
define_composed_unit!(Gallon, "gal", 0.003_785_411_784);        // US gallon
define_composed_unit!(Quart, "qt", 0.000_946_352_946);          // US quart
define_composed_unit!(Pint, "pt", 0.000_473_176_473);           // US pint
define_composed_unit!(FluidOunce, "fl oz", 0.000_029_573_53);   // US fluid ounce

// Astronomical volume units
define_composed_unit!(CubicAstronomicalUnit, "AU³", 3.347_928_976e33);  // 1 AU³ in m³
define_composed_unit!(CubicLightYear, "ly³", 8.467_731_016e47);         // 1 ly³ in m³  
define_composed_unit!(CubicParsec, "pc³", 2.937_799_588e49);            // 1 pc³ in m³

// Generate prefixed aliases
define_prefixed_aliases! {
    CubicMeter => [Kilo, Mega, Giga, Centi, Milli, Micro],
    Liter => [Kilo, Centi, Milli, Micro],
}

// Convenience constructors
impl_quantity_constructors!(
    Volume,
    CubicMeter, CubicKilometer, CubicCentimeter, CubicMillimeter,
    Liter, Milliliter,
    CubicInch, CubicFoot, CubicYard, CubicMile,
    Gallon, Quart, Pint, FluidOunce,
    CubicAstronomicalUnit, CubicLightYear, CubicParsec
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_units() {
        let vol1 = Volume::<CubicMeter>::new(1000.0);
        let vol2 = Volume::<(Meter, Meter, Meter)>::new(1000.0);
        
        assert_eq!(vol1.value(), 1000.0);
        assert_eq!(vol2.value(), 1000.0);
    }

    #[test]
    fn test_volume_conversions() {
        let vol_m3 = Volume::<CubicMeter>::new(1.0);
        let vol_l: Volume<Liter> = vol_m3.convert_to();
        
        // 1 m³ = 1000 L
        assert_eq!(vol_l.value(), 1000.0);
    }

    #[test]
    fn test_liter_to_cubic_cm() {
        let vol_l = Volume::<Liter>::new(1.0);
        let vol_cm3: Volume<CubicCentimeter> = vol_l.convert_to();
        
        // 1 L = 1000 cm³
        assert_eq!(vol_cm3.value(), 1000.0);
    }

    #[test]
    fn test_gallon_conversion() {
        let vol_gal = Volume::<Gallon>::new(1.0);
        let vol_l: Volume<Liter> = vol_gal.convert_to();
        
        // 1 US gallon ≈ 3.785 L
        assert!((vol_l.value() - 3.785_411_784).abs() < 0.001);
    }

    #[test]
    fn test_astronomical_volume() {
        let vol_au = Volume::<CubicAstronomicalUnit>::new(1.0);
        let vol_km3: Volume<CubicKilometer> = vol_au.convert_to();
        
        // 1 AU³ should be enormous in km³
        assert!(vol_km3.value() > 1e24);
    }

    #[test]
    fn test_dimensional_analysis_result() {
        // This should work: Area × Distance = Volume
        let area = Area::<SquareMeter>::new(100.0);  // 10m × 10m
        let height = Distance::<Meter>::new(5.0);
        let volume = area * height;
        
        // Result should be Volume type with value 500
        assert_eq!(volume.value(), 500.0);
        
        // Should be able to convert to specific volume units
        let volume_l: Volume<Liter> = volume.convert_to();
        assert_eq!(volume_l.value(), 500_000.0); // 500 m³ = 500,000 L
    }

    #[test]
    fn test_triple_multiplication() {
        // This should work: Distance × Distance × Distance = Volume
        let length = Distance::<Meter>::new(2.0);
        let width = Distance::<Meter>::new(3.0);
        let height = Distance::<Meter>::new(4.0);
        
        let volume = length * width * height;
        
        // Result should be Volume type with value 24
        assert_eq!(volume.value(), 24.0);
    }

    #[test]
    fn test_milliliter_equivalence() {
        let vol_ml = Volume::<Milliliter>::new(500.0);
        let vol_cm3: Volume<CubicCentimeter> = vol_ml.convert_to();
        
        // 1 mL = 1 cm³
        assert_eq!(vol_cm3.value(), 500.0);
    }
}