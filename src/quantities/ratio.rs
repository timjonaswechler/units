#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

define_quantity!(Ratio, L = 0, M = 0, T = 0, THETA = 0, I = 0, J = 0, N = 0); // Dimensionless

define_units! {
    dimension: { L = 0, M = 0, T = 0, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: Unit = 1.0,
    units: {
        Fraction = 1.0, // Alias for Unit
        Percent = 0.01,
        PartsPerMillion = 1e-6,
        PartsPerBillion = 1e-9,
    }
}
