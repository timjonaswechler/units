#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

define_quantity!(
    Pressure,
    L = -1,
    M = 1,
    T = -2,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Mass/(Length×Time²)

define_units! {
    dimension: { L = -1, M = 1, T = -2, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: Pascal = 1.0,
    units: {
        Bar = 100_000.0,
    }
}
