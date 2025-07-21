#![allow(non_snake_case)]
use crate::{define_quantity, define_units};

define_quantity!(Frequency); // 1/Time

// Define Frequency units (1/Time)
define_units! {
    base_unit: Hertz = 1.0,
    units: {}
}
