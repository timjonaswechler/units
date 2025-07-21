#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

// Magnetic flux (Weber) - Mass×Length²/(Current×Time²)
define_quantity!(MagneticFlux); // Mass×Length²/(Current×Time²)

define_units! {
    base_unit: Weber = 1.0,
    units: {
        Maxwell = 1e-8, // CGS unit
    }
}
