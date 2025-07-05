//! Multi-unit syntax support for intuitive quantity definitions.
//!
//! This module provides support for defining quantities with multiple unit parameters,
//! enabling more intuitive syntax like:
//! - `Velocity::<Meter, Second>::new(10.0)` for m/s
//! - `Acceleration::<Meter, Second>::new(9.81)` for m/s²
//! - `Force::<Kilogram, Meter, Second>::new(98.1)` for kg⋅m/s²

use crate::core::*;
use crate::prefix::*;
use std::marker::PhantomData;

/// Trait for units that can be used in multi-unit syntax.
pub trait MultiUnit {
    /// Convert this multi-unit to its symbol representation.
    fn symbol() -> String;
    
    /// Convert this multi-unit to SI base units.
    fn to_si_factor() -> f64;
}

/// Composite unit type for two-parameter syntax (e.g., Velocity<Meter, Second>).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct DualUnit<U1, U2>(PhantomData<(U1, U2)>);

/// Composite unit type for three-parameter syntax (e.g., Force<Kilogram, Meter, Second>).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct TripleUnit<U1, U2, U3>(PhantomData<(U1, U2, U3)>);

/// Marker type for explicit division notation.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Per<U>(PhantomData<U>);

/// Marker type for exponentiation.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Exponent<U, const N: i8>(PhantomData<U>);

/// Convenient type alias for squared units.
pub type Squared = Exponent<(), 2>;

/// Convenient type alias for cubed units.
pub type Cubed = Exponent<(), 3>;

// ================================================================================================
// DUAL UNIT IMPLEMENTATIONS
// ================================================================================================

impl<U1, U2> UnitSymbol for DualUnit<U1, U2>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
{
    fn symbol() -> &'static str {
        // For now, we'll use a simple format. In a real implementation,
        // this would need more sophisticated handling.
        Box::leak(format!("{}/{}", U1::symbol(), U2::symbol()).into_boxed_str())
    }
}

impl<U1, U2> MultiUnit for DualUnit<U1, U2>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
{
    fn symbol() -> String {
        format!("{}/{}", U1::symbol(), U2::symbol())
    }
    
    fn to_si_factor() -> f64 {
        // This would need proper implementation based on the specific units
        1.0
    }
}

// ================================================================================================
// TRIPLE UNIT IMPLEMENTATIONS  
// ================================================================================================

impl<U1, U2, U3> UnitSymbol for TripleUnit<U1, U2, U3>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
    U3: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}⋅{}/{}", U1::symbol(), U2::symbol(), U3::symbol()).into_boxed_str())
    }
}

impl<U1, U2, U3> MultiUnit for TripleUnit<U1, U2, U3>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
    U3: UnitSymbol,
{
    fn symbol() -> String {
        format!("{}⋅{}/{}²", U1::symbol(), U2::symbol(), U3::symbol())
    }
    
    fn to_si_factor() -> f64 {
        1.0
    }
}

// ================================================================================================
// PER NOTATION IMPLEMENTATIONS
// ================================================================================================

impl<U> UnitSymbol for Per<U>
where
    U: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("/{}", U::symbol()).into_boxed_str())
    }
}

// ================================================================================================
// MACRO FOR DEFINING MULTI-UNIT QUANTITIES
// ================================================================================================

/// Macro for defining quantities that support multiple unit syntax forms.
///
/// This macro creates type aliases for different syntax forms of the same quantity.
#[macro_export]
macro_rules! define_multi_quantity {
    (
        $quantity_name:ident,
        dimensions: ($l:expr, $m:expr, $t:expr, $k:expr, $i:expr, $j:expr, $n:expr),
        dual_units: ($u1_name:ident, $u2_name:ident),
        triple_units: ($t1_name:ident, $t2_name:ident, $t3_name:ident)
    ) => {
        // Dual unit syntax: Quantity::<Unit1, Unit2>
        pub type $quantity_name<U1, U2> = Quantity<DualUnit<U1, U2>, $l, $m, $t, $k, $i, $j, $n>;
        
        // Triple unit syntax for Force-like quantities
        paste::paste! {
            pub type [<$quantity_name Triple>]<U1, U2, U3> = Quantity<TripleUnit<U1, U2, U3>, $l, $m, $t, $k, $i, $j, $n>;
        }
    };
}

// ================================================================================================
// TEST IMPLEMENTATIONS
// ================================================================================================

// Import necessary units for testing  
use crate::{Meter, Second, Kilogram};

// Test type aliases
pub type VelocityMulti<U1, U2> = Quantity<DualUnit<U1, U2>, 1, 0, -1, 0, 0, 0, 0>;
pub type AccelerationMulti<U1, U2> = Quantity<DualUnit<U1, U2>, 1, 0, -2, 0, 0, 0, 0>;
pub type ForceMulti<U1, U2, U3> = Quantity<TripleUnit<U1, U2, U3>, 1, 1, -2, 0, 0, 0, 0>;

// ================================================================================================
// TOSL/FROMSI IMPLEMENTATIONS
// ================================================================================================

// For now, simple implementations that convert to SI base units
impl<U1, U2> ToSI for VelocityMulti<U1, U2> {
    fn to_si(&self) -> f64 {
        // Simple conversion - would need proper unit factor calculation
        self.value
    }
}

impl<U1, U2> FromSI for VelocityMulti<U1, U2> {
    fn from_si(value: f64) -> Self {
        Self::new(value)
    }
}

impl<U1, U2> ToSI for AccelerationMulti<U1, U2> {
    fn to_si(&self) -> f64 {
        self.value
    }
}

impl<U1, U2> FromSI for AccelerationMulti<U1, U2> {
    fn from_si(value: f64) -> Self {
        Self::new(value)
    }
}

impl<U1, U2, U3> ToSI for ForceMulti<U1, U2, U3> {
    fn to_si(&self) -> f64 {
        self.value
    }
}

impl<U1, U2, U3> FromSI for ForceMulti<U1, U2, U3> {
    fn from_si(value: f64) -> Self {
        Self::new(value)
    }
}