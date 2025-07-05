//! Arithmetic operations for quantities with the same unit and dimensions.
//!
//! This module handles the basic arithmetic operations (addition, subtraction,
//! multiplication by scalars) for quantities that have compatible dimensions.

use crate::core::*;
use std::ops::{Add, Sub, Mul, Div, Neg};

// Note: Basic same-unit operations are already implemented in core/quantity.rs
// This file is reserved for any additional same-unit arithmetic functionality
// that might be needed in the future.

// Re-export the core implementations for clarity
pub use crate::core::quantity::Quantity;

// Additional helper functions for same-unit operations could go here
// For example:
// - Minimum/maximum functions
// - Averaging functions
// - Statistical operations on collections of same-unit quantities