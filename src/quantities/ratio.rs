#![allow(non_snake_case)]

use crate::{define_quantity, define_units};

define_quantity!(Ratio); // Dimensionless

define_units! {
    base_unit: Unit = 1.0,
    units: {
        Fraction = 1.0, // Alias for Unit
        Percent = 0.01,
        PartsPerMillion = 1e-6,
        PartsPerBillion = 1e-9,
    }
}
