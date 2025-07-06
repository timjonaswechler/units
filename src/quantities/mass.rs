#![allow(non_snake_case)]
//! Mass units

use crate::*;

// Re-export the type alias from core
pub use crate::core::Mass;

// Define base mass units
define_units_for_dimension! {
    Mass => {
        Kilogram = "kg", 1.0,          // SI base unit
        Gram = "g", 0.001,
        Tonne = "t", 1000.0,

        // Imperial units
        Pound = "lb", 0.453_592_37,
        Ounce = "oz", 0.028_349_523_125,
        Stone = "st", 6.350_293_18,

        // Astronomical masses
        SolarMass = "M☉", 1.988_47e30,
        EarthMass = "M⊕", 5.972_16e24,
        JupiterMass = "M♃", 1.898_13e27,
        LunarMass = "M☽", 7.342e22,

        // Atomic masses
        AtomicMassUnit = "u", 1.660_538_921e-27,
        ElectronMass = "mₑ", 9.109_383_56e-31,
        ProtonMass = "mₚ", 1.672_621_898e-27,
        NeutronMass = "mₙ", 1.674_927_471e-27,

        // Planck mass
        PlanckMass = "mₚ", 2.176_470e-8,
    }
}

// Generate prefixed aliases
define_prefixed_aliases! {
    Gram => [Kilo, Milli, Micro, Nano],
    Kilogram => [Mega, Giga],
}

// Convenience constructors
impl_quantity_constructors!(
    Mass,
    Kilogram,
    Gram,
    Tonne,
    Pound,
    Ounce,
    Stone,
    SolarMass,
    EarthMass,
    JupiterMass,
    LunarMass,
    AtomicMassUnit,
    ElectronMass,
    ProtonMass,
    NeutronMass,
    PlanckMass
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_units() {
        let kg = Mass::<Kilogram>::new(1.0);
        let g = Mass::<Gram>::new(1000.0);
        let t = Mass::<Tonne>::new(0.001);

        // Test conversions
        let kg_from_g: Mass<Kilogram> = g.convert_to();
        assert_eq!(kg_from_g.value(), 1.0);

        let kg_from_t: Mass<Kilogram> = t.convert_to();
        assert_eq!(kg_from_t.value(), 1.0);
    }

    #[test]
    fn test_astronomical_masses() {
        let solar = Mass::<SolarMass>::new(1.0);
        let earth = Mass::<EarthMass>::new(1.0);

        let solar_in_kg: Mass<Kilogram> = solar.convert_to();
        let earth_in_kg: Mass<Kilogram> = earth.convert_to();

        // Sun should be much more massive than Earth
        assert!(solar_in_kg.value() > earth_in_kg.value() * 100000.0);
    }

    #[test]
    fn test_imperial_units() {
        let lb = Mass::<Pound>::new(1.0);
        let kg: Mass<Kilogram> = lb.convert_to();

        // 1 pound ≈ 0.453 kg
        assert!((kg.value() - 0.453_592_37).abs() < 1e-6);
    }

    #[test]
    fn test_atomic_masses() {
        let amu = Mass::<AtomicMassUnit>::new(1.0);
        let kg: Mass<Kilogram> = amu.convert_to();

        assert!((kg.value() - 1.660_538_921e-27).abs() < 1e-35);
    }

    #[test]
    fn test_convenience_constructors() {
        let m1 = Mass::<Kilogram>::Kilogram(2.0);
        let m2 = Mass::<Gram>::Gram(2000.0);

        assert_eq!(m1.value(), 2.0);

        let m1_as_g: Mass<Gram> = m1.convert_to();
        assert!((m1_as_g.value() - m2.value()).abs() < 1e-10);
    }
}
