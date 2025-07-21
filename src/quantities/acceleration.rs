#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units};

define_quantity!(Acceleration); // Length/Time²

// Define Acceleration units (Length/Time²)
define_units! {
    base_unit: MeterPerSecondSquared = 1.0,
    units: {
        StandardGravity = 9.80665,
    }
}
