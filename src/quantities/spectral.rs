#![allow(non_snake_case)]

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Physical constants
const SPEED_OF_LIGHT: DefaultFloat = 299792458.0; // m/s
const PLANCK_CONSTANT: DefaultFloat = 6.62607015e-34; // J⋅s

// Wavelength is just Distance, but we define it separately for clarity
define_quantity!(Wavelength); // Length

define_units! {
    base_unit: WavelengthMeter = 1.0,
    units: {
        Angstrom = 1e-10,
    }
}

// Frequency already defined in dimensions.rs, but we can add spectral-specific units
// We'll extend the existing Frequency dimension

// Wavenumber (1/Length) - common in spectroscopy
define_quantity!(Wavenumber); // 1/Length

define_units! {
    base_unit: PerMeter = 1.0,
    units: {
        PerAngstrom = 1e10,
    }
}
