#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

define_quantity!(Force, L = 1, M = 1, T = -2, THETA = 0, I = 0, J = 0, N = 0); // Mass×Length/Time²

// Define Force units (Mass×Length/Time²)
define_units! {
    dimension:{ L = 1, M = 1, T = -2, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: Newton = 1.0,
    units: {}
}
