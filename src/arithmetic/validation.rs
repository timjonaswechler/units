//! Validation system for physical quantities and dimensional composition
//!
//! This module provides the core validation traits that enable runtime checking
//! of physical operations and automatic quantity type resolution.

use crate::core::{Dimension, Quantity, UnitScale};
use crate::features::Float;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Core trait for all physical quantities
///
/// This trait provides a common interface for all physical quantities,
/// enabling automatic operation detection and validation.
pub trait PhysicalQuantity {
    type Unit: Dimension + UnitScale;
    type Value: Float;
    
    /// Get the dimensional specification for this quantity type
    fn dimensions() -> [i8; 7];
    
    /// Get the name of this quantity type  
    fn quantity_name() -> &'static str;
    
    /// Create from a base quantity
    fn from_base_quantity(quantity: Quantity<Self::Unit, Self::Value>) -> Self;
    
    /// Convert to base quantity
    fn into_base_quantity(self) -> Quantity<Self::Unit, Self::Value>;
}

/// Registry trait mapping dimensional combinations to defined physical quantities
///
/// This trait serves as a compile-time registry that maps unit type combinations
/// to their corresponding physical quantity types. It enables automatic validation
/// of operations and provides helpful error messages when quantities are undefined.
///
/// # Implementation
///
/// This trait is automatically implemented by the `define_quantity!` macro for each
/// defined physical quantity. Users should not implement this trait manually.
///
/// # Example
///
/// ```rust,ignore
/// // After defining Distance and Time quantities:
/// define_quantity!(Distance, L=1, M=0, T=0, THETA=0, I=0, J=0, N=0);
/// define_quantity!(Time, L=0, M=0, T=1, THETA=0, I=0, J=0, N=0);
/// define_quantity!(Velocity, L=1, M=0, T=-1, THETA=0, I=0, J=0, N=0);
///
/// // The system automatically knows that (Distance, Per<Time>) → Velocity
/// let distance = Distance<Meter>::new(100.0);
/// let time = Time<Second>::new(10.0);
/// let velocity = distance / time; // Works automatically
/// ```
pub trait IsDefinedQuantity<U>
where
    U: Dimension + UnitScale,
{
    /// The physical quantity type that corresponds to the dimensional combination U
    type Quantity;
}

/// Conversion trait for uniform arithmetic operations
///
/// This trait enables seamless conversion between different quantity types
/// and the underlying `Quantity<U, V>` type, allowing for uniform arithmetic
/// operations across all physical quantities.
///
/// # Implementation
///
/// This trait is automatically implemented by the `define_quantity!` macro for each
/// defined physical quantity. Users should not implement this trait manually.
///
/// # Example
///
/// ```rust,ignore
/// // Enables uniform operations like:
/// fn multiply_quantities<T1, T2>(a: T1, b: T2) -> <() as IsDefinedQuantity<...>>::Quantity
/// where
///     T1: IntoQuantity<U1, V>,
///     T2: IntoQuantity<U2, V>,
///     (): IsDefinedQuantity<(U1, U2)>,
/// {
///     // Convert to base quantities and multiply
///     let result = a.into_quantity() * b.into_quantity();
///     Self::Output::from_quantity(result)
/// }
/// ```
pub trait IntoQuantity<U, V>
where
    U: Dimension + UnitScale,
    V: Float,
{
    /// Convert this type into a base `Quantity<U, V>`
    fn into_quantity(self) -> Quantity<U, V>;
}

/// Trait for creating quantities from base `Quantity<U, V>` types
///
/// This trait is the inverse of `IntoQuantity` and enables creation of
/// concrete quantity types from the base `Quantity<U, V>` type.
///
/// # Implementation
///
/// This trait is automatically implemented by the `define_quantity!` macro for each
/// defined physical quantity. Users should not implement this trait manually.
pub trait FromQuantity<U, V>
where
    U: Dimension + UnitScale,
    V: Float,
{
    /// Create this type from a base `Quantity<U, V>`
    fn from_quantity(quantity: Quantity<U, V>) -> Self;
}

/// Helper trait for providing better error messages when quantities are undefined
///
/// This trait is used internally to generate helpful compile-time error messages
/// when attempting to use undefined quantity combinations.
pub trait QuantityUndefined<U>
where
    U: Dimension + UnitScale,
{
    /// Error message to display when this quantity combination is undefined
    const ERROR_MESSAGE: &'static str;
}

impl<U> QuantityUndefined<U> for ()
where
    U: Dimension + UnitScale,
{
    const ERROR_MESSAGE: &'static str = concat!(
        "No physical quantity defined for these dimensions.\n",
        "Consider defining it with: define_quantity!(YourQuantity, ...dimensions...);"
    );
}

/// Dimension specification as a hashable type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimensionSpec {
    pub l: i8,
    pub m: i8, 
    pub t: i8,
    pub theta: i8,
    pub i: i8,
    pub j: i8,
    pub n: i8,
}

impl DimensionSpec {
    pub const fn new(l: i8, m: i8, t: i8, theta: i8, i: i8, j: i8, n: i8) -> Self {
        Self { l, m, t, theta, i, j, n }
    }
    
    /// Add two dimension specifications (for multiplication)
    pub const fn add(self, other: Self) -> Self {
        Self {
            l: self.l + other.l,
            m: self.m + other.m,
            t: self.t + other.t,
            theta: self.theta + other.theta,
            i: self.i + other.i,
            j: self.j + other.j,
            n: self.n + other.n,
        }
    }
    
    /// Subtract dimension specifications (for division)
    pub const fn sub(self, other: Self) -> Self {
        Self {
            l: self.l - other.l,
            m: self.m - other.m,
            t: self.t - other.t,
            theta: self.theta - other.theta,
            i: self.i - other.i,
            j: self.j - other.j,
            n: self.n - other.n,
        }
    }
}

impl<U: Dimension> From<U> for DimensionSpec {
    fn from(_: U) -> Self {
        Self::new(U::L, U::M, U::T, U::THETA, U::I, U::J, U::N)
    }
}

/// Global registry of defined quantities
/// Maps dimension specifications to quantity type names
pub static QUANTITY_REGISTRY: LazyLock<HashMap<DimensionSpec, &'static str>> = LazyLock::new(|| {
    HashMap::new()
});

/// Register a quantity type in the global registry
pub fn register_quantity(dims: DimensionSpec, name: &'static str) {
    // For now, we'll use a simple approach since we can't modify static HashMap
    // In a real implementation, this would use a more sophisticated registration system
}

/// Check if a dimensional combination corresponds to a defined quantity
pub fn is_quantity_defined(dims: DimensionSpec) -> bool {
    QUANTITY_REGISTRY.contains_key(&dims)
}

/// Get the quantity name for given dimensions
pub fn get_quantity_name(dims: DimensionSpec) -> Option<&'static str> {
    QUANTITY_REGISTRY.get(&dims).copied()
}

/// Validate that an operation result corresponds to a defined quantity
pub fn validate_operation_result(left_dims: DimensionSpec, right_dims: DimensionSpec, operation: &str) -> Result<DimensionSpec, String> {
    let result_dims = match operation {
        "multiply" => left_dims.add(right_dims),
        "divide" => left_dims.sub(right_dims),
        _ => return Err(format!("Unknown operation: {}", operation)),
    };
    
    if is_quantity_defined(result_dims) {
        Ok(result_dims)
    } else {
        Err(format!(
            "Operation result with dimensions L={}, M={}, T={}, THETA={}, I={}, J={}, N={} is not a defined quantity.\n\
             Consider defining it with: define_quantity!(YourQuantity, L={}, M={}, T={}, THETA={}, I={}, J={}, N={});",
            result_dims.l, result_dims.m, result_dims.t, result_dims.theta, result_dims.i, result_dims.j, result_dims.n,
            result_dims.l, result_dims.m, result_dims.t, result_dims.theta, result_dims.i, result_dims.j, result_dims.n
        ))
    }
}

/// Macro for asserting dimensional constraints at compile time
///
/// This macro provides a cleaner syntax for asserting that a unit type
/// has specific dimensional values.
///
/// # Example
///
/// ```rust,ignore
/// assert_dimensions!(Meter, L=1, M=0, T=0, THETA=0, I=0, J=0, N=0);
/// ```
#[macro_export]
macro_rules! assert_dimensions {
    ($unit:ty, $($dim:ident = $val:expr),*) => {
        $((): $crate::core::ConstAssert<{<$unit>::$dim == $val}>)*
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Dimension;

    // Test unit for validation
    struct TestUnit;
    impl Dimension for TestUnit {
        const L: i8 = 1;
        const M: i8 = 0;
        const T: i8 = 0;
        const THETA: i8 = 0;
        const I: i8 = 0;
        const J: i8 = 0;
        const N: i8 = 0;
    }
    impl crate::core::UnitScale for TestUnit {
        const SCALE: f64 = 1.0;
    }

    #[test]
    fn test_error_message_generation() {
        // Test that the error message constant is accessible
        let _message = <() as QuantityUndefined<TestUnit>>::ERROR_MESSAGE;
        assert!(_message.contains("No physical quantity defined"));
    }
}