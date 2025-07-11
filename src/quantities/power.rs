#![allow(non_snake_case)]

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Conversion constants
const WATTS_PER_SOLAR_LUMINOSITY: DefaultFloat = 3.828e26;

define_quantity!(Power, L = 2, M = 1, T = -3, THETA = 0, I = 0, J = 0, N = 0); // Mass×Length²/Time³

define_units! {
    dimension: { L = 2, M = 1, T = -3, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: Watt = 1.0,
    units: {
        SolarLuminosity = WATTS_PER_SOLAR_LUMINOSITY,
    }
}
