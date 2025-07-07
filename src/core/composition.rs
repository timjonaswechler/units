//! Compositional operators for building complex units
//!
//! This module provides operators for composing complex units from simpler ones:
//! - `Per<U>` - Inverts the dimensions of a unit (e.g., Per<Second> = T⁻¹)
//! - `Exponent<U, N>` - Raises a unit to a power (e.g., Exponent<Meter, 2> = L²)
//! - Tuple implementations for multiplication

use super::DimensionExtractor;
use std::marker::PhantomData;

/// Inverts all dimensions of a unit type
///
/// `Per<U>` represents the reciprocal of unit `U`. For example:
/// - `Per<Second>` represents s⁻¹ (frequency dimension)
/// - `Per<Meter>` represents m⁻¹ (wavenumber dimension)
///
/// # Examples
///
/// ```rust
/// use physics_units::core::composition::Per;
/// use physics_units::units::base::{Meter, Second};
///
/// // Velocity: meter per second
/// type Velocity = (Meter, Per<Second>);
///
/// // Frequency: per second
/// type Frequency = Per<Second>;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Per<U>(PhantomData<U>);

impl<U: DimensionExtractor> DimensionExtractor for Per<U> {
    const L: i8 = -U::L;
    const M: i8 = -U::M;
    const T: i8 = -U::T;
    const THETA: i8 = -U::THETA;
    const I: i8 = -U::I;
    const J: i8 = -U::J;
    const N: i8 = -U::N;
}

/// Raises a unit to a power
///
/// `Exponent<U, N>` represents unit `U` raised to the power `N`. For example:
/// - `Exponent<Meter, 2>` represents m² (area dimension)
/// - `Exponent<Second, -1>` represents s⁻¹ (same as `Per<Second>`)
///
/// # Examples
///
/// ```rust
/// use physics_units::core::composition::Exponent;
/// use physics_units::units::base::{Meter, Second};
///
/// // Area: square meters
/// type Area = Exponent<Meter, 2>;
///
/// // Acceleration: meter per second squared
/// type Acceleration = (Meter, Per<Exponent<Second, 2>>);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exponent<U, const N: i8>(PhantomData<U>);

impl<U: DimensionExtractor, const N: i8> DimensionExtractor for Exponent<U, N> {
    const L: i8 = U::L * N;
    const M: i8 = U::M * N;
    const T: i8 = U::T * N;
    const THETA: i8 = U::THETA * N;
    const I: i8 = U::I * N;
    const J: i8 = U::J * N;
    const N: i8 = U::N * N;
}

/// Represents a dimensionless unit (no dimensions)
///
/// Used for dimensionless quantities like angles, ratios, and pure numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DimensionlessUnit;

impl DimensionExtractor for DimensionlessUnit {
    // All dimensions are 0 by default
}

// Tuple composition for 2 units (multiplication)
impl<U1: DimensionExtractor, U2: DimensionExtractor> DimensionExtractor for (U1, U2) {
    const L: i8 = U1::L + U2::L;
    const M: i8 = U1::M + U2::M;
    const T: i8 = U1::T + U2::T;
    const THETA: i8 = U1::THETA + U2::THETA;
    const I: i8 = U1::I + U2::I;
    const J: i8 = U1::J + U2::J;
    const N: i8 = U1::N + U2::N;
}

// Tuple composition for 3 units
impl<U1, U2, U3> DimensionExtractor for (U1, U2, U3)
where
    U1: DimensionExtractor,
    U2: DimensionExtractor,
    U3: DimensionExtractor,
{
    const L: i8 = U1::L + U2::L + U3::L;
    const M: i8 = U1::M + U2::M + U3::M;
    const T: i8 = U1::T + U2::T + U3::T;
    const THETA: i8 = U1::THETA + U2::THETA + U3::THETA;
    const I: i8 = U1::I + U2::I + U3::I;
    const J: i8 = U1::J + U2::J + U3::J;
    const N: i8 = U1::N + U2::N + U3::N;
}

// Tuple composition for 4 units
impl<U1, U2, U3, U4> DimensionExtractor for (U1, U2, U3, U4)
where
    U1: DimensionExtractor,
    U2: DimensionExtractor,
    U3: DimensionExtractor,
    U4: DimensionExtractor,
{
    const L: i8 = U1::L + U2::L + U3::L + U4::L;
    const M: i8 = U1::M + U2::M + U3::M + U4::M;
    const T: i8 = U1::T + U2::T + U3::T + U4::T;
    const THETA: i8 = U1::THETA + U2::THETA + U3::THETA + U4::THETA;
    const I: i8 = U1::I + U2::I + U3::I + U4::I;
    const J: i8 = U1::J + U2::J + U3::J + U4::J;
    const N: i8 = U1::N + U2::N + U3::N + U4::N;
}

// TODO: Unit simplification rules
// These conflict with the generic implementations above.
// We'll need to use type-level programming techniques or macros to implement these properly.
// For now, users can manually simplify their types if needed.
// // Unit simplification rules                                                         │ │
//                                                                                      │ │
// /// Double inversion simplification: Per<Per<U>> = U                                 │ │
// impl<U: DimensionExtractor> DimensionExtractor for Per<Per<U>> {                     │ │
//     const L: i8 = U::L;                                                              │ │
//     const M: i8 = U::M;                                                              │ │
//     const T: i8 = U::T;                                                              │ │
//     const THETA: i8 = U::THETA;                                                      │ │
//     const I: i8 = U::I;                                                              │ │
//     const J: i8 = U::J;                                                              │ │
//     const N: i8 = U::N;                                                              │ │
// }                                                                                    │ │
//                                                                                      │ │
// /// Power of 1 simplification: Exponent<U, 1> = U                                    │ │
// impl<U: DimensionExtractor> DimensionExtractor for Exponent<U, 1> {                  │ │
//     const L: i8 = U::L;                                                              │ │
//     const M: i8 = U::M;                                                              │ │
//     const T: i8 = U::T;                                                              │ │
//     const THETA: i8 = U::THETA;                                                      │ │
//     const I: i8 = U::I;                                                              │ │
//     const J: i8 = U::J;                                                              │ │
//     const N: i8 = U::N;                                                              │ │
// }                                                                                    │ │
//                                                                                      │ │
// /// Power of 0 simplification: Exponent<U, 0> = dimensionless                        │ │
// impl<U: DimensionExtractor> DimensionExtractor for Exponent<U, 0> {                  │ │
//     const L: i8 = 0;                                                                 │ │
//     const M: i8 = 0;                                                                 │ │
//     const T: i8 = 0;                                                                 │ │
//     const THETA: i8 = 0;                                                             │ │
//     const I: i8 = 0;                                                                 │ │
//     const J: i8 = 0;                                                                 │ │
//     const N: i8 = 0;                                                                 │ │
// }
