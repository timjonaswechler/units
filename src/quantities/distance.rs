#![allow(non_snake_case)]

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Conversion constants
const METERS_PER_AU: DefaultFloat = 1.495978707e11;
const METERS_PER_EARTH_RADIUS: DefaultFloat = 6.3781e6;
const METERS_PER_SUN_RADIUS: DefaultFloat = 6.96e8;
const METERS_PER_LIGHT_YEAR: DefaultFloat = 9.4607304725808e15;
const METERS_PER_PARSEC: DefaultFloat = 3.0856775814913673e16;

define_quantity!(
    Distance,
    L = 1,
    M = 0,
    T = 0,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Length

define_units! {
    dimension: { L = 1, M = 0, T = 0, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: Meter = 1.0,
    units: {
        AstronomicalUnit = METERS_PER_AU,
        EarthRadius = METERS_PER_EARTH_RADIUS,
        SunRadius = METERS_PER_SUN_RADIUS,
        LightYear = METERS_PER_LIGHT_YEAR,
        Parsec = METERS_PER_PARSEC,
    }
}

// ================================================================================================
// CONVENIENCE TYPE ALIASES FOR COMMON PREFIXED UNITS
// ================================================================================================
