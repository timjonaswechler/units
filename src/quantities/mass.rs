#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::composition::Prefixed;
use crate::features::DefaultFloat;
use crate::prefix::Kilo;
use crate::{define_quantity, define_units};

// Conversion constants to kg
const KG_PER_GRAM: DefaultFloat = 0.001;
const KG_PER_EARTH_MASS: DefaultFloat = 5.972e24;
const KG_PER_SOLAR_MASS: DefaultFloat = 1.989e30;

define_quantity!(Mass, L = 0, M = 1, T = 0, THETA = 0, I = 0, J = 0, N = 0);

// Define Mass units with astronomical focus
// Note: Using Gram as base unit to avoid confusion with prefix system
// Kilogram will be available as Prefixed<Kilo, Gram>
define_units! {
    dimension: { L = 0, M = 1, T = 0, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: Gram = KG_PER_GRAM,
    units: {
        EarthMass = KG_PER_EARTH_MASS,
        SolarMass = KG_PER_SOLAR_MASS,
    }
}

// ================================================================================================
// CONVENIENCE TYPE ALIASES FOR COMMON PREFIXED UNITS
// ================================================================================================

// Mass prefixes (Gram is now the base unit, so Kilogram is a proper prefix)
pub type Kilogram = Prefixed<Kilo, Gram>;
