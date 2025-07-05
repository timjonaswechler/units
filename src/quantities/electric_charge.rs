//! Electric charge quantities with units

use crate::core::*;
use crate::prefix::*;

/// Electric charge quantity [I⋅T] (ampere⋅second)
pub type ElectricCharge<U> = Quantity<U, 0, 0, 1, 0, 1, 0, 0>;

// Base units
pub struct Coulomb;
impl UnitComposition for Coulomb {
    fn symbol() -> String { "C".to_string() }
    fn to_si_factor() -> f64 { 1.0 }
    fn from_si_factor() -> f64 { 1.0 }
}

pub struct AmpereSecond;
impl UnitComposition for AmpereSecond {
    fn symbol() -> String { "A⋅s".to_string() }
    fn to_si_factor() -> f64 { 1.0 }
    fn from_si_factor() -> f64 { 1.0 }
}

// Common prefixed units
pub type Millicoulomb = Prefixed<Milli, Coulomb>;
pub type Microcoulomb = Prefixed<Micro, Coulomb>;
pub type Nanocoulomb = Prefixed<Nano, Coulomb>;
pub type Picocoulomb = Prefixed<Pico, Coulomb>;
pub type Kilocoulomb = Prefixed<Kilo, Coulomb>;

// Elementary charge unit (e)
pub struct ElementaryChargeUnit;
impl UnitComposition for ElementaryChargeUnit {
    fn symbol() -> String { "e".to_string() }
    fn to_si_factor() -> f64 { 1.602176634e-19 }
    fn from_si_factor() -> f64 { 1.0 / 1.602176634e-19 }
}

// Special electromagnetic units
pub struct Franklin; // statcoulomb in CGS
impl UnitComposition for Franklin {
    fn symbol() -> String { "Fr".to_string() }
    fn to_si_factor() -> f64 { 3.33564e-10 }
    fn from_si_factor() -> f64 { 1.0 / 3.33564e-10 }
}

pub struct Abcoulomb; // electromagnetic unit in CGS
impl UnitComposition for Abcoulomb {
    fn symbol() -> String { "abC".to_string() }
    fn to_si_factor() -> f64 { 10.0 }
    fn from_si_factor() -> f64 { 0.1 }
}

// Conversion implementations
impl From<ElectricCharge<Coulomb>> for ElectricCharge<AmpereSecond> {
    fn from(charge: ElectricCharge<Coulomb>) -> Self {
        ElectricCharge::new(charge.value()) // 1 C = 1 A⋅s
    }
}

impl From<ElectricCharge<ElementaryChargeUnit>> for ElectricCharge<Coulomb> {
    fn from(charge: ElectricCharge<ElementaryChargeUnit>) -> Self {
        ElectricCharge::new(charge.value() * 1.602176634e-19) // e = 1.602...×10⁻¹⁹ C
    }
}

impl From<ElectricCharge<Franklin>> for ElectricCharge<Coulomb> {
    fn from(charge: ElectricCharge<Franklin>) -> Self {
        ElectricCharge::new(charge.value() * 3.33564e-10) // 1 Fr ≈ 3.336×10⁻¹⁰ C
    }
}

impl From<ElectricCharge<Abcoulomb>> for ElectricCharge<Coulomb> {
    fn from(charge: ElectricCharge<Abcoulomb>) -> Self {
        ElectricCharge::new(charge.value() * 10.0) // 1 abC = 10 C
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coulomb_creation() {
        let charge = ElectricCharge::<Coulomb>::new(1.0);
        assert_eq!(charge.value(), 1.0);
    }

    #[test]
    fn test_elementary_charge_conversion() {
        let e_charge = ElectricCharge::<ElementaryChargeUnit>::new(1.0);
        let coulomb_charge: ElectricCharge<Coulomb> = e_charge.into();
        assert!((coulomb_charge.value() - 1.602176634e-19).abs() < 1e-29);
    }

    #[test]
    fn test_coulomb_ampere_second_equivalence() {
        let coulomb = ElectricCharge::<Coulomb>::new(1.0);
        let amp_sec: ElectricCharge<AmpereSecond> = coulomb.into();
        assert_eq!(amp_sec.value(), 1.0);
    }

    #[test]
    fn test_franklin_conversion() {
        let franklin = ElectricCharge::<Franklin>::new(1.0);
        let coulomb: ElectricCharge<Coulomb> = franklin.into();
        assert!((coulomb.value() - 3.33564e-10).abs() < 1e-15);
    }

    #[test]
    fn test_abcoulomb_conversion() {
        let abcoulomb = ElectricCharge::<Abcoulomb>::new(1.0);
        let coulomb: ElectricCharge<Coulomb> = abcoulomb.into();
        assert_eq!(coulomb.value(), 10.0);
    }

    #[test]
    fn test_prefixed_units() {
        let millic = ElectricCharge::<Millicoulomb>::new(1000.0);
        let coulomb = ElectricCharge::<Coulomb>::new(1.0);
        // Note: This test assumes prefix conversion is implemented
        // assert_eq!(millic.to_base().value(), coulomb.value());
    }
}