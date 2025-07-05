//! Arithmetic operations module for the unit system.
//!
//! This module provides various types of arithmetic operations:
//! - Same-unit operations (addition, subtraction within the same unit)
//! - Mixed-unit operations (operations between different units of same dimension)
//! - Scalar operations (multiplication/division with plain numbers)
//! - Dimensional inference (automatic dimension calculation in multiplication/division)

pub mod same_units;
pub mod mixed_units;
pub mod scalar_ops;
pub mod dimensional_inference;

// Re-export key functionality
pub use same_units::*;
pub use mixed_units::{AddDifferentUnit, SubDifferentUnit};
pub use scalar_ops::*;
pub use dimensional_inference::*;