#![allow(non_snake_case)]

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Conversion constants
const WATTS_PER_SOLAR_LUMINOSITY: DefaultFloat = 3.828e26;

define_quantity!(
    Luminosity,
    L = 2,
    M = 1,
    T = -3,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // ML²T⁻³

define_units! {
    dimension: { L = 2, M = 1, T = -3, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: Watt = 1.0,
    units: {
        SolarLuminosity = WATTS_PER_SOLAR_LUMINOSITY,
        ErgPerSecond = 1e-7, // 1 erg/s = 1e-7 W
    }
}
