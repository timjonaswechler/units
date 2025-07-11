#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units};

define_quantity!(
    Temperature,
    L = 0,
    M = 0,
    T = 0,
    THETA = 1,
    I = 0,
    J = 0,
    N = 0
); // Temperature

// Define Temperature units
define_units! {
    dimension: { L = 0, M = 0, T = 0, THETA = 1, I = 0, J = 0, N = 0 },
    base_unit: Kelvin = 1.0,
    units: {

    }
}
