#![allow(non_snake_case)]
use crate::{define_quantity, define_units};

define_quantity!(Area); // Length²

// Define Area units (Length²)
define_units! {
    base_unit: SquareMeter = 1.0,
    units: {
        SquareKilometer = 1_000_000.0,
    }
}
