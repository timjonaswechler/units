#![allow(non_snake_case)]

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Conversion constants
const TESLA_PER_GAUSS: DefaultFloat = 1e-4;

// Define magnetic field quantity (Mass/(Current×Time²))
define_quantity!(
    MagneticField,
    L = 0,
    M = 1,
    T = -2,
    THETA = 0,
    I = -1,
    J = 0,
    N = 0
); // Mass/(Current×Time²)

define_units! {
    dimension: { L = 0, M = 1, T = -2, THETA = 0, I = -1, J = 0, N = 0 },
    base_unit: Tesla = 1.0,
    units: {
        Gauss = TESLA_PER_GAUSS,
    }
}

// Magnetic flux density is the same as magnetic field, but sometimes distinguished
pub type MagneticFluxDensity<U> = MagneticField<U>;

// Convenience type aliases
pub type TeslaField = MagneticField<Tesla>;
pub type GaussField = MagneticField<Gauss>;
