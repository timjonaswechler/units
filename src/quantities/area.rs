//! Area units for surface calculations in stellar systems

use crate::*;

// Re-export the type alias from core
pub use crate::core::Area;

// Define area units as aliases
define_composed_unit!(SquareMeter, "m²", 1.0);  // SI base unit

// Metric area units
define_composed_unit!(SquareKilometer, "km²", 1_000_000.0);  // 1 km² = 10⁶ m²
define_composed_unit!(SquareCentimeter, "cm²", 0.0001);      // 1 cm² = 10⁻⁴ m²
define_composed_unit!(SquareMillimeter, "mm²", 0.000_001);   // 1 mm² = 10⁻⁶ m²

// Imperial area units
define_composed_unit!(SquareInch, "in²", 0.00064516);        // 1 in² = 6.4516 cm²
define_composed_unit!(SquareFoot, "ft²", 0.092_903_04);      // 1 ft² = 0.092903 m²
define_composed_unit!(SquareYard, "yd²", 0.836_127_36);      // 1 yd² = 0.836127 m²
define_composed_unit!(SquareMile, "mi²", 2_589_988.110_336); // 1 mi² = 2.59 km²

// Agricultural and land units
define_composed_unit!(Acre, "acre", 4046.856_422_4);         // 1 acre = 4047 m²
define_composed_unit!(Hectare, "ha", 10_000.0);              // 1 ha = 10,000 m²

// Astronomical area units
define_composed_unit!(SquareAstronomicalUnit, "AU²", 2.235_695_4e22);  // 1 AU² in m²
define_composed_unit!(SquareLightYear, "ly²", 8.950_681_9e31);         // 1 ly² in m²
define_composed_unit!(SquareParsec, "pc²", 9.521_554_4e31);            // 1 pc² in m²

// Cross-sectional units  
define_composed_unit!(Barn, "b", 1e-28);                     // 1 barn = 10⁻²⁸ m² (nuclear physics)
define_composed_unit!(MilliBarn, "mb", 1e-31);               // 1 mbarn = 10⁻³¹ m²
define_composed_unit!(MicroBarn, "μb", 1e-34);               // 1 μbarn = 10⁻³⁴ m²

// Generate prefixed aliases
define_prefixed_aliases! {
    SquareMeter => [Kilo, Mega, Giga, Centi, Milli, Micro],
}

// Convenience constructors
impl_quantity_constructors!(
    Area,
    SquareMeter, SquareKilometer, SquareCentimeter, SquareMillimeter,
    SquareInch, SquareFoot, SquareYard, SquareMile,
    Acre, Hectare,
    SquareAstronomicalUnit, SquareLightYear, SquareParsec,
    Barn, MilliBarn, MicroBarn
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_units() {
        let area1 = Area::<SquareMeter>::new(100.0);
        let area2 = Area::<(Meter, Meter)>::new(100.0);
        
        assert_eq!(area1.value(), 100.0);
        assert_eq!(area2.value(), 100.0);
    }

    #[test]
    fn test_area_conversions() {
        let area_m2 = Area::<SquareMeter>::new(1.0);
        let area_cm2: Area<SquareCentimeter> = area_m2.convert_to();
        
        // 1 m² = 10,000 cm²
        assert_eq!(area_cm2.value(), 10_000.0);
    }

    #[test]
    fn test_hectare_conversion() {
        let area_ha = Area::<Hectare>::new(1.0);
        let area_m2: Area<SquareMeter> = area_ha.convert_to();
        
        // 1 hectare = 10,000 m²
        assert_eq!(area_m2.value(), 10_000.0);
    }

    #[test]
    fn test_acre_conversion() {
        let area_acre = Area::<Acre>::new(1.0);
        let area_m2: Area<SquareMeter> = area_acre.convert_to();
        
        // 1 acre ≈ 4047 m²
        assert!((area_m2.value() - 4046.856_422_4).abs() < 0.1);
    }

    #[test]
    fn test_astronomical_area() {
        let area_au = Area::<SquareAstronomicalUnit>::new(1.0);
        let area_km2: Area<SquareKilometer> = area_au.convert_to();
        
        // 1 AU² should be huge in km²
        assert!(area_km2.value() > 1e16);
    }

    #[test]
    fn test_nuclear_cross_section() {
        let cross_section = Area::<Barn>::new(1.0);
        let area_m2: Area<SquareMeter> = cross_section.convert_to();
        
        // 1 barn = 10⁻²⁸ m²
        assert_eq!(area_m2.value(), 1e-28);
    }

    #[test]
    fn test_dimensional_analysis_result() {
        // This should work: Distance × Distance = Area
        let length = Distance::<Meter>::new(5.0);
        let width = Distance::<Meter>::new(3.0);
        let area = length * width;
        
        // Result should be Area type with value 15
        assert_eq!(area.value(), 15.0);
        
        // Should be able to convert to specific area units
        let area_cm2: Area<SquareCentimeter> = area.convert_to();
        assert_eq!(area_cm2.value(), 150_000.0); // 15 m² = 150,000 cm²
    }
}