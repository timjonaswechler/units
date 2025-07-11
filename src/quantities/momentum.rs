#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

// Additional derived quantities
define_quantity!(
    Momentum,
    L = 1,
    M = 1,
    T = -1,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Mass×Length/Time

define_units! {
    dimension: { L = 1, M = 1, T = -1, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: KilogramMeterPerSecond = 1.0,
    units: {
    }
}
