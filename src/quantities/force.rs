#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

define_quantity!(Force); // Mass×Length/Time²

// Define Force units (Mass×Length/Time²)
define_units! {
    base_unit: Newton = 1.0,
    units: {}
}
