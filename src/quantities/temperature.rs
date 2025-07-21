#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units};

define_quantity!(Temperature); // Temperature

// Define Temperature units
define_units! {
    base_unit: Kelvin = 1.0,
    units: {

    }
}
