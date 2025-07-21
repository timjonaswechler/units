#![allow(non_snake_case)]

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Conversion constants
const JOULES_PER_KG_PER_CAL_PER_G: DefaultFloat = 4184.0;

// Specific energy (Energy/Mass) - Length²/Time²
define_quantity!(SpecificEnergy); // Length²/Time²

define_units! {
    base_unit: JoulePerKilogram = 1.0,
    units: {
        CaloriePerGram = JOULES_PER_KG_PER_CAL_PER_G,
        ErgPerGram = 0.1, // 1 erg/g = 0.1 J/kg
    }
}

// Specific heat capacity (Energy/(Mass×Temperature)) - Length²/(Time²×Temperature)
define_quantity!(SpecificHeatCapacity); // Length²/(Time²×Temperature)

define_units! {
    base_unit: JoulePerKilogramKelvin = 1.0,
    units: {
        CaloriePerGramKelvin = JOULES_PER_KG_PER_CAL_PER_G,
    }
}

// Specific volume (Volume/Mass) - Length³/Mass
define_quantity!(SpecificVolume); // Length³/Mass

define_units! {
    base_unit: CubicMeterPerKilogram = 1.0,
    units: {
        CubicCentimeterPerGram = 0.001, // 1 cm³/g = 0.001 m³/kg
    }
}
