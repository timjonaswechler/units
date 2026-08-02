use crate::dimension::Dimension;
use crate::quantity::Quantity;
use crate::unit::Unit;
use crate::value::Value;
use core::ops::{Add, Sub};

/// Physical quantity: Absolute Temperature
///
/// Represents an absolute temperature (e.g., "20°C", "293.15 K")
/// Cannot be added to another absolute temperature (that would be meaningless).
/// Can only be subtracted from another absolute temperature to get a difference.
///
/// SI Base Unit: Kelvin (K)
#[derive(Debug, Clone, Copy)]
pub struct AbsoluteTemperature;

impl Quantity for AbsoluteTemperature {
    const DIMENSION: Dimension = Dimension::temperature();
    const NAME: &'static str = "AbsoluteTemperature";
}

/// Physical quantity: Temperature Difference
///
/// Represents a temperature difference or delta (e.g., "+10°C", "+10 K")
/// Can be added to absolute temperatures and to other differences.
/// This is what you get when you subtract two absolute temperatures.
///
/// SI Base Unit: Kelvin (K)
#[derive(Debug, Clone, Copy)]
pub struct TemperatureDifference;

impl Quantity for TemperatureDifference {
    const DIMENSION: Dimension = Dimension::temperature();
    const NAME: &'static str = "TemperatureDifference";
}

// Temperature differences CAN be added together
impl crate::quantity::CanAddSameQuantity for TemperatureDifference {}

// ============================================================================
// Units for Absolute Temperature
// ============================================================================

/// Kelvin - SI base unit for absolute temperature
#[derive(Debug, Clone, Copy)]
pub struct Kelvin;

impl Unit for Kelvin {
    type BaseQuantity = AbsoluteTemperature;
    const SYMBOL: &'static str = "K";
    const TO_SI: f64 = 1.0;
    const OFFSET: f64 = 0.0;
}

/// Celsius - offset-based temperature unit
/// 0°C = 273.15 K
#[derive(Debug, Clone, Copy)]
pub struct Celsius;

impl Unit for Celsius {
    type BaseQuantity = AbsoluteTemperature;
    const SYMBOL: &'static str = "°C";
    const TO_SI: f64 = 1.0;
    const OFFSET: f64 = 273.15;
}

/// Fahrenheit - offset and scale-based temperature unit
/// T(K) = (T(°F) - 32) × 5/9 + 273.15
/// This is more complex but we can express it as:
/// T(K) = T(°F) × 5/9 + 255.372222...
#[derive(Debug, Clone, Copy)]
pub struct Fahrenheit;

impl Unit for Fahrenheit {
    type BaseQuantity = AbsoluteTemperature;
    const SYMBOL: &'static str = "°F";
    const TO_SI: f64 = 5.0 / 9.0;
    const OFFSET: f64 = 255.37222222222223; // 273.15 - 32 * 5/9
}

// ============================================================================
// Units for Temperature Difference
// ============================================================================

/// Kelvin difference - SI unit for temperature differences
#[derive(Debug, Clone, Copy)]
pub struct KelvinDelta;

impl Unit for KelvinDelta {
    type BaseQuantity = TemperatureDifference;
    const SYMBOL: &'static str = "K";
    const TO_SI: f64 = 1.0;
    const OFFSET: f64 = 0.0; // No offset for differences!
}

/// Celsius difference - same as Kelvin difference (no offset)
/// A change of 1°C is the same as a change of 1 K
#[derive(Debug, Clone, Copy)]
pub struct CelsiusDelta;

impl Unit for CelsiusDelta {
    type BaseQuantity = TemperatureDifference;
    const SYMBOL: &'static str = "°C";
    const TO_SI: f64 = 1.0;
    const OFFSET: f64 = 0.0; // No offset for differences!
}

/// Fahrenheit difference
/// A change of 1°F = 5/9 K
#[derive(Debug, Clone, Copy)]
pub struct FahrenheitDelta;

impl Unit for FahrenheitDelta {
    type BaseQuantity = TemperatureDifference;
    const SYMBOL: &'static str = "°F";
    const TO_SI: f64 = 5.0 / 9.0;
    const OFFSET: f64 = 0.0; // No offset for differences!
}

// ============================================================================
// Special Arithmetic for Temperatures
// ============================================================================

// Rule 1: AbsoluteTemperature - AbsoluteTemperature = TemperatureDifference
impl<U1, U2> Sub<Value<AbsoluteTemperature, U2>> for Value<AbsoluteTemperature, U1>
where
    U1: Unit<BaseQuantity = AbsoluteTemperature>,
    U2: Unit<BaseQuantity = AbsoluteTemperature>,
{
    type Output = Value<TemperatureDifference, KelvinDelta>;

    #[inline]
    fn sub(self, rhs: Value<AbsoluteTemperature, U2>) -> Self::Output {
        // Subtract SI values to get the difference
        let diff_si = self.get_si() - rhs.get_si();
        Value::<TemperatureDifference, KelvinDelta>::from_si(diff_si)
    }
}

// Rule 2: AbsoluteTemperature + TemperatureDifference = AbsoluteTemperature
impl<U1, U2> Add<Value<TemperatureDifference, U2>> for Value<AbsoluteTemperature, U1>
where
    U1: Unit<BaseQuantity = AbsoluteTemperature>,
    U2: Unit<BaseQuantity = TemperatureDifference>,
{
    type Output = Value<AbsoluteTemperature, U1>;

    #[inline]
    fn add(self, rhs: Value<TemperatureDifference, U2>) -> Self::Output {
        // Add the difference to the absolute temperature
        let result_si = self.get_si() + rhs.get_si();
        Value::<AbsoluteTemperature, U1>::from_si(result_si)
    }
}

// Rule 3: AbsoluteTemperature - TemperatureDifference = AbsoluteTemperature
impl<U1, U2> Sub<Value<TemperatureDifference, U2>> for Value<AbsoluteTemperature, U1>
where
    U1: Unit<BaseQuantity = AbsoluteTemperature>,
    U2: Unit<BaseQuantity = TemperatureDifference>,
{
    type Output = Value<AbsoluteTemperature, U1>;

    #[inline]
    fn sub(self, rhs: Value<TemperatureDifference, U2>) -> Self::Output {
        // Subtract the difference from the absolute temperature
        let result_si = self.get_si() - rhs.get_si();
        Value::<AbsoluteTemperature, U1>::from_si(result_si)
    }
}

// Rule 4 & 5: TemperatureDifference +/- TemperatureDifference
// These are handled by the generic implementations in operators.rs
// because TemperatureDifference implements CanAddSameQuantity

// Note: AbsoluteTemperature + AbsoluteTemperature is NOT implemented!
// AbsoluteTemperature does NOT implement CanAddSameQuantity,
// so this will cause a compile error, which is exactly what we want.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kelvin_to_celsius() {
        let k = Value::<AbsoluteTemperature, Kelvin>::new(273.15);
        let c = k.convert::<Celsius>();
        assert!((c.get() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        let c = Value::<AbsoluteTemperature, Celsius>::new(0.0);
        assert!((c.get_si() - 273.15).abs() < 1e-10);
    }

    #[test]
    fn test_celsius_conversion() {
        let c = Value::<AbsoluteTemperature, Celsius>::new(20.0);
        let k = c.convert::<Kelvin>();
        assert!((k.get() - 293.15).abs() < 1e-10);
    }

    #[test]
    fn test_fahrenheit_conversion() {
        let f = Value::<AbsoluteTemperature, Fahrenheit>::new(32.0);
        let c = f.convert::<Celsius>();
        assert!((c.get() - 0.0).abs() < 1e-10);

        let f2 = Value::<AbsoluteTemperature, Fahrenheit>::new(212.0);
        let c2 = f2.convert::<Celsius>();
        assert!((c2.get() - 100.0).abs() < 1e-9);
    }

    #[test]
    fn test_absolute_subtraction() {
        let t1 = Value::<AbsoluteTemperature, Celsius>::new(30.0);
        let t2 = Value::<AbsoluteTemperature, Celsius>::new(20.0);
        let diff = t1 - t2;
        assert!((diff.get() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_add_difference_to_absolute() {
        let temp = Value::<AbsoluteTemperature, Celsius>::new(20.0);
        let diff = Value::<TemperatureDifference, KelvinDelta>::new(10.0);
        let result = temp + diff;
        assert!((result.get() - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_subtract_difference_from_absolute() {
        let temp = Value::<AbsoluteTemperature, Celsius>::new(20.0);
        let diff = Value::<TemperatureDifference, KelvinDelta>::new(10.0);
        let result = temp - diff;
        assert!((result.get() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_add_differences() {
        let diff1 = Value::<TemperatureDifference, KelvinDelta>::new(10.0);
        let diff2 = Value::<TemperatureDifference, KelvinDelta>::new(5.0);
        let result = diff1 + diff2;
        assert!((result.get() - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_the_celsius_problem_from_idea_md() {
        // The problem from idea.md:
        // 10°C + 20°C should NOT equal 30°C when done naively
        // But with our system, this operation is not allowed!

        let t1 = Value::<AbsoluteTemperature, Celsius>::new(10.0);
        let t2 = Value::<AbsoluteTemperature, Celsius>::new(20.0);

        // This would be a compile error:
        // let invalid = t1 + t2;  // ❌ Cannot add two absolute temperatures!

        // Instead, we can:
        // 1. Subtract to get a difference
        let diff = t2 - t1;
        assert!((diff.get() - 10.0).abs() < 1e-10);

        // 2. Add that difference to an absolute temperature
        let t3 = t1 + diff;
        assert!((t3.get() - 20.0).abs() < 1e-10);

        // 3. If we want to add temperature changes:
        let change1 = Value::<TemperatureDifference, CelsiusDelta>::new(10.0);
        let change2 = Value::<TemperatureDifference, CelsiusDelta>::new(20.0);
        let total_change = change1 + change2;
        assert!((total_change.get() - 30.0).abs() < 1e-10);

        // 4. Apply the total change to a starting temperature
        let start = Value::<AbsoluteTemperature, Celsius>::new(0.0);
        let final_temp = start + total_change;
        assert!((final_temp.get() - 30.0).abs() < 1e-10);
    }
}
