#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units};

define_quantity!(Density); // Mass/Length³

// Define Density units (Mass/Length³)
define_units! {
    base_unit: KilogramPerCubicMeter = 1.0,
    units: {
        GramPerCubicCentimeter = 1000.0,
    }
}
