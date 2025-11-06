//! Astronomical quantities and units
//!
//! This module provides quantities and units commonly used in astronomy,
//! ported from the original implementation using the new macro system.

use crate::{define_quantity, define_units, define_quantity_with_units};
use crate::dimension::Dimension;
use crate::prefix::Kilo;
use crate::prefix::Prefixed;

// ============================================================================
// Distance (Astronomical)
// ============================================================================

// Conversion constants
const METERS_PER_AU: f64 = 1.495978707e11;
const METERS_PER_EARTH_RADIUS: f64 = 6.3781e6;
const METERS_PER_SUN_RADIUS: f64 = 6.96e8;
const METERS_PER_LIGHT_YEAR: f64 = 9.4607304725808e15;
const METERS_PER_PARSEC: f64 = 3.0856775814913673e16;

define_quantity!(Distance, Dimension::length());

define_units! {
    quantity: Distance,
    base_unit: Meter = 1.0,
    units: {
        AstronomicalUnit = METERS_PER_AU,
        EarthRadius = METERS_PER_EARTH_RADIUS,
        SunRadius = METERS_PER_SUN_RADIUS,
        LightYear = METERS_PER_LIGHT_YEAR,
        Parsec = METERS_PER_PARSEC,
    }
}

pub type Kilometer = Prefixed<Kilo, Meter>;

// ============================================================================
// Mass (Astronomical)
// ============================================================================

const KG_PER_EARTH_MASS: f64 = 5.972e24;
const KG_PER_SOLAR_MASS: f64 = 1.989e30;

define_quantity!(AstroMass, Dimension::mass());

define_units! {
    quantity: AstroMass,
    base_unit: Kilogram = 1.0,
    units: {
        EarthMass = KG_PER_EARTH_MASS,
        SolarMass = KG_PER_SOLAR_MASS,
    }
}

// ============================================================================
// Power (Luminosity)
// ============================================================================

const WATTS_PER_SOLAR_LUMINOSITY: f64 = 3.828e26;

define_quantity_with_units! {
    quantity: Luminosity,
    dimension: Dimension::POWER,
    base_unit: Watt = 1.0,
    units: {
        SolarLuminosity = WATTS_PER_SOLAR_LUMINOSITY,
    }
}

// ============================================================================
// Angle
// ============================================================================

const RADIANS_PER_DEGREE: f64 = core::f64::consts::PI / 180.0;

define_quantity_with_units! {
    quantity: Angle,
    dimension: Dimension::DIMENSIONLESS,
    base_unit: Radian = 1.0,
    units: {
        Degree = RADIANS_PER_DEGREE,
        Arcminute = RADIANS_PER_DEGREE / 60.0,
        Arcsecond = RADIANS_PER_DEGREE / 3600.0,
        Milliarcsecond = RADIANS_PER_DEGREE / 3_600_000.0,
    }
}

// ============================================================================
// Velocity
// ============================================================================

const SPEED_OF_LIGHT: f64 = 299_792_458.0; // m/s

define_quantity_with_units! {
    quantity: Velocity,
    dimension: Dimension::VELOCITY,
    base_unit: MeterPerSecond = 1.0,
    units: {
        KilometerPerSecond = 1000.0,
        SpeedOfLight = SPEED_OF_LIGHT,
    }
}

// ============================================================================
// Acceleration
// ============================================================================

const STANDARD_GRAVITY: f64 = 9.80665;

define_quantity_with_units! {
    quantity: Acceleration,
    dimension: Dimension::ACCELERATION,
    base_unit: MeterPerSecondSquared = 1.0,
    units: {
        StandardGravity = STANDARD_GRAVITY,
    }
}

// ============================================================================
// Area
// ============================================================================

define_quantity_with_units! {
    quantity: Area,
    dimension: Dimension::AREA,
    base_unit: SquareMeter = 1.0,
    units: {
        SquareKilometer = 1_000_000.0,
    }
}

// ============================================================================
// Pressure
// ============================================================================

define_quantity_with_units! {
    quantity: Pressure,
    dimension: Dimension::PRESSURE,
    base_unit: Pascal = 1.0,
    units: {
        Bar = 100_000.0,
        Atmosphere = 101_325.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_astronomical_unit() {
        let au = Value::<Distance, AstronomicalUnit>::new(1.0);
        let m = au.convert::<Meter>();
        assert!((m.get() - METERS_PER_AU).abs() < 1e6);
    }

    #[test]
    fn test_solar_mass() {
        let sm = Value::<AstroMass, SolarMass>::new(1.0);
        let kg = sm.convert::<Kilogram>();
        assert!((kg.get() - KG_PER_SOLAR_MASS).abs() < 1e20);
    }

    #[test]
    fn test_angle_conversion() {
        let deg = Value::<Angle, Degree>::new(180.0);
        let rad = deg.convert::<Radian>();
        assert!((rad.get() - core::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_velocity() {
        let c = Value::<Velocity, SpeedOfLight>::new(1.0);
        let mps = c.convert::<MeterPerSecond>();
        assert_eq!(mps.get(), SPEED_OF_LIGHT);
    }

    #[test]
    fn test_pressure() {
        let bar = Value::<Pressure, Bar>::new(1.0);
        let pa = bar.convert::<Pascal>();
        assert_eq!(pa.get(), 100_000.0);
    }
}
