#![allow(non_snake_case)]
//! Acceleration units

use crate::*;

// Re-export the type alias from core
pub use crate::core::Acceleration;

// Define composed acceleration units as aliases
define_composed_unit!(MeterPerSecondSquared, "m/s²", 1.0);
define_composed_unit!(
    KilometerPerHourSquared,
    "km/h²",
    (1000.0_f64 / 3600.0_f64).powi(2)
);
define_composed_unit!(FootPerSecondSquared, "ft/s²", 0.3048);

// Generate prefixed aliases
define_prefixed_aliases! {
    MeterPerSecondSquared => [Kilo, Milli, Micro],
}

// Convenience constructors
impl_quantity_constructors!(
    Acceleration,
    MeterPerSecondSquared,
    KilometerPerHourSquared,
    FootPerSecondSquared
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceleration_units() {
        let acc1 = Acceleration::<MeterPerSecondSquared>::new(9.81);
        let acc2 = Acceleration::<(Meter, Second)>::new(9.81);

        assert_eq!(acc1.value(), 9.81);
        assert_eq!(acc2.value(), 9.81);
    }

    #[test]
    fn test_acceleration_conversions() {
        let acc_si = Acceleration::<MeterPerSecondSquared>::new(1.0);
        let acc_imperial: Acceleration<FootPerSecondSquared> = acc_si.convert_to();

        // 1 m/s² ≈ 3.28 ft/s²
        assert!((acc_imperial.value() - 3.28084).abs() < 0.001);
    }
}
