#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units};

define_quantity!(
    Acceleration,
    L = 1,
    M = 0,
    T = -2,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Length/Time²

// Define Acceleration units (Length/Time²)
define_units! {
    dimension: { L = 1, M = 0, T = -2, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: MeterPerSecondSquared = 1.0,
    units: {
        StandardGravity = 9.80665,
    }
}
