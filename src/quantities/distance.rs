#![allow(non_snake_case)]
//! Distance/Length units

use crate::*;

// Re-export the type alias from core
pub use crate::core::Distance;

// Define base length units
define_units_for_dimension! {
    Length => {
        Meter = "m", 1.0,
        Centimeter = "cm", 0.01,
        Millimeter = "mm", 0.001,
        Kilometer = "km", 1000.0,
        Inch = "in", 0.0254,
        Foot = "ft", 0.3048,
        Yard = "yd", 0.9144,
        Mile = "mi", 1609.344,
        NauticalMile = "nmi", 1852.0,

        // Astronomical units
        AstronomicalUnit = "AU", 1.495_978_707e11,
        LightYear = "ly", 9.460_730_472_580_8e15,
        Parsec = "pc", 3.085_677_581e16,

        // Planck and atomic scales
        PlanckLength = "ℓₚ", 1.616_255e-35,
        BohrRadius = "a₀", 5.291_772_109e-11,

        // Planetary radii
        EarthRadius = "R⊕", 6.371e6,
        SolarRadius = "R☉", 6.96e8,
        JupiterRadius = "R♃", 7.1492e7,
    }
}

// Generate prefixed aliases
define_prefixed_aliases! {
    Meter => [Kilo, Centi, Milli, Micro, Nano, Pico],
    Parsec => [Kilo, Mega, Giga],
}

// Convenience constructors
impl_quantity_constructors!(
    Distance,
    Meter,
    Centimeter,
    Millimeter,
    Kilometer,
    Inch,
    Foot,
    Yard,
    Mile,
    NauticalMile,
    AstronomicalUnit,
    LightYear,
    Parsec,
    PlanckLength,
    BohrRadius,
    EarthRadius,
    SolarRadius,
    JupiterRadius
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_units() {
        let m = Distance::<Meter>::new(1.0);
        let km = Distance::<Kilometer>::new(1.0);
        let cm = Distance::<Centimeter>::new(100.0);

        // Test conversions
        let m_from_km: Distance<Meter> = km.convert_to();
        assert_eq!(m_from_km.value(), 1000.0);

        let m_from_cm: Distance<Meter> = cm.convert_to();
        assert_eq!(m_from_cm.value(), 1.0);
    }

    #[test]
    fn test_astronomical_units() {
        let au = Distance::<AstronomicalUnit>::new(1.0);
        let m: Distance<Meter> = au.convert_to();
        assert!((m.value() - 1.495_978_707e11).abs() < 1e5);
    }

    #[test]
    fn test_prefixed_units() {
        let km = Distance::<Kilometer>::new(1.0);
        let m: Distance<Meter> = km.convert_to();
        assert_eq!(m.value(), 1000.0);
    }

    #[test]
    fn test_convenience_constructors() {
        let d1 = Distance::<Meter>::Meter(100.0);
        let d2 = Distance::<Kilometer>::Kilometer(0.1);

        assert_eq!(d1.value(), 100.0);
        assert_eq!(d2.value(), 0.1);

        // Should be equivalent
        let d1_as_km: Distance<Kilometer> = d1.convert_to();
        assert!((d1_as_km.value() - d2.value()).abs() < 1e-10);
    }
}
