//! Arithmetic operations and validation for physical quantities
//!
//! This module provides the core arithmetic system that enables automatic
//! validation and operations between physical quantities.

mod ops;
mod validation;

// Re-export the core traits and implementations
pub use validation::{
    DimensionSpec, PhysicalQuantity, 
    get_quantity_name, is_quantity_defined, register_quantity, validate_operation_result,
    FromQuantity, IntoQuantity, IsDefinedQuantity, QuantityUndefined,
};

// Operations are automatically available through trait implementations
// No need to explicitly import them - they work through the type system