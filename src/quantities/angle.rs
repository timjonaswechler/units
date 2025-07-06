#![allow(non_snake_case)]
//! Angular measurement units for orbital mechanics and stellar rotation

use crate::*;

// Re-export the type alias from core
pub use crate::core::Angle;

// Define angle units as aliases
define_composed_unit!(Radian, "rad", 1.0); // SI base unit (dimensionless)

// Degree-based angular units
define_composed_unit!(Degree, "°", 0.017_453_292_519_943_295); // π/180 radians
define_composed_unit!(Arcminute, "'", 0.000_290_888_208_665_721_5); // 1/60 degree
define_composed_unit!(Arcsecond, "\"", 0.000_004_848_136_811_095_36); // 1/3600 degree
define_composed_unit!(Milliarcsecond, "mas", 0.000_000_004_848_136_811_095_36); // 1/1000 arcsecond

// Full rotation units
define_composed_unit!(Revolution, "rev", 6.283_185_307_179_586); // 2π radians
define_composed_unit!(Turn, "turn", 6.283_185_307_179_586); // 2π radians (alias)

// Gradian (400 gradians = 360 degrees)
define_composed_unit!(Gradian, "grad", 0.015_707_963_267_948_966); // π/200 radians

// Astronomical angles for parallax and proper motion
define_composed_unit!(Microarcsecond, "μas", 0.000_000_000_004_848_136_811); // 1/1,000,000 arcsecond

// Generate prefixed aliases
define_prefixed_aliases! {
    Degree => [Milli, Micro],
    Arcsecond => [Milli, Micro],
}

// Convenience constructors
impl_quantity_constructors!(
    Angle,
    Radian,
    Degree,
    Arcminute,
    Arcsecond,
    Milliarcsecond,
    Microarcsecond,
    Revolution,
    Turn,
    Gradian
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_units() {
        let angle1 = Angle::<Radian>::new(1.0);
        let angle2 = Angle::<()>::new(1.0); // Dimensionless tuple syntax

        assert_eq!(angle1.value(), 1.0);
        assert_eq!(angle2.value(), 1.0);
    }

    #[test]
    fn test_angle_conversions() {
        let angle_rad = Angle::<Radian>::new(std::f64::consts::PI);
        let angle_deg: Angle<Degree> = angle_rad.convert_to();

        // π radians = 180 degrees
        assert!((angle_deg.value() - 180.0).abs() < 1e-10);
    }

    #[test]
    fn test_degree_to_arcsecond() {
        let angle_deg = Angle::<Degree>::new(1.0);
        let angle_arcsec: Angle<Arcsecond> = angle_deg.convert_to();

        // 1 degree = 3600 arcseconds
        assert!((angle_arcsec.value() - 3600.0).abs() < 1e-6);
    }

    #[test]
    fn test_milliarcsecond_precision() {
        let parallax = Angle::<Milliarcsecond>::new(768.5); // Proxima Centauri
        let parallax_arcsec: Angle<Arcsecond> = parallax.convert_to();

        // 768.5 mas = 0.7685 arcseconds
        assert!((parallax_arcsec.value() - 0.7685).abs() < 1e-10);
    }

    #[test]
    fn test_revolution_conversion() {
        let full_turn = Angle::<Revolution>::new(1.0);
        let full_turn_deg: Angle<Degree> = full_turn.convert_to();

        // 1 revolution = 360 degrees
        assert!((full_turn_deg.value() - 360.0).abs() < 1e-10);
    }

    #[test]
    fn test_gradian_conversion() {
        let right_angle_grad = Angle::<Gradian>::new(100.0);
        let right_angle_deg: Angle<Degree> = right_angle_grad.convert_to();

        // 100 gradians = 90 degrees
        assert!((right_angle_deg.value() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_small_angle_precision() {
        let tiny_angle = Angle::<Microarcsecond>::new(1.0);
        let tiny_rad: Angle<Radian> = tiny_angle.convert_to();

        // 1 μas should be extremely small in radians
        assert!(tiny_rad.value() < 1e-10);
        assert!(tiny_rad.value() > 0.0);
    }

    #[test]
    fn test_astronomical_parallax() {
        // Test realistic stellar parallax values
        let sirius_parallax = Angle::<Milliarcsecond>::new(379.21);
        let sirius_deg: Angle<Degree> = sirius_parallax.convert_to();

        // Should be a very small angle in degrees
        assert!(sirius_deg.value() < 0.001);
        assert!(sirius_deg.value() > 0.0);
    }

    #[test]
    fn test_dimensionless_ratios() {
        let angle1 = Angle::<Degree>::new(90.0);
        let angle2 = Angle::<Degree>::new(45.0);
        let ratio = angle1 / angle2;

        // 90° ÷ 45° = 2 (dimensionless)
        assert!((ratio - 2.0).abs() < 1e-10);
    }
}
