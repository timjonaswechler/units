use crate::dimension::Dimension;
use crate::quantity::Quantity;
use crate::unit::Unit;

/// Physical quantity: Length
///
/// SI Base Unit: Meter (m)
#[derive(Debug, Clone, Copy)]
pub struct Length;

impl Quantity for Length {
    const DIMENSION: Dimension = Dimension::length();
    const NAME: &'static str = "Length";
}

impl crate::quantity::CanAddSameQuantity for Length {}

// ============================================================================
// SI Base Unit
// ============================================================================

/// Meter - SI base unit for length
#[derive(Debug, Clone, Copy)]
pub struct Meter;

impl Unit for Meter {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "m";
    const TO_SI: f64 = 1.0;
    const OFFSET: f64 = 0.0;
}

// ============================================================================
// Metric Units
// ============================================================================

/// Kilometer (1 km = 1000 m)
#[derive(Debug, Clone, Copy)]
pub struct Kilometer;

impl Unit for Kilometer {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "km";
    const TO_SI: f64 = 1000.0;
    const OFFSET: f64 = 0.0;
}

/// Centimeter (1 cm = 0.01 m)
#[derive(Debug, Clone, Copy)]
pub struct Centimeter;

impl Unit for Centimeter {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "cm";
    const TO_SI: f64 = 0.01;
    const OFFSET: f64 = 0.0;
}

/// Millimeter (1 mm = 0.001 m)
#[derive(Debug, Clone, Copy)]
pub struct Millimeter;

impl Unit for Millimeter {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "mm";
    const TO_SI: f64 = 0.001;
    const OFFSET: f64 = 0.0;
}

/// Micrometer (1 μm = 1e-6 m)
#[derive(Debug, Clone, Copy)]
pub struct Micrometer;

impl Unit for Micrometer {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "μm";
    const TO_SI: f64 = 1e-6;
    const OFFSET: f64 = 0.0;
}

/// Nanometer (1 nm = 1e-9 m)
#[derive(Debug, Clone, Copy)]
pub struct Nanometer;

impl Unit for Nanometer {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "nm";
    const TO_SI: f64 = 1e-9;
    const OFFSET: f64 = 0.0;
}

// ============================================================================
// Imperial/US Units
// ============================================================================

/// Inch (1 in = 0.0254 m)
#[derive(Debug, Clone, Copy)]
pub struct Inch;

impl Unit for Inch {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "in";
    const TO_SI: f64 = 0.0254;
    const OFFSET: f64 = 0.0;
}

/// Foot (1 ft = 0.3048 m)
#[derive(Debug, Clone, Copy)]
pub struct Foot;

impl Unit for Foot {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "ft";
    const TO_SI: f64 = 0.3048;
    const OFFSET: f64 = 0.0;
}

/// Yard (1 yd = 0.9144 m)
#[derive(Debug, Clone, Copy)]
pub struct Yard;

impl Unit for Yard {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "yd";
    const TO_SI: f64 = 0.9144;
    const OFFSET: f64 = 0.0;
}

/// Mile (1 mi = 1609.344 m)
#[derive(Debug, Clone, Copy)]
pub struct Mile;

impl Unit for Mile {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "mi";
    const TO_SI: f64 = 1609.344;
    const OFFSET: f64 = 0.0;
}

// ============================================================================
// Astronomical Units
// ============================================================================

/// Astronomical Unit (1 AU ≈ 1.496e11 m)
#[derive(Debug, Clone, Copy)]
pub struct AstronomicalUnit;

impl Unit for AstronomicalUnit {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "AU";
    const TO_SI: f64 = 1.495_978_707e11;
    const OFFSET: f64 = 0.0;
}

/// Light Year (1 ly ≈ 9.461e15 m)
#[derive(Debug, Clone, Copy)]
pub struct LightYear;

impl Unit for LightYear {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "ly";
    const TO_SI: f64 = 9.460_730_472_580_8e15;
    const OFFSET: f64 = 0.0;
}

/// Parsec (1 pc ≈ 3.086e16 m)
#[derive(Debug, Clone, Copy)]
pub struct Parsec;

impl Unit for Parsec {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "pc";
    const TO_SI: f64 = 3.085_677_581e16;
    const OFFSET: f64 = 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_length_dimension() {
        assert_eq!(Length::dimension(), Dimension::length());
    }

    #[test]
    fn test_meter_conversion() {
        let m = Value::<Length, Meter>::new(1.0);
        assert_eq!(m.get_si(), 1.0);
    }

    #[test]
    fn test_kilometer_conversion() {
        let km = Value::<Length, Kilometer>::new(1.0);
        assert_eq!(km.get_si(), 1000.0);

        let m = km.convert::<Meter>();
        assert_eq!(m.get(), 1000.0);
    }

    #[test]
    fn test_centimeter_conversion() {
        let cm = Value::<Length, Centimeter>::new(100.0);
        let m = cm.convert::<Meter>();
        assert_eq!(m.get(), 1.0);
    }

    #[test]
    fn test_inch_conversion() {
        let inch = Value::<Length, Inch>::new(1.0);
        let m = inch.convert::<Meter>();
        assert!((m.get() - 0.0254).abs() < 1e-10);
    }

    #[test]
    fn test_length_addition() {
        let m1 = Value::<Length, Meter>::new(100.0);
        let km1 = Value::<Length, Kilometer>::new(1.0);
        let result = m1 + km1;
        assert_eq!(result.get(), 1100.0);
    }
}
