//! Arithmetic operations with automatic dimensional composition
//!
//! This module implements the core arithmetic operations that automatically
//! compose dimensions according to physics rules:
//! - Multiplication: combines dimensions (L¹ × T¹ = L¹T¹)
//! - Division: creates ratios (L¹ ÷ T¹ = L¹T⁻¹)
//! - Addition/Subtraction: requires same dimensions (dimensional compatibility)

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::marker::PhantomData;
use crate::core::{Quantity, DimensionExtractor};
use crate::core::composition::Per;

// ================================================================================================
// MULTIPLICATION - Combines dimensions
// ================================================================================================

impl<U1, U2, V> Mul<Quantity<U2, V>> for Quantity<U1, V>
where
    U1: DimensionExtractor,
    U2: DimensionExtractor,
    V: Mul<V, Output = V> + Copy,
{
    type Output = Quantity<(U1, U2), V>;

    /// Multiplication automatically composes dimensions
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use physics_units::prelude::*;
    /// 
    /// let distance = Quantity::<Meter>::new(10.0);
    /// let time = Quantity::<Second>::new(2.0);
    /// let area_time = distance * time;  // Type: Quantity<(Meter, Second)>
    /// ```
    #[inline]
    fn mul(self, rhs: Quantity<U2, V>) -> Self::Output {
        Quantity::new(self.value() * rhs.value())
    }
}

// Scalar multiplication (quantity * scalar)
impl<U, V> Mul<V> for Quantity<U, V>
where
    U: DimensionExtractor,
    V: Mul<V, Output = V> + Copy,
{
    type Output = Quantity<U, V>;

    #[inline]
    fn mul(self, rhs: V) -> Self::Output {
        Quantity::new(self.value() * rhs)
    }
}

// Scalar multiplication (scalar * quantity)
impl<U, V> Mul<Quantity<U, V>> for f64
where
    U: DimensionExtractor,
    V: Mul<f64, Output = V> + Copy,
{
    type Output = Quantity<U, V>;

    #[inline]
    fn mul(self, rhs: Quantity<U, V>) -> Self::Output {
        Quantity::new(rhs.value() * self)
    }
}

impl<U, V> Mul<Quantity<U, V>> for f32
where
    U: DimensionExtractor,
    V: Mul<f32, Output = V> + Copy,
{
    type Output = Quantity<U, V>;

    #[inline]
    fn mul(self, rhs: Quantity<U, V>) -> Self::Output {
        Quantity::new(rhs.value() * self)
    }
}

// ================================================================================================
// DIVISION - Creates dimensional ratios
// ================================================================================================

impl<U1, U2, V> Div<Quantity<U2, V>> for Quantity<U1, V>
where
    U1: DimensionExtractor,
    U2: DimensionExtractor,
    V: Div<V, Output = V> + Copy,
{
    type Output = Quantity<(U1, Per<U2>), V>;

    /// Division automatically creates dimensional ratios
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use physics_units::prelude::*;
    /// 
    /// let distance = Quantity::<Meter>::new(10.0);
    /// let time = Quantity::<Second>::new(2.0);
    /// let velocity = distance / time;  // Type: Quantity<(Meter, Per<Second>)>
    /// assert_eq!(velocity.value(), 5.0);
    /// ```
    #[inline]
    fn div(self, rhs: Quantity<U2, V>) -> Self::Output {
        Quantity::new(self.value() / rhs.value())
    }
}

// Scalar division (quantity / scalar)
impl<U, V> Div<V> for Quantity<U, V>
where
    U: DimensionExtractor,
    V: Div<V, Output = V> + Copy,
{
    type Output = Quantity<U, V>;

    #[inline]
    fn div(self, rhs: V) -> Self::Output {
        Quantity::new(self.value() / rhs)
    }
}

// Scalar division (scalar / quantity) - creates reciprocal dimensions
impl<U> Div<Quantity<U, f64>> for f64
where
    U: DimensionExtractor,
{
    type Output = Quantity<Per<U>, f64>;

    #[inline]
    fn div(self, rhs: Quantity<U, f64>) -> Self::Output {
        Quantity::new(self / rhs.value())
    }
}

impl<U> Div<Quantity<U, f32>> for f32
where
    U: DimensionExtractor,
{
    type Output = Quantity<Per<U>, f32>;

    #[inline]
    fn div(self, rhs: Quantity<U, f32>) -> Self::Output {
        Quantity::new(self / rhs.value())
    }
}

// ================================================================================================
// ADDITION/SUBTRACTION - Requires dimensional compatibility
// ================================================================================================

impl<U, V> Add<Quantity<U, V>> for Quantity<U, V>
where
    U: DimensionExtractor,
    V: Add<V, Output = V> + Copy,
{
    type Output = Quantity<U, V>;

    /// Addition requires same dimensions (dimensional compatibility)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use physics_units::prelude::*;
    /// 
    /// let d1 = Quantity::<Meter>::new(10.0);
    /// let d2 = Quantity::<Meter>::new(5.0);
    /// let total = d1 + d2;  // Type: Quantity<Meter>
    /// assert_eq!(total.value(), 15.0);
    /// ```
    #[inline]
    fn add(self, rhs: Quantity<U, V>) -> Self::Output {
        Quantity::new(self.value() + rhs.value())
    }
}

impl<U, V> Sub<Quantity<U, V>> for Quantity<U, V>
where
    U: DimensionExtractor,
    V: Sub<V, Output = V> + Copy,
{
    type Output = Quantity<U, V>;

    /// Subtraction requires same dimensions (dimensional compatibility)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use physics_units::prelude::*;
    /// 
    /// let d1 = Quantity::<Meter>::new(10.0);
    /// let d2 = Quantity::<Meter>::new(3.0);
    /// let diff = d1 - d2;  // Type: Quantity<Meter>
    /// assert_eq!(diff.value(), 7.0);
    /// ```
    #[inline]
    fn sub(self, rhs: Quantity<U, V>) -> Self::Output {
        Quantity::new(self.value() - rhs.value())
    }
}

// ================================================================================================
// NEGATION
// ================================================================================================

impl<U, V> Neg for Quantity<U, V>
where
    U: DimensionExtractor,
    V: Neg<Output = V> + Copy,
{
    type Output = Quantity<U, V>;

    /// Negation preserves dimensions
    #[inline]
    fn neg(self) -> Self::Output {
        Quantity::new(-self.value())
    }
}