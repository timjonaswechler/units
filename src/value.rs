use crate::quantity::Quantity;
use crate::unit::Unit;
use core::fmt;
use core::marker::PhantomData;

/// A physical value with compile-time unit and quantity information
///
/// This is the core type that represents a measured value with units.
/// The type parameters ensure compile-time checking of dimensional correctness.
///
/// # Type Parameters
///
/// - `Q`: The quantity being measured (e.g., Length, Mass, Time)
/// - `U`: The unit of measurement (e.g., Meter, Kilogram, Second)
///
/// # Storage
///
/// Values are stored in two forms:
/// - `value`: The value in the specified unit (for display/user interaction)
/// - `si_value`: The value in SI base units (for calculations)
///
/// This dual storage allows efficient conversions and calculations while
/// maintaining the user's preferred display unit.
///
/// # Example
///
/// ```rust
/// use units::prelude::*;
///
/// let distance = Value::<Length, Meter>::new(100.0);
/// let converted = distance.convert::<Kilometer>();
/// assert_eq!(converted.get(), 0.1);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Value<Q: Quantity, U: Unit<BaseQuantity = Q>> {
    /// The value in the unit U
    value: f64,
    /// The value in SI base units (for calculations)
    si_value: f64,
    /// Phantom data to carry type information
    _phantom: PhantomData<(Q, U)>,
}

impl<Q: Quantity, U: Unit<BaseQuantity = Q>> Value<Q, U> {
    /// Creates a new Value with the given amount in unit U
    ///
    /// # Example
    ///
    /// ```rust
    /// use units::prelude::*;
    ///
    /// let distance = Value::<Length, Meter>::new(100.0);
    /// ```
    #[inline]
    pub const fn new(value: f64) -> Self {
        let si_value = value * U::TO_SI + U::OFFSET;
        Self {
            value,
            si_value,
            _phantom: PhantomData,
        }
    }

    /// Creates a Value from an SI base unit value
    ///
    /// This is useful when you have a value in SI units and want to
    /// represent it in a specific unit.
    #[inline]
    pub const fn from_si(si_value: f64) -> Self {
        let value = (si_value - U::OFFSET) / U::TO_SI;
        Self {
            value,
            si_value,
            _phantom: PhantomData,
        }
    }

    /// Gets the value in the current unit
    #[inline]
    pub const fn get(&self) -> f64 {
        self.value
    }

    /// Gets the value in SI base units
    #[inline]
    pub const fn get_si(&self) -> f64 {
        self.si_value
    }

    /// Converts this value to a different unit of the same quantity
    ///
    /// # Example
    ///
    /// ```rust
    /// use units::prelude::*;
    ///
    /// let meters = Value::<Length, Meter>::new(1000.0);
    /// let kilometers = meters.convert::<Kilometer>();
    /// assert_eq!(kilometers.get(), 1.0);
    /// ```
    #[inline]
    pub fn convert<U2: Unit<BaseQuantity = Q>>(self) -> Value<Q, U2> {
        Value::<Q, U2>::from_si(self.si_value)
    }

    /// Returns the absolute value
    #[inline]
    pub fn abs(self) -> Self {
        Self {
            value: self.value.abs(),
            si_value: self.si_value.abs(),
            _phantom: PhantomData,
        }
    }

    /// Returns the square root (only valid if Q^0.5 is meaningful)
    ///
    /// Note: This doesn't change the dimensional type, so use with care.
    /// Prefer type-safe operations when possible.
    #[inline]
    pub fn sqrt(self) -> Self {
        Self {
            value: self.value.sqrt(),
            si_value: self.si_value.sqrt(),
            _phantom: PhantomData,
        }
    }

    /// Raises to the given power (only valid if Q^n is meaningful)
    ///
    /// Note: This doesn't change the dimensional type, so use with care.
    /// Prefer type-safe operations when possible.
    #[inline]
    pub fn powi(self, n: i32) -> Self {
        Self {
            value: self.value.powi(n),
            si_value: self.si_value.powi(n),
            _phantom: PhantomData,
        }
    }

    /// Returns true if the value is finite (not infinite or NaN)
    #[inline]
    pub fn is_finite(self) -> bool {
        self.value.is_finite() && self.si_value.is_finite()
    }

    /// Returns true if the value is NaN
    #[inline]
    pub fn is_nan(self) -> bool {
        self.value.is_nan() || self.si_value.is_nan()
    }

    /// Returns true if the value is positive
    #[inline]
    pub fn is_sign_positive(self) -> bool {
        self.si_value.is_sign_positive()
    }

    /// Returns true if the value is negative
    #[inline]
    pub fn is_sign_negative(self) -> bool {
        self.si_value.is_sign_negative()
    }
}

// Implement PartialEq - compares SI values for accuracy
impl<Q: Quantity, U1: Unit<BaseQuantity = Q>, U2: Unit<BaseQuantity = Q>> PartialEq<Value<Q, U2>>
    for Value<Q, U1>
{
    fn eq(&self, other: &Value<Q, U2>) -> bool {
        // Compare SI values to handle different units correctly
        (self.si_value - other.si_value).abs() < f64::EPSILON * self.si_value.abs().max(other.si_value.abs())
    }
}

// Display implementation
impl<Q: Quantity, U: Unit<BaseQuantity = Q>> fmt::Display for Value<Q, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::SYMBOL)
    }
}

// Copy and Clone are automatically derived

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimension::Dimension;

    #[derive(Debug, Clone, Copy)]
    struct TestQuantity;

    impl Quantity for TestQuantity {
        const DIMENSION: Dimension = Dimension::length();
        const NAME: &'static str = "TestQuantity";
    }

    #[derive(Debug, Clone, Copy)]
    struct TestUnit1;

    impl Unit for TestUnit1 {
        type BaseQuantity = TestQuantity;
        const SYMBOL: &'static str = "u1";
        const TO_SI: f64 = 1.0;
        const OFFSET: f64 = 0.0;
    }

    #[derive(Debug, Clone, Copy)]
    struct TestUnit2;

    impl Unit for TestUnit2 {
        type BaseQuantity = TestQuantity;
        const SYMBOL: &'static str = "u2";
        const TO_SI: f64 = 2.0;
        const OFFSET: f64 = 0.0;
    }

    #[test]
    fn test_value_creation() {
        let val = Value::<TestQuantity, TestUnit1>::new(10.0);
        assert_eq!(val.get(), 10.0);
        assert_eq!(val.get_si(), 10.0);
    }

    #[test]
    fn test_value_conversion() {
        let val1 = Value::<TestQuantity, TestUnit1>::new(10.0);
        let val2 = val1.convert::<TestUnit2>();
        assert_eq!(val2.get(), 5.0); // 10.0 / 2.0
        assert_eq!(val2.get_si(), 10.0);
    }

    #[test]
    fn test_value_from_si() {
        let val = Value::<TestQuantity, TestUnit2>::from_si(10.0);
        assert_eq!(val.get(), 5.0);
        assert_eq!(val.get_si(), 10.0);
    }

    #[test]
    fn test_value_abs() {
        let val = Value::<TestQuantity, TestUnit1>::new(-10.0);
        let abs_val = val.abs();
        assert_eq!(abs_val.get(), 10.0);
    }

    #[test]
    fn test_value_equality() {
        let val1 = Value::<TestQuantity, TestUnit1>::new(10.0);
        let val2 = Value::<TestQuantity, TestUnit2>::new(5.0);
        assert_eq!(val1, val2); // Same SI value
    }
}
