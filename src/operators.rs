use crate::quantity::{Quantity, CanAddSameQuantity};
use crate::unit::Unit;
use crate::value::Value;
use core::ops::{Add, Sub, Mul, Div, Neg};

// ============================================================================
// Addition
// ============================================================================

/// Addition of two values with the same quantity
///
/// Only values with the same dimensional signature can be added.
/// The result has the same quantity and the unit of the left operand.
///
/// Note: This only works for quantities that implement CanAddSameQuantity.
/// Absolute temperatures cannot be added together (use TemperatureDifference instead).
impl<Q, U1, U2> Add<Value<Q, U2>> for Value<Q, U1>
where
    Q: CanAddSameQuantity,
    U1: Unit<BaseQuantity = Q>,
    U2: Unit<BaseQuantity = Q>,
{
    type Output = Value<Q, U1>;

    #[inline]
    fn add(self, rhs: Value<Q, U2>) -> Self::Output {
        // Add SI values for accuracy
        let result_si = self.get_si() + rhs.get_si();
        Value::<Q, U1>::from_si(result_si)
    }
}

// ============================================================================
// Subtraction
// ============================================================================

/// Subtraction of two values with the same quantity
///
/// Only values with the same dimensional signature can be subtracted.
/// The result has the same quantity and the unit of the left operand.
///
/// Note: This only works for quantities that implement CanAddSameQuantity.
/// For absolute temperatures, subtraction is handled specially and returns a TemperatureDifference.
impl<Q, U1, U2> Sub<Value<Q, U2>> for Value<Q, U1>
where
    Q: CanAddSameQuantity,
    U1: Unit<BaseQuantity = Q>,
    U2: Unit<BaseQuantity = Q>,
{
    type Output = Value<Q, U1>;

    #[inline]
    fn sub(self, rhs: Value<Q, U2>) -> Self::Output {
        // Subtract SI values for accuracy
        let result_si = self.get_si() - rhs.get_si();
        Value::<Q, U1>::from_si(result_si)
    }
}

// ============================================================================
// Negation
// ============================================================================

/// Negation of a value
impl<Q, U> Neg for Value<Q, U>
where
    Q: Quantity,
    U: Unit<BaseQuantity = Q>,
{
    type Output = Value<Q, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        Value::<Q, U>::from_si(-self.get_si())
    }
}

// ============================================================================
// Multiplication by scalar
// ============================================================================

/// Multiply a value by a scalar (f64)
impl<Q, U> Mul<f64> for Value<Q, U>
where
    Q: Quantity,
    U: Unit<BaseQuantity = Q>,
{
    type Output = Value<Q, U>;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Value::<Q, U>::from_si(self.get_si() * rhs)
    }
}

/// Multiply a scalar (f64) by a value
impl<Q, U> Mul<Value<Q, U>> for f64
where
    Q: Quantity,
    U: Unit<BaseQuantity = Q>,
{
    type Output = Value<Q, U>;

    #[inline]
    fn mul(self, rhs: Value<Q, U>) -> Self::Output {
        Value::<Q, U>::from_si(self * rhs.get_si())
    }
}

// ============================================================================
// Division by scalar
// ============================================================================

/// Divide a value by a scalar (f64)
impl<Q, U> Div<f64> for Value<Q, U>
where
    Q: Quantity,
    U: Unit<BaseQuantity = Q>,
{
    type Output = Value<Q, U>;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Value::<Q, U>::from_si(self.get_si() / rhs)
    }
}

// ============================================================================
// Multiplication of values - Results in compound quantities
// ============================================================================

// Note: For multiplying two values with different quantities, we need to
// define the result quantity. This requires implementing specific traits
// for each combination. We'll provide a framework for this.

/// Trait for defining multiplication between quantities
///
/// This trait allows defining what quantity results from multiplying
/// two other quantities together.
///
/// # Example
///
/// ```rust
/// // Length * Length = Area
/// impl QuantityMul<Length> for Length {
///     type Output = Area;
/// }
/// ```
pub trait QuantityMul<Rhs: Quantity>: Quantity {
    type Output: Quantity;
}

/// Trait for defining division between quantities
///
/// This trait allows defining what quantity results from dividing
/// one quantity by another.
///
/// # Example
///
/// ```rust
/// // Length / Time = Velocity
/// impl QuantityDiv<Time> for Length {
///     type Output = Velocity;
/// }
/// ```
pub trait QuantityDiv<Rhs: Quantity>: Quantity {
    type Output: Quantity;
}

// ============================================================================
// Division of values with the same quantity = dimensionless ratio
// ============================================================================

/// Dividing two values of the same quantity yields a dimensionless ratio
impl<Q, U1, U2> Div<Value<Q, U2>> for Value<Q, U1>
where
    Q: Quantity,
    U1: Unit<BaseQuantity = Q>,
    U2: Unit<BaseQuantity = Q>,
{
    type Output = f64;

    #[inline]
    fn div(self, rhs: Value<Q, U2>) -> Self::Output {
        self.get_si() / rhs.get_si()
    }
}

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

    impl CanAddSameQuantity for TestQuantity {}

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
    fn test_addition_same_unit() {
        let a = Value::<TestQuantity, TestUnit1>::new(10.0);
        let b = Value::<TestQuantity, TestUnit1>::new(5.0);
        let result = a + b;
        assert_eq!(result.get(), 15.0);
    }

    #[test]
    fn test_addition_different_units() {
        let a = Value::<TestQuantity, TestUnit1>::new(10.0); // 10 in SI
        let b = Value::<TestQuantity, TestUnit2>::new(5.0);  // 10 in SI (5 * 2.0)
        let result = a + b;
        assert_eq!(result.get(), 20.0); // 20 in unit1
    }

    #[test]
    fn test_subtraction() {
        let a = Value::<TestQuantity, TestUnit1>::new(10.0);
        let b = Value::<TestQuantity, TestUnit1>::new(5.0);
        let result = a - b;
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_negation() {
        let a = Value::<TestQuantity, TestUnit1>::new(10.0);
        let result = -a;
        assert_eq!(result.get(), -10.0);
    }

    #[test]
    fn test_scalar_multiplication() {
        let a = Value::<TestQuantity, TestUnit1>::new(10.0);
        let result = a * 2.0;
        assert_eq!(result.get(), 20.0);

        let result2 = 3.0 * a;
        assert_eq!(result2.get(), 30.0);
    }

    #[test]
    fn test_scalar_division() {
        let a = Value::<TestQuantity, TestUnit1>::new(10.0);
        let result = a / 2.0;
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_value_division() {
        let a = Value::<TestQuantity, TestUnit1>::new(10.0);
        let b = Value::<TestQuantity, TestUnit1>::new(5.0);
        let ratio = a / b;
        assert_eq!(ratio, 2.0);
    }
}
