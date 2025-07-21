#![allow(non_snake_case)]

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Conversion constants
const TESLA_PER_GAUSS: DefaultFloat = 1e-4;

// Define magnetic field quantity (Mass/(Current×Time²))
define_quantity!(MagneticField); // Mass/(Current×Time²)

define_units! {
    base_unit: Tesla = 1.0,
    units: {
        Gauss = TESLA_PER_GAUSS,
    }
}
