#![allow(non_snake_case)]

use crate::core::*;
use crate::{define_quantity, define_units};

define_quantity!(
    Frequency,
    L = 0,
    M = 0,
    T = -1,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // 1/Time

// Define Frequency units (1/Time)
define_units! {
    dimension: { L = 0, M = 0, T = -1, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: Hertz = 1.0,
    units: {}
}
