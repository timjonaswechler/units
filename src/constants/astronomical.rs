//! Astronomical constants
//!
//! Standard astronomical constants with type-safe units

use crate::prelude::*;

/// Astronomical Unit: 1 AU = 149,597,870.7 km (exact)
pub const ASTRONOMICAL_UNIT: Distance<Meter> = Distance::new(149_597_870_700.0);

/// Parsec: 1 pc = 648,000/π AU ≈ 3.0857×10¹⁶ m
pub const PARSEC: Distance<Meter> = Distance::new(3.0857e16);

/// Light year: 1 ly = c × 1 tropical year ≈ 9.4607×10¹⁵ m
pub const LIGHT_YEAR: Distance<Meter> = Distance::new(9.4607304725808e15);

/// Solar mass: M☉ = 1.98847×10³⁰ kg
pub const SOLAR_MASS: Mass<Kilogram> = Mass::new(1.98847e30);

/// Solar radius: R☉ = 6.957×10⁸ m
pub const SOLAR_RADIUS: Distance<Meter> = Distance::new(6.957e8);

/// Solar luminosity: L☉ = 3.828×10²⁶ W
pub const SOLAR_LUMINOSITY: Power<Watt> = Power::new(3.828e26);

/// Solar effective temperature: T☉ = 5778 K
pub const SOLAR_TEMPERATURE: f64 = 5778.0;

/// Earth mass: M⊕ = 5.9722×10²⁴ kg
pub const EARTH_MASS: Mass<Kilogram> = Mass::new(5.9722e24);

/// Earth equatorial radius: R⊕ = 6.378137×10⁶ m
pub const EARTH_RADIUS: Distance<Meter> = Distance::new(6.378137e6);

/// Earth-Moon distance (semi-major axis): 3.844×10⁸ m
pub const EARTH_MOON_DISTANCE: Distance<Meter> = Distance::new(3.844e8);

/// Lunar mass: M☽ = 7.342×10²² kg
pub const LUNAR_MASS: Mass<Kilogram> = Mass::new(7.342e22);

/// Lunar radius: R☽ = 1.737×10⁶ m
pub const LUNAR_RADIUS: Distance<Meter> = Distance::new(1.737e6);

/// Jupiter mass: M♃ = 1.8982×10²⁷ kg
pub const JUPITER_MASS: Mass<Kilogram> = Mass::new(1.8982e27);

/// Jupiter equatorial radius: R♃ = 7.1492×10⁷ m
pub const JUPITER_RADIUS: Distance<Meter> = Distance::new(7.1492e7);

/// Standard gravity at Earth's surface: g = 9.80665 m/s² (exact)
pub const STANDARD_GRAVITY: Acceleration<MeterPerSecondSquared> = Acceleration::new(9.80665);

/// Tropical year: 1 year = 365.24219 days = 31,556,925.216 s
pub const TROPICAL_YEAR: Time<Second> = Time::new(31_556_925.216);

/// Sidereal year: 1 sidereal year = 365.25636 days = 31,558,149.504 s
pub const SIDEREAL_YEAR: Time<Second> = Time::new(31_558_149.504);

/// Julian year: 1 Julian year = 365.25 days = 31,557,600 s (exact)
pub const JULIAN_YEAR: Time<Second> = Time::new(31_557_600.0);

/// Hubble constant: H₀ ≈ 70 km/(s⋅Mpc) = 2.27×10⁻¹⁸ s⁻¹
pub const HUBBLE_CONSTANT: f64 = 2.27e-18;

/// Age of the universe: t₀ ≈ 13.8×10⁹ years
pub const AGE_OF_UNIVERSE: Time<Second> = Time::new(4.35e17);

/// Critical density of the universe: ρc = 3H₀²/(8πG) ≈ 9.47×10⁻²⁷ kg/m³
pub const CRITICAL_DENSITY: Density<KilogramPerCubicMeter> = Density::new(9.47e-27);

/// Cosmic microwave background temperature: T_CMB = 2.72548 K
pub const CMB_TEMPERATURE: f64 = 2.72548;

/// Galaxy mass (Milky Way): M_MW ≈ 1.5×10¹² M☉
pub const MILKY_WAY_MASS: Mass<SolarMass> = Mass::new(1.5e12);

/// Galaxy diameter (Milky Way): D_MW ≈ 100,000 ly
pub const MILKY_WAY_DIAMETER: Distance<LightYear> = Distance::new(100_000.0);

/// Solar system age: t_☉ ≈ 4.6×10⁹ years
pub const SOLAR_SYSTEM_AGE: Time<Second> = Time::new(1.45e17);

/// Earth's orbital velocity: v_⊕ ≈ 29.78 km/s
pub const EARTH_ORBITAL_VELOCITY: Velocity<MeterPerSecond> = Velocity::new(29_780.0);

/// Escape velocity from Earth: v_esc = √(2GM/R) ≈ 11.18 km/s
pub const EARTH_ESCAPE_VELOCITY: Velocity<MeterPerSecond> = Velocity::new(11_180.0);

/// Solar wind velocity: v_sw ≈ 400 km/s
pub const SOLAR_WIND_VELOCITY: Velocity<MeterPerSecond> = Velocity::new(400_000.0);

/// Galactic rotation velocity (at solar radius): v_gal ≈ 220 km/s
pub const GALACTIC_ROTATION_VELOCITY: Velocity<MeterPerSecond> = Velocity::new(220_000.0);

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_astronomical_unit() {
        assert_eq!(ASTRONOMICAL_UNIT.value(), 149_597_870_700.0);
    }

    #[test]
    fn test_parsec_definition() {
        // 1 pc = 648,000/π AU
        let expected = 648_000.0 / PI * ASTRONOMICAL_UNIT.value();
        assert!((PARSEC.value() - expected).abs() / expected < 1e-6);
    }

    #[test]
    fn test_light_year_definition() {
        // 1 ly = c × 1 tropical year
        let c = 299_792_458.0; // m/s
        let year = TROPICAL_YEAR.value(); // s
        let expected = c * year;
        assert!((LIGHT_YEAR.value() - expected).abs() / expected < 1e-10);
    }

    #[test]
    fn test_standard_gravity() {
        assert_eq!(STANDARD_GRAVITY.value(), 9.80665);
    }

    #[test]
    fn test_tropical_vs_julian_year() {
        // Tropical year should be slightly shorter than Julian year
        assert!(TROPICAL_YEAR.value() < JULIAN_YEAR.value());
        let diff = JULIAN_YEAR.value() - TROPICAL_YEAR.value();
        assert!(diff > 600.0 && diff < 700.0); // Should be around 674.784 seconds
    }

    #[test]
    fn test_earth_escape_velocity() {
        // v_esc = √(2GM/R)
        use crate::constants::fundamental::GRAVITATIONAL_CONSTANT;
        let g = GRAVITATIONAL_CONSTANT;
        let m = EARTH_MASS.value();
        let r = EARTH_RADIUS.value();
        
        let calculated = (2.0 * g * m / r).sqrt();
        assert!((EARTH_ESCAPE_VELOCITY.value() - calculated).abs() / calculated < 0.01);
    }

    #[test]
    fn test_hubble_time() {
        // Hubble time ≈ 1/H₀ ≈ 14 billion years
        let hubble_time = 1.0 / HUBBLE_CONSTANT;
        let billion_years = 1e9 * JULIAN_YEAR.value();
        assert!(hubble_time > 13.0 * billion_years);
        assert!(hubble_time < 15.0 * billion_years);
    }

    #[test]
    fn test_solar_mass_scale() {
        // Solar mass should be much larger than Earth mass
        let ratio = SOLAR_MASS.value() / EARTH_MASS.value();
        assert!(ratio > 300_000.0 && ratio < 400_000.0); // Should be ~333,000
    }
}