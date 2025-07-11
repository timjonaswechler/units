#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units};

define_quantity!(Current, L = 0, M = 0, T = 0, THETA = 0, I = 1, J = 0, N = 0); // Current

// Define Current units
define_units! {
    dimension:{ L = 0, M = 0, T = 0, THETA = 0, I = 1, J = 0, N = 0},
    base_unit: Ampere = 1.0,
    units: {

    }
}
