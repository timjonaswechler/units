//! Advanced unit composition system for variadic types.
//!
//! This module extends the composition system to work with variadic unit types,
//! providing type-safe dimensional analysis for complex unit combinations.

use crate::core::*;
use crate::prefix::*;
use crate::variadic::unit_factors::UnitFactor;
use std::marker::PhantomData;

/// A unit raised to an exponent for variadic systems.
///
/// This type represents mathematical powers of units in variadic contexts,
/// such as `Meter²` or `Second⁻¹`. It maintains type safety while allowing
/// complex unit compositions in variadic syntax.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct VariadicExponent<U, const N: i8>(PhantomData<U>);

impl<U, const N: i8> UnitSymbol for VariadicExponent<U, N>
where
    U: UnitSymbol,
{
    fn symbol() -> &'static str {
        match N {
            1 => U::symbol(),
            2 => Box::leak(format!("{}²", U::symbol()).into_boxed_str()),
            3 => Box::leak(format!("{}³", U::symbol()).into_boxed_str()),
            -1 => Box::leak(format!("{}⁻¹", U::symbol()).into_boxed_str()),
            -2 => Box::leak(format!("{}⁻²", U::symbol()).into_boxed_str()),
            -3 => Box::leak(format!("{}⁻³", U::symbol()).into_boxed_str()),
            _ => Box::leak(format!("{}^{}", U::symbol(), N).into_boxed_str()),
        }
    }
}

impl<U, const N: i8> UnitFactor for VariadicExponent<U, N>
where
    U: UnitFactor,
{
    fn factor() -> f64 {
        let base_factor = U::factor();
        if N == 0 {
            1.0
        } else if N > 0 {
            (0..N).fold(1.0, |acc, _| acc * base_factor)
        } else {
            (0..(-N)).fold(1.0, |acc, _| acc / base_factor)
        }
    }
}

/// Trait for variadic power operations.
///
/// This trait enables natural power operations on variadic quantities,
/// such as `velocity.squared()` for kinetic energy calculations.
pub trait VariadicPower<const N: i8> {
    type Output;
    
    /// Raise this quantity to the Nth power.
    fn power(self) -> Self::Output;
}

// Specializations for common powers
pub trait VariadicSquare {
    type Output;
    fn squared(self) -> Self::Output;
}

pub trait VariadicCube {
    type Output;
    fn cubed(self) -> Self::Output;
}

// Implementation for distance quantities in variadic context
impl<Unit> VariadicSquare for Quantity<Unit, 1, 0, 0, 0, 0, 0, 0>
where
    Unit: UnitSymbol + UnitFactor,
    Self: ToSI,
{
    type Output = Quantity<VariadicExponent<Unit, 2>, 2, 0, 0, 0, 0, 0, 0>;

    fn squared(self) -> Self::Output {
        let si_value = self.to_si();
        let result_si = si_value * si_value;
        Self::Output::new(result_si)
    }
}

impl<Unit> VariadicCube for Quantity<Unit, 1, 0, 0, 0, 0, 0, 0>
where
    Unit: UnitSymbol + UnitFactor,
    Self: ToSI,
{
    type Output = Quantity<VariadicExponent<Unit, 3>, 3, 0, 0, 0, 0, 0, 0>;

    fn cubed(self) -> Self::Output {
        let si_value = self.to_si();
        let result_si = si_value * si_value * si_value;
        Self::Output::new(result_si)
    }
}

/// Trait for variadic root operations.
pub trait VariadicRoot<const N: i8> {
    type Output;
    
    /// Take the Nth root of this quantity.
    fn root(self) -> Self::Output;
}

pub trait VariadicSqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

// Implementation for area quantities to get distance
impl<Unit> VariadicSqrt for Quantity<VariadicExponent<Unit, 2>, 2, 0, 0, 0, 0, 0, 0>
where
    Unit: UnitSymbol + UnitFactor,
    Self: ToSI,
{
    type Output = Quantity<Unit, 1, 0, 0, 0, 0, 0, 0>;

    fn sqrt(self) -> Self::Output {
        let si_value = self.to_si();
        let result_si = si_value.sqrt();
        Self::Output::new(result_si)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantities::*;

    #[test]
    fn test_variadic_exponent_symbol() {
        assert_eq!(VariadicExponent::<Meter, 2>::symbol(), "m²");
        assert_eq!(VariadicExponent::<Second, -1>::symbol(), "s⁻¹");
    }

    #[test]
    fn test_variadic_power_operations() {
        let distance = Distance::<Meter>::new(5.0);
        let area = distance.squared();
        assert_eq!(area.value(), 25.0);

        let volume = distance.cubed();
        assert_eq!(volume.value(), 125.0);
    }
}