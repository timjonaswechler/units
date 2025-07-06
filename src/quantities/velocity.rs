#![allow(non_snake_case)]
//! Velocity units

use crate::*;

// Re-export the type alias from core
pub use crate::core::Velocity;

// Define composed velocity units as aliases
define_composed_unit!(MeterPerSecond, "m/s", 1.0);
define_composed_unit!(KilometerPerHour, "km/h", 1000.0 / 3600.0);
define_composed_unit!(MilePerHour, "mph", 0.447_04);
define_composed_unit!(Knot, "kn", 0.514_444);
define_composed_unit!(FootPerSecond, "ft/s", 0.3048);

// Physics constants
define_composed_unit!(SpeedOfLight, "c", 299_792_458.0);

// Generate prefixed aliases for the base composed units
define_prefixed_aliases! {
    MeterPerSecond => [Kilo, Milli, Micro],
}

// Convenience constructors
impl_quantity_constructors!(
    Velocity,
    MeterPerSecond,
    KilometerPerHour,
    MilePerHour,
    Knot,
    FootPerSecond,
    SpeedOfLight
);

// Type aliases for common combinations using tuple syntax
pub type MeterPerSecondTuple = (Meter, Second);
pub type KilometerPerHourTuple = (crate::quantities::distance::Kilometer, Hour);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_units() {
        let mps = Velocity::<MeterPerSecond>::new(10.0);
        let kmh = Velocity::<KilometerPerHour>::new(36.0);

        // 36 km/h should equal 10 m/s
        let mps_from_kmh: Velocity<MeterPerSecond> = kmh.convert_to();
        assert!((mps_from_kmh.value() - mps.value()).abs() < 0.01);
    }

    #[test]
    fn test_tuple_syntax() {
        let v1 = Velocity::<(Meter, Second)>::new(10.0);
        let v2 = Velocity::<MeterPerSecond>::new(10.0);

        // Both should have same SI value
        assert_eq!(v1.to_si(), 10.0);
        assert_eq!(v2.to_si(), 10.0);
    }

    #[test]
    fn test_imperial_velocity() {
        let mph = Velocity::<MilePerHour>::new(60.0);
        let mps: Velocity<MeterPerSecond> = mph.convert_to();

        // 60 mph ≈ 26.8 m/s
        assert!((mps.value() - 26.8224).abs() < 0.1);
    }

    #[test]
    fn test_nautical_velocity() {
        let knot = Velocity::<Knot>::new(1.0);
        let mps: Velocity<MeterPerSecond> = knot.convert_to();

        // 1 knot ≈ 0.514 m/s
        assert!((mps.value() - 0.514_444).abs() < 0.001);
    }

    #[test]
    fn test_speed_of_light() {
        let c = Velocity::<SpeedOfLight>::new(1.0);
        let mps: Velocity<MeterPerSecond> = c.convert_to();

        assert_eq!(mps.value(), 299_792_458.0);
    }

    #[test]
    fn test_convenience_constructors() {
        let v1 = Velocity::<MeterPerSecond>::MeterPerSecond(25.0);
        let v2 = Velocity::<KilometerPerHour>::KilometerPerHour(90.0);

        assert_eq!(v1.value(), 25.0);
        assert_eq!(v2.value(), 90.0);

        // 90 km/h = 25 m/s
        let v2_as_mps: Velocity<MeterPerSecond> = v2.convert_to();
        assert!((v2_as_mps.value() - v1.value()).abs() < 0.01);
    }
}
