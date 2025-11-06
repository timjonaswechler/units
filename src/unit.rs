use crate::quantity::Quantity;

/// Trait for defining physical units
///
/// A unit is a specific measure of a quantity (like Meter for Length, Second for Time).
/// Each unit has:
/// - An associated Quantity type
/// - A conversion factor to the SI base unit
/// - A symbol for display
///
/// # Type Safety
///
/// Units are tied to specific Quantity types at compile time, ensuring that
/// you cannot accidentally mix incompatible units.
///
/// # Example
///
/// ```rust
/// use units::unit::Unit;
/// use units::quantity::Quantity;
/// use units::dimension::Dimension;
///
/// #[derive(Debug, Clone, Copy)]
/// pub struct Length;
///
/// impl Quantity for Length {
///     const DIMENSION: Dimension = Dimension::length();
///     const NAME: &'static str = "Length";
/// }
///
/// #[derive(Debug, Clone, Copy)]
/// pub struct Meter;
///
/// impl Unit for Meter {
///     type BaseQuantity = Length;
///     const SYMBOL: &'static str = "m";
///     const TO_SI: f64 = 1.0;
///     const OFFSET: f64 = 0.0;
/// }
///
/// #[derive(Debug, Clone, Copy)]
/// pub struct Kilometer;
///
/// impl Unit for Kilometer {
///     type BaseQuantity = Length;
///     const SYMBOL: &'static str = "km";
///     const TO_SI: f64 = 1000.0;
///     const OFFSET: f64 = 0.0;
/// }
/// ```
pub trait Unit: 'static + Copy + Clone + Sized {
    /// The quantity this unit measures
    type BaseQuantity: Quantity;

    /// Symbol for this unit (e.g., "m" for meter, "kg" for kilogram)
    const SYMBOL: &'static str;

    /// Conversion factor to SI base unit
    ///
    /// For example:
    /// - Meter: 1.0 (it IS the SI unit)
    /// - Kilometer: 1000.0 (1 km = 1000 m)
    /// - Centimeter: 0.01 (1 cm = 0.01 m)
    const TO_SI: f64;

    /// Offset for affine conversions (mainly for temperature)
    ///
    /// For most units, this is 0.0.
    /// For temperature units like Celsius:
    /// - Celsius: offset = 273.15 (0°C = 273.15 K)
    /// - Fahrenheit: more complex conversion needed
    const OFFSET: f64;

    /// Convert a value in this unit to SI base unit
    #[inline]
    fn to_si(value: f64) -> f64 {
        value * Self::TO_SI + Self::OFFSET
    }

    /// Convert a value from SI base unit to this unit
    #[inline]
    fn from_si(si_value: f64) -> f64 {
        (si_value - Self::OFFSET) / Self::TO_SI
    }

    /// Get the symbol at runtime
    fn symbol() -> &'static str {
        Self::SYMBOL
    }

    /// Get the dimension of this unit's quantity
    fn dimension() -> crate::dimension::Dimension {
        Self::BaseQuantity::DIMENSION
    }
}

/// Marker trait for units that use affine conversion (like absolute temperature units)
///
/// These units cannot be directly added together because they have an offset.
/// Example: Celsius has an offset of 273.15 from Kelvin
pub trait AffineUnit: Unit {
    /// Whether this unit uses affine conversion (has a non-zero offset)
    const IS_AFFINE: bool = Self::OFFSET != 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimension::Dimension;
    use crate::quantity::Quantity;

    #[derive(Debug, Clone, Copy)]
    struct TestQuantity;

    impl Quantity for TestQuantity {
        const DIMENSION: Dimension = Dimension::length();
        const NAME: &'static str = "TestQuantity";
    }

    #[derive(Debug, Clone, Copy)]
    struct TestUnit;

    impl Unit for TestUnit {
        type BaseQuantity = TestQuantity;
        const SYMBOL: &'static str = "tu";
        const TO_SI: f64 = 2.0;
        const OFFSET: f64 = 0.0;
    }

    #[test]
    fn test_unit_to_si() {
        let result = TestUnit::to_si(5.0);
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_unit_from_si() {
        let result = TestUnit::from_si(10.0);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_unit_symbol() {
        assert_eq!(TestUnit::symbol(), "tu");
    }

    #[test]
    fn test_unit_dimension() {
        assert_eq!(TestUnit::dimension(), Dimension::length());
    }
}
