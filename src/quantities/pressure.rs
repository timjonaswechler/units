#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

define_quantity!(Pressure); // Mass/(Length×Time²)

define_units! {
    base_unit: Pascal = 1.0,
    units: {
        Bar = 100_000.0,
    }
}
