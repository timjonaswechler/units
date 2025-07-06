#![allow(non_snake_case)]
//! Frequency units for oscillations, waves, and electromagnetic radiation

use crate::*;

// Re-export the type alias from core
pub use crate::core::Frequency;

// Define frequency units as aliases
define_composed_unit!(Hertz, "Hz", 1.0); // SI base unit

// Standard frequency units
define_composed_unit!(Kilohertz, "kHz", 1_000.0);
define_composed_unit!(Megahertz, "MHz", 1_000_000.0);
define_composed_unit!(Gigahertz, "GHz", 1_000_000_000.0);
define_composed_unit!(Terahertz, "THz", 1_000_000_000_000.0);

// Small frequency units
define_composed_unit!(Millihertz, "mHz", 0.001);
define_composed_unit!(Microhertz, "μHz", 0.000_001);
define_composed_unit!(Nanohertz, "nHz", 0.000_000_001);

// Rotational frequency units
define_composed_unit!(RevolutionsPerSecond, "rps", 1.0); // 1 rps = 1 Hz
define_composed_unit!(RevolutionsPerMinute, "rpm", 1.0 / 60.0); // 1 rpm = 1/60 Hz

// Astronomical frequency units
define_composed_unit!(CyclesPerDay, "cpd", 1.0 / 86_400.0); // 1 cycle/day
define_composed_unit!(CyclesPerYear, "cpy", 1.0 / 31_557_600.0); // 1 cycle/year (365.25 days)

// Angular frequency equivalents (for completeness, though dimensionally different)
define_composed_unit!(
    RadiansPerSecond,
    "rad/s",
    1.0 / (2.0 * std::f64::consts::PI)
); // ω = 2πf

// Generate prefixed aliases
define_prefixed_aliases! {
    Hertz => [Kilo, Mega, Giga, Tera, Milli, Micro, Nano],
}

// Convenience constructors
impl_quantity_constructors!(
    Frequency,
    Hertz,
    Kilohertz,
    Megahertz,
    Gigahertz,
    Terahertz,
    Millihertz,
    Microhertz,
    Nanohertz,
    RevolutionsPerSecond,
    RevolutionsPerMinute,
    CyclesPerDay,
    CyclesPerYear,
    RadiansPerSecond
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_units() {
        let freq1 = Frequency::<Hertz>::new(1000.0);
        let freq2 = Frequency::<((), (), Second)>::new(1000.0); // 1/T dimension

        assert_eq!(freq1.value(), 1000.0);
        assert_eq!(freq2.value(), 1000.0);
    }

    #[test]
    fn test_frequency_conversions() {
        let freq_hz = Frequency::<Hertz>::new(1000.0);
        let freq_khz: Frequency<Kilohertz> = freq_hz.convert_to();

        // 1000 Hz = 1 kHz
        assert_eq!(freq_khz.value(), 1.0);
    }

    #[test]
    fn test_large_frequency_conversions() {
        let freq_ghz = Frequency::<Gigahertz>::new(2.4); // WiFi frequency
        let freq_hz: Frequency<Hertz> = freq_ghz.convert_to();

        // 2.4 GHz = 2.4e9 Hz
        assert_eq!(freq_hz.value(), 2.4e9);
    }

    #[test]
    fn test_rpm_conversion() {
        let rotation = Frequency::<RevolutionsPerMinute>::new(60.0);
        let rotation_hz: Frequency<Hertz> = rotation.convert_to();

        // 60 rpm = 1 Hz
        assert_eq!(rotation_hz.value(), 1.0);
    }

    #[test]
    fn test_astronomical_frequencies() {
        let daily_cycle = Frequency::<CyclesPerDay>::new(1.0);
        let daily_hz: Frequency<Hertz> = daily_cycle.convert_to();

        // 1 cycle/day should be very small in Hz
        assert!(daily_hz.value() < 1e-4);
        assert!(daily_hz.value() > 1e-6);
    }

    #[test]
    fn test_pulsar_frequencies() {
        // Typical pulsar: ~1000 Hz
        let pulsar = Frequency::<Hertz>::new(1000.0);
        let pulsar_khz: Frequency<Kilohertz> = pulsar.convert_to();

        assert_eq!(pulsar_khz.value(), 1.0);

        // Millisecond pulsar: ~500 Hz
        let ms_pulsar = Frequency::<Hertz>::new(500.0);
        let ms_rpm: Frequency<RevolutionsPerMinute> = ms_pulsar.convert_to();

        // 500 Hz = 30,000 rpm
        assert_eq!(ms_rpm.value(), 30_000.0);
    }

    #[test]
    fn test_small_frequency_precision() {
        let micro_freq = Frequency::<Microhertz>::new(1.0);
        let micro_hz: Frequency<Hertz> = micro_freq.convert_to();

        // 1 μHz = 1e-6 Hz
        assert_eq!(micro_hz.value(), 1e-6);
    }

    #[test]
    fn test_angular_frequency_relation() {
        let freq = Frequency::<Hertz>::new(1.0);
        let angular: Frequency<RadiansPerSecond> = freq.convert_to();

        // f = 1 Hz → ω = 2π rad/s, but as frequency unit: 1 Hz = 2π "rad/s frequency"
        // Since RadiansPerSecond is defined as 1/(2π) Hz, 1 Hz = 2π of these units
        assert!((angular.value() - 2.0 * std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_electromagnetic_spectrum() {
        // Visible light frequency (~500 THz for green)
        let green_light = Frequency::<Terahertz>::new(500.0);
        let green_hz: Frequency<Hertz> = green_light.convert_to();

        // 500 THz = 5e14 Hz
        assert_eq!(green_hz.value(), 5e14);
    }

    #[test]
    fn test_dimensionless_ratios() {
        let freq1 = Frequency::<Hertz>::new(1000.0);
        let freq2 = Frequency::<Hertz>::new(500.0);
        let ratio = freq1 / freq2;

        // 1000 Hz ÷ 500 Hz = 2 (dimensionless)
        assert!((ratio - 2.0).abs() < 1e-10);
    }
}
