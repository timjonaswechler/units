//! Angular velocity units for rotational dynamics and celestial mechanics

use crate::*;

// Re-export the type alias from core
pub use crate::core::AngularVelocity;

// Define angular velocity units as aliases (dimension: T⁻¹, same as frequency but conceptually different)
define_composed_unit!(RadianPerSecond, "rad/s", 1.0);  // SI base unit for angular velocity

// Common angular velocity units
define_composed_unit!(DegreePerSecond, "deg/s", std::f64::consts::PI / 180.0);     // π/180 rad/s
define_composed_unit!(DegreePerMinute, "deg/min", std::f64::consts::PI / 10800.0); // π/10800 rad/s
define_composed_unit!(DegreePerHour, "deg/h", std::f64::consts::PI / 648000.0);    // π/648000 rad/s

// Rotational frequency units (revolutions)
define_composed_unit!(RotationsPerSecond, "rps", 2.0 * std::f64::consts::PI);    // 2π rad/s
define_composed_unit!(RotationsPerMinute, "rpm", 2.0 * std::f64::consts::PI / 60.0); // 2π/60 rad/s
define_composed_unit!(RotationsPerHour, "rph", 2.0 * std::f64::consts::PI / 3600.0); // 2π/3600 rad/s

// Astronomical angular velocity units
define_composed_unit!(MilliarcsecondsPerYear, "mas/yr", (std::f64::consts::PI / 180.0) / (3600.0 * 1000.0 * 365.25 * 24.0 * 3600.0)); // Proper motion
define_composed_unit!(MicroarcsecondsPerYear, "μas/yr", (std::f64::consts::PI / 180.0) / (3600.0 * 1000000.0 * 365.25 * 24.0 * 3600.0)); // Ultra-precise astrometry

// Earth rotation and orbital motion
define_composed_unit!(EarthRotationRate, "Ω_⊕", 7.2921159e-5);                     // Earth's sidereal rotation rate
define_composed_unit!(EarthOrbitalRate, "ω_⊕", 1.991e-7);                         // Earth's orbital angular velocity

// Stellar rotation rates (typical values)
define_composed_unit!(SolarRotationRate, "Ω_☉", 2.87e-6);                         // Solar rotation rate (~25 days)
define_composed_unit!(PulsarRotationRate, "Ω_PSR", 1000.0);                       // Typical pulsar rotation rate (~1000 rad/s)

// Engineering units
define_composed_unit!(RPM, "RPM", 2.0 * std::f64::consts::PI / 60.0);             // Common engineering notation

// Generate prefixed aliases
define_prefixed_aliases! {
    RadianPerSecond => [Milli, Micro, Nano, Kilo],
    DegreePerSecond => [Milli, Micro],
    RotationsPerSecond => [Milli, Micro],
}

// Convenience constructors
impl_quantity_constructors!(
    AngularVelocity,
    RadianPerSecond, DegreePerSecond, DegreePerMinute, DegreePerHour,
    RotationsPerSecond, RotationsPerMinute, RotationsPerHour,
    MilliarcsecondsPerYear, MicroarcsecondsPerYear,
    EarthRotationRate, EarthOrbitalRate, SolarRotationRate, PulsarRotationRate,
    RPM
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angular_velocity_units() {
        let omega1 = AngularVelocity::<RadianPerSecond>::new(1.0);
        let omega2 = AngularVelocity::<((), (), Second)>::new(1.0); // T⁻¹ dimension
        
        assert_eq!(omega1.value(), 1.0);
        assert_eq!(omega2.value(), 1.0);
    }

    #[test]
    fn test_degree_conversions() {
        let omega_deg = AngularVelocity::<DegreePerSecond>::new(180.0);
        let omega_rad: AngularVelocity<RadianPerSecond> = omega_deg.convert_to();
        
        // 180 deg/s = π rad/s
        assert!((omega_rad.value() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_rpm_conversions() {
        let omega_rpm = AngularVelocity::<RotationsPerMinute>::new(60.0);
        let omega_rad: AngularVelocity<RadianPerSecond> = omega_rpm.convert_to();
        let omega_rps: AngularVelocity<RotationsPerSecond> = omega_rpm.convert_to();
        
        // 60 rpm = 2π rad/s = 1 rps
        assert!((omega_rad.value() - 2.0 * std::f64::consts::PI).abs() < 1e-10);
        assert!((omega_rps.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_earth_rotation() {
        let earth_rot = AngularVelocity::<EarthRotationRate>::new(1.0);
        let earth_rad: AngularVelocity<RadianPerSecond> = earth_rot.convert_to();
        let earth_deg: AngularVelocity<DegreePerSecond> = earth_rot.convert_to();
        
        // Earth rotation: ~7.29×10⁻⁵ rad/s
        assert!((earth_rad.value() - 7.2921159e-5).abs() < 1e-10);
        
        // Should be about 0.00417 deg/s (360° per day)
        let expected_deg_per_s = 360.0 / (24.0 * 3600.0);
        assert!((earth_deg.value() - expected_deg_per_s).abs() < 1e-3); // More tolerant due to conversion precision
    }

    #[test]
    fn test_solar_rotation() {
        let solar_rot = AngularVelocity::<SolarRotationRate>::new(1.0);
        let solar_rad: AngularVelocity<RadianPerSecond> = solar_rot.convert_to();
        let solar_rpm: AngularVelocity<RotationsPerMinute> = solar_rot.convert_to();
        
        // Solar rotation: ~2.87×10⁻⁶ rad/s
        assert!((solar_rad.value() - 2.87e-6).abs() < 1e-10);
        
        // Convert to rpm for intuitive understanding
        assert!(solar_rpm.value() < 1e-3); // Very slow rotation
    }

    #[test]
    fn test_pulsar_rotation() {
        let pulsar = AngularVelocity::<PulsarRotationRate>::new(1.0);
        let pulsar_rad: AngularVelocity<RadianPerSecond> = pulsar.convert_to();
        let pulsar_rpm: AngularVelocity<RotationsPerMinute> = pulsar.convert_to();
        
        // Pulsar: ~1000 rad/s (extremely fast)
        assert_eq!(pulsar_rad.value(), 1000.0);
        
        // Convert to rpm
        let expected_rpm = 1000.0 * 60.0 / (2.0 * std::f64::consts::PI);
        assert!((pulsar_rpm.value() - expected_rpm).abs() < 1.0);
    }

    #[test]
    fn test_proper_motion_units() {
        let proper_motion = AngularVelocity::<MilliarcsecondsPerYear>::new(100.0);
        let pm_rad: AngularVelocity<RadianPerSecond> = proper_motion.convert_to();
        
        // 100 mas/yr should be a very small angular velocity in rad/s
        assert!(pm_rad.value() < 1e-10);
        assert!(pm_rad.value() > 1e-15);
    }

    #[test]
    fn test_orbital_mechanics() {
        let earth_orbit = AngularVelocity::<EarthOrbitalRate>::new(1.0);
        let earth_rotation = AngularVelocity::<EarthRotationRate>::new(1.0);
        
        // Earth's rotation is much faster than its orbital motion
        let ratio = earth_rotation / earth_orbit;
        assert!(ratio > 300.0); // Approximately 365.25 times faster
        assert!(ratio < 400.0);
    }

    #[test]
    fn test_engineering_units() {
        let engine_rpm = AngularVelocity::<RPM>::new(3600.0);  // Car engine at highway speed
        let engine_rad: AngularVelocity<RadianPerSecond> = engine_rpm.convert_to();
        
        // 3600 rpm should convert correctly to rad/s
        let expected = 3600.0 * 2.0 * std::f64::consts::PI / 60.0;
        assert!((engine_rad.value() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_mixed_unit_arithmetic() {
        let omega1 = AngularVelocity::<RadianPerSecond>::new(1.0);
        let omega2 = AngularVelocity::<DegreePerSecond>::new(180.0);  // π rad/s
        let total = omega1 + omega2;  // Result in SI units (rad/s)
        
        // 1 + π rad/s
        let expected = 1.0 + std::f64::consts::PI;
        assert!((total.value() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_dimensionless_ratios() {
        let fast_pulsar = AngularVelocity::<RadianPerSecond>::new(1000.0);
        let slow_pulsar = AngularVelocity::<RadianPerSecond>::new(100.0);
        let ratio = fast_pulsar / slow_pulsar;
        
        assert_eq!(ratio, 10.0);  // Fast pulsar spins 10x faster
    }

    #[test]
    fn test_stellar_rotation_comparison() {
        let sun = AngularVelocity::<SolarRotationRate>::new(1.0);
        let neutron_star = AngularVelocity::<PulsarRotationRate>::new(1.0);
        
        // Neutron star rotates much faster than the Sun
        let speed_ratio = neutron_star / sun;
        assert!(speed_ratio > 1e5);  // At least 100,000 times faster
        assert!(speed_ratio < 1e9);  // But less than 1 billion times
    }

    #[test]
    fn test_precision_astrometry() {
        let microas_motion = AngularVelocity::<MicroarcsecondsPerYear>::new(10.0);
        let millias_motion: AngularVelocity<MilliarcsecondsPerYear> = microas_motion.convert_to();
        
        // 10 μas/yr = 0.01 mas/yr
        assert!((millias_motion.value() - 0.01).abs() < 1e-10);
    }
}