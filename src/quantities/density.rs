#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units};

define_quantity!(
    Density,
    L = -3,
    M = 1,
    T = 0,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Mass/Length³

// Define Density units (Mass/Length³)
define_units! {
    dimension :{L = -3, M = 1, T = 0, THETA = 0, I = 0, J = 0, N = 0},
    base_unit: KilogramPerCubicMeter = 1.0,
    units: {
        GramPerCubicCentimeter = 1000.0,
    }
}
