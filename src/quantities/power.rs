#![allow(non_snake_case)]

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Conversion constants
const WATTS_PER_SOLAR_LUMINOSITY: DefaultFloat = 3.828e26;

define_quantity!(Power); // Mass×Length²/Time³

define_units! {
    base_unit: Watt = 1.0,
    units: {
        SolarLuminosity = WATTS_PER_SOLAR_LUMINOSITY,
    }
}
