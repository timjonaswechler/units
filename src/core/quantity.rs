//! Core quantity type for dimensional analysis
//!
//! This module defines the main `Quantity<U, V>` type that represents a numerical value
//! with associated dimensional units.

use super::DimensionExtractor;
use crate::DefaultFloat;
use crate::arithmetic::conversion::UnitConverter;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::{Mul, Div};

/// A physical quantity with dimensional units
///
/// `Quantity<U, V>` represents a numerical value of type `V` with dimensional units of type `U`.
/// The dimensional information is encoded entirely at the type level and incurs zero runtime cost.
///
/// # Type Parameters
///
/// - `U`: Unit type that implements `DimensionExtractor` - encodes dimensional information
/// - `V`: Value type for the numerical value (defaults to `DefaultFloat` from feature flags)
///
/// # Examples
///
/// ```rust
/// use physics_units::prelude::*;
///
/// // Basic quantities with simple units
/// let distance: Quantity<Meter> = Quantity::new(5.0);
/// let time: Quantity<Second> = Quantity::new(2.0);
///
/// // Complex units using composition
/// let velocity: Quantity<(Meter, Per<Second>)> = distance / time;
///
/// // Physical constants with proper dimensions
/// let planck: Quantity<(Joule, Second)> = Quantity::new(6.62607015e-34);
/// ```
#[derive(Clone, PartialEq)]
pub struct Quantity<U, V = DefaultFloat>
where
    U: DimensionExtractor,
{
    value: V,
    _phantom: PhantomData<U>,
}

// Implement Copy manually with appropriate bounds
impl<U, V> Copy for Quantity<U, V>
where
    U: DimensionExtractor + Copy,
    V: Copy,
{
}

impl<U, V> Quantity<U, V>
where
    U: DimensionExtractor,
{
    /// Create a new quantity with the given value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use physics_units::prelude::*;
    ///
    /// let distance = Quantity::<Meter>::new(10.0);
    /// let time = Quantity::<Second>::new(2.5);
    /// ```
    #[inline]
    pub const fn new(value: V) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    /// Get the numerical value of this quantity
    ///
    /// # Examples
    ///
    /// ```rust
    /// use physics_units::prelude::*;
    ///
    /// let distance = Quantity::<Meter>::new(10.0);
    /// assert_eq!(distance.value(), 10.0);
    /// ```
    #[inline]
    pub const fn value(&self) -> V
    where
        V: Copy,
    {
        self.value
    }

    /// Get a reference to the numerical value
    #[inline]
    pub const fn value_ref(&self) -> &V {
        &self.value
    }

    /// Get a mutable reference to the numerical value
    #[inline]
    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    /// Get the absolute value of this quantity
    #[inline]
    pub fn abs(self) -> Self
    where
        V: num_traits::Float,
    {
        Quantity::new(self.value().abs())
    }

    /// Get the square root of this quantity
    #[inline]
    pub fn sqrt(self) -> Self
    where
        V: num_traits::Float,
    {
        // TODO: Implement proper dimensional square root operations
        // For now, just take sqrt of value - dimensions stay the same
        Quantity::new(self.value().sqrt())
    }
}

// Implement common traits for Quantity

impl<U, V> Default for Quantity<U, V>
where
    U: DimensionExtractor,
    V: Default,
{
    fn default() -> Self {
        Self::new(V::default())
    }
}

// Display implementation moved to formatting/display.rs

// Type aliases for common quantity types
/// Distance quantity (length dimension L¹)
pub type Distance<U, V = DefaultFloat> = Quantity<U, V>;

/// Mass quantity (mass dimension M¹)
pub type Mass<U, V = DefaultFloat> = Quantity<U, V>;

/// Time quantity (time dimension T¹)
pub type Time<U, V = DefaultFloat> = Quantity<U, V>;

/// Temperature quantity (temperature dimension Θ¹)
pub type Temperature<U, V = DefaultFloat> = Quantity<U, V>;

/// Current quantity (current dimension I¹)
pub type Current<U, V = DefaultFloat> = Quantity<U, V>;

/// Luminous intensity quantity (luminous intensity dimension J¹)
pub type LuminousIntensity<U, V = DefaultFloat> = Quantity<U, V>;

/// Amount of substance quantity (amount dimension N¹)
pub type AmountOfSubstance<U, V = DefaultFloat> = Quantity<U, V>;

/// Velocity quantity (L¹T⁻¹)
pub type Velocity<U, V = DefaultFloat> = Quantity<U, V>;

/// Acceleration quantity (L¹T⁻²)
pub type Acceleration<U, V = DefaultFloat> = Quantity<U, V>;

/// Force quantity (L¹M¹T⁻²)
pub type Force<U, V = DefaultFloat> = Quantity<U, V>;

/// Energy quantity (L²M¹T⁻²)
pub type Energy<U, V = DefaultFloat> = Quantity<U, V>;

/// Power quantity (L²M¹T⁻³)
pub type Power<U, V = DefaultFloat> = Quantity<U, V>;

/// Area quantity (L²)
pub type Area<U, V = DefaultFloat> = Quantity<U, V>;

/// Volume quantity (L³)
pub type Volume<U, V = DefaultFloat> = Quantity<U, V>;

/// Frequency quantity (T⁻¹)
pub type Frequency<U, V = DefaultFloat> = Quantity<U, V>;
