#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units};

define_quantity!(Current); // Current

// Define Current units
define_units! {
    base_unit: Ampere = 1.0,
    units: {

    }
}
