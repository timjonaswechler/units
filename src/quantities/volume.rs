#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

define_quantity!(Volume, L = 3, M = 0, T = 0, THETA = 0, I = 0, J = 0, N = 0); // Length³

define_units! {
    dimension: { L = 3, M = 0, T = 0, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: CubicMeter = 1.0,
    units: {
        Liter = 0.001,
    }
}
