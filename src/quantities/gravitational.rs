#![allow(non_snake_case)]

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Conversion constants
const SOLAR_GRAVITATIONAL_PARAMETER: DefaultFloat = 1.32712440042e20;
const EARTH_GRAVITATIONAL_PARAMETER: DefaultFloat = 3.986004418e14;

// Define gravitational parameter quantity (Length³/Time²)
define_quantity!(
    GravitationalParameter,
    L = 3,
    M = 0,
    T = -2,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Length³/Time²

define_units! {
    dimension: { L = 3, M = 0, T = -2, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: CubicMeterPerSecondSquared = 1.0,
    units: {
        SolarGravitationalParameter = SOLAR_GRAVITATIONAL_PARAMETER,
        EarthGravitationalParameter = EARTH_GRAVITATIONAL_PARAMETER,
    }
}
