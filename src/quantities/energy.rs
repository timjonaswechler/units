#![allow(non_snake_case)]

use crate::{define_quantity, define_units, features::DefaultFloat};

const JOULES_PER_ERG: DefaultFloat = 1e-7; // 1 erg = 10⁻⁷ Joules
const JOULES_PER_EV: DefaultFloat = 1.602176634e-19; // 1 eV = 1.602176634 × 10⁻¹⁹ Joules

define_quantity!(Energy); // Mass×Length²/Time²

define_units! {
    base_unit: Joule = 1.0,
    units: {
        Erg = JOULES_PER_ERG,
        ElectronVolt = JOULES_PER_EV,
    }
}
