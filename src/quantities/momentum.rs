#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

// Additional derived quantities
define_quantity!(Momentum); // Mass×Length/Time

define_units! {
    base_unit: KilogramMeterPerSecond = 1.0,
    units: {
    }
}
