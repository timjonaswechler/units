#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

// Magnetic flux (Weber) - Mass×Length²/(Current×Time²)
define_quantity!(
    MagneticFlux,
    L = 2,
    M = 1,
    T = -2,
    THETA = 0,
    I = -1,
    J = 0,
    N = 0
); // Mass×Length²/(Current×Time²)

define_units! {
    dimension: { L = 2, M = 1, T = -2, THETA = 0, I = -1, J = 0, N = 0 },
    base_unit: Weber = 1.0,
    units: {
        Maxwell = 1e-8, // CGS unit
    }
}
