#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

define_quantity!(Volume); // Length³

define_units! {
    base_unit: CubicMeter = 1.0,
    units: {
        Liter = 0.001,
    }
}
