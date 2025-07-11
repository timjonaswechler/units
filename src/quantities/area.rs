#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::core::*;
use crate::{define_quantity, define_units};

define_quantity!(Area, L = 2, M = 0, T = 0, THETA = 0, I = 0, J = 0, N = 0); // Length²

// Define Area units (Length²)
define_units! {
    dimension:{L = 2, M = 0, T = 0, THETA = 0, I = 0, J = 0, N = 0},
    base_unit: SquareMeter = 1.0,
    units: {
        SquareKilometer = 1_000_000.0,
    }
}
