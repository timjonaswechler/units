#![allow(non_snake_case)]

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Conversion constants
const JOULES_PER_KG_PER_CAL_PER_G: DefaultFloat = 4184.0;

// Specific energy (Energy/Mass) - Length²/Time²
define_quantity!(
    SpecificEnergy,
    L = 2,
    M = 0,
    T = -2,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Length²/Time²

define_units! {
    dimension: { L = 2, M = 0, T = -2, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: JoulePerKilogram = 1.0,
    units: {
        CaloriePerGram = JOULES_PER_KG_PER_CAL_PER_G,
        ErgPerGram = 0.1, // 1 erg/g = 0.1 J/kg
    }
}

// Specific heat capacity (Energy/(Mass×Temperature)) - Length²/(Time²×Temperature)
define_quantity!(
    SpecificHeatCapacity,
    L = 2,
    M = 0,
    T = -2,
    THETA = -1,
    I = 0,
    J = 0,
    N = 0
); // Length²/(Time²×Temperature)

define_units! {
    dimension: { L = 2, M = 0, T = -2, THETA = -1, I = 0, J = 0, N = 0 },
    base_unit: JoulePerKilogramKelvin = 1.0,
    units: {
        CaloriePerGramKelvin = JOULES_PER_KG_PER_CAL_PER_G,
    }
}

// Specific entropy (same dimensions as specific heat capacity)
pub type SpecificEntropy<U> = SpecificHeatCapacity<U>;

// Specific gas constant (Energy/(Mass×Temperature)) - same as specific heat capacity
pub type SpecificGasConstant<U> = SpecificHeatCapacity<U>;

// Specific volume (Volume/Mass) - Length³/Mass
define_quantity!(
    SpecificVolume,
    L = 3,
    M = -1,
    T = 0,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Length³/Mass

define_units! {
    dimension: { L = 3, M = -1, T = 0, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: CubicMeterPerKilogram = 1.0,
    units: {
        CubicCentimeterPerGram = 0.001, // 1 cm³/g = 0.001 m³/kg
    }
}

// Convenience type aliases
pub type SpecificEnergyJKg = SpecificEnergy<JoulePerKilogram>;
pub type SpecificHeatJKgK = SpecificHeatCapacity<JoulePerKilogramKelvin>;
pub type SpecificVolumeM3Kg = SpecificVolume<CubicMeterPerKilogram>;

// Useful constants for stellar physics
impl SpecificEnergy<JoulePerKilogram> {
    /// Nuclear binding energy per nucleon for hydrogen fusion (~7 MeV/nucleon)
    pub fn hydrogen_fusion_energy() -> Self {
        Self::new(6.3e14) // J/kg - approximate value for hydrogen to helium
    }

    /// Gravitational binding energy per unit mass at stellar surface
    /// GM/R for typical stellar parameters
    pub fn stellar_binding_energy(gm: f64, radius: f64) -> Self {
        Self::new(gm / radius)
    }
}

impl SpecificHeatCapacity<JoulePerKilogramKelvin> {
    /// Specific heat capacity of an ideal monatomic gas (3/2 * R/M)
    pub fn ideal_monatomic_gas(molar_mass_kg: f64) -> Self {
        const GAS_CONSTANT: f64 = 8.314462618; // J/(mol⋅K)
        Self::new(1.5 * GAS_CONSTANT / molar_mass_kg)
    }

    /// Specific heat capacity of an ideal diatomic gas (5/2 * R/M)
    pub fn ideal_diatomic_gas(molar_mass_kg: f64) -> Self {
        const GAS_CONSTANT: f64 = 8.314462618; // J/(mol⋅K)
        Self::new(2.5 * GAS_CONSTANT / molar_mass_kg)
    }
}
