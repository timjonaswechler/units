#![allow(non_snake_case)]
//! Time units

use crate::*;

// Re-export the type alias from core
pub use crate::core::Time;

// Define base time units
define_units_for_dimension! {
    Time => {
        Second = "s", 1.0,
        Minute = "min", 60.0,
        Hour = "h", 3600.0,
        Day = "d", 86400.0,
        Week = "week", 604800.0,
        Year = "yr", 31_557_600.0,  // Julian year

        // Astronomical time scales
        SiderealDay = "sid_day", 86164.0905,
        SiderealYear = "sid_yr", 31_558_149.5,
        TropicalYear = "trop_yr", 31_556_925.216,

        // Planck time
        PlanckTime = "tₚ", 5.391_247e-44,

        // Common fractions
        Millisecond = "ms", 0.001,
        Microsecond = "μs", 1e-6,
        Nanosecond = "ns", 1e-9,
        Picosecond = "ps", 1e-12,
    }
}

// Generate prefixed aliases
define_prefixed_aliases! {
    Second => [Milli, Micro, Nano, Pico, Femto],
    Year => [Kilo, Mega, Giga],
}

// Convenience constructors
impl_quantity_constructors!(
    Time,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Year,
    SiderealDay,
    SiderealYear,
    TropicalYear,
    PlanckTime,
    Millisecond,
    Microsecond,
    Nanosecond,
    Picosecond
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_units() {
        let s = Time::<Second>::new(1.0);
        let min = Time::<Minute>::new(1.0);
        let h = Time::<Hour>::new(1.0);

        // Test conversions
        let s_from_min: Time<Second> = min.convert_to();
        assert_eq!(s_from_min.value(), 60.0);

        let s_from_h: Time<Second> = h.convert_to();
        assert_eq!(s_from_h.value(), 3600.0);
    }

    #[test]
    fn test_astronomical_time() {
        let year = Time::<Year>::new(1.0);
        let day = Time::<Day>::new(365.25);

        let year_in_seconds: Time<Second> = year.convert_to();
        let day_in_seconds: Time<Second> = day.convert_to();

        // Should be approximately equal (Julian year vs 365.25 days)
        let diff = (year_in_seconds.value() - day_in_seconds.value()).abs();
        assert!(diff < 1000.0); // Within 1000 seconds
    }

    #[test]
    fn test_small_time_units() {
        let ms = Time::<Millisecond>::new(1000.0);
        let s: Time<Second> = ms.convert_to();
        assert_eq!(s.value(), 1.0);

        let ns = Time::<Nanosecond>::new(1e9);
        let s2: Time<Second> = ns.convert_to();
        assert_eq!(s2.value(), 1.0);
    }

    #[test]
    fn test_convenience_constructors() {
        let t1 = Time::<Second>::Second(1.0);
        let t2 = Time::<Minute>::Minute(1.0 / 60.0);

        assert_eq!(t1.value(), 1.0);

        let t1_as_min: Time<Minute> = t1.convert_to();
        assert!((t1_as_min.value() - t2.value()).abs() < 1e-10);
    }
}
