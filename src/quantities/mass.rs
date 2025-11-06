use crate::dimension::Dimension;
use crate::quantity::Quantity;
use crate::unit::Unit;

/// Physical quantity: Mass
///
/// SI Base Unit: Kilogram (kg)
#[derive(Debug, Clone, Copy)]
pub struct Mass;

impl Quantity for Mass {
    const DIMENSION: Dimension = Dimension::mass();
    const NAME: &'static str = "Mass";
}

impl crate::quantity::CanAddSameQuantity for Mass {}

// ============================================================================
// SI Base Unit
// ============================================================================

/// Kilogram - SI base unit for mass
#[derive(Debug, Clone, Copy)]
pub struct Kilogram;

impl Unit for Kilogram {
    type BaseQuantity = Mass;
    const SYMBOL: &'static str = "kg";
    const TO_SI: f64 = 1.0;
    const OFFSET: f64 = 0.0;
}

// ============================================================================
// Metric Units
// ============================================================================

/// Gram (1 g = 0.001 kg)
#[derive(Debug, Clone, Copy)]
pub struct Gram;

impl Unit for Gram {
    type BaseQuantity = Mass;
    const SYMBOL: &'static str = "g";
    const TO_SI: f64 = 0.001;
    const OFFSET: f64 = 0.0;
}

/// Milligram (1 mg = 1e-6 kg)
#[derive(Debug, Clone, Copy)]
pub struct Milligram;

impl Unit for Milligram {
    type BaseQuantity = Mass;
    const SYMBOL: &'static str = "mg";
    const TO_SI: f64 = 1e-6;
    const OFFSET: f64 = 0.0;
}

/// Microgram (1 μg = 1e-9 kg)
#[derive(Debug, Clone, Copy)]
pub struct Microgram;

impl Unit for Microgram {
    type BaseQuantity = Mass;
    const SYMBOL: &'static str = "μg";
    const TO_SI: f64 = 1e-9;
    const OFFSET: f64 = 0.0;
}

/// Tonne / Metric Ton (1 t = 1000 kg)
#[derive(Debug, Clone, Copy)]
pub struct Tonne;

impl Unit for Tonne {
    type BaseQuantity = Mass;
    const SYMBOL: &'static str = "t";
    const TO_SI: f64 = 1000.0;
    const OFFSET: f64 = 0.0;
}

// ============================================================================
// Imperial/US Units
// ============================================================================

/// Pound (1 lb ≈ 0.453592 kg)
#[derive(Debug, Clone, Copy)]
pub struct Pound;

impl Unit for Pound {
    type BaseQuantity = Mass;
    const SYMBOL: &'static str = "lb";
    const TO_SI: f64 = 0.45359237;
    const OFFSET: f64 = 0.0;
}

/// Ounce (1 oz ≈ 0.0283495 kg)
#[derive(Debug, Clone, Copy)]
pub struct Ounce;

impl Unit for Ounce {
    type BaseQuantity = Mass;
    const SYMBOL: &'static str = "oz";
    const TO_SI: f64 = 0.028349523125;
    const OFFSET: f64 = 0.0;
}

/// Stone (1 st = 6.35029 kg)
#[derive(Debug, Clone, Copy)]
pub struct Stone;

impl Unit for Stone {
    type BaseQuantity = Mass;
    const SYMBOL: &'static str = "st";
    const TO_SI: f64 = 6.35029318;
    const OFFSET: f64 = 0.0;
}

// ============================================================================
// Astronomical Units
// ============================================================================

/// Solar Mass (M☉ ≈ 1.989e30 kg)
#[derive(Debug, Clone, Copy)]
pub struct SolarMass;

impl Unit for SolarMass {
    type BaseQuantity = Mass;
    const SYMBOL: &'static str = "M☉";
    const TO_SI: f64 = 1.98892e30;
    const OFFSET: f64 = 0.0;
}

/// Earth Mass (M⊕ ≈ 5.972e24 kg)
#[derive(Debug, Clone, Copy)]
pub struct EarthMass;

impl Unit for EarthMass {
    type BaseQuantity = Mass;
    const SYMBOL: &'static str = "M⊕";
    const TO_SI: f64 = 5.97219e24;
    const OFFSET: f64 = 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_mass_dimension() {
        assert_eq!(Mass::dimension(), Dimension::mass());
    }

    #[test]
    fn test_kilogram_conversion() {
        let kg = Value::<Mass, Kilogram>::new(1.0);
        assert_eq!(kg.get_si(), 1.0);
    }

    #[test]
    fn test_gram_conversion() {
        let g = Value::<Mass, Gram>::new(1000.0);
        assert_eq!(g.get_si(), 1.0);

        let kg = g.convert::<Kilogram>();
        assert_eq!(kg.get(), 1.0);
    }

    #[test]
    fn test_pound_conversion() {
        let lb = Value::<Mass, Pound>::new(1.0);
        let kg = lb.convert::<Kilogram>();
        assert!((kg.get() - 0.45359237).abs() < 1e-10);
    }

    #[test]
    fn test_mass_addition() {
        let kg1 = Value::<Mass, Kilogram>::new(1.0);
        let g500 = Value::<Mass, Gram>::new(500.0);
        let result = kg1 + g500;
        assert_eq!(result.get(), 1.5);
    }
}
