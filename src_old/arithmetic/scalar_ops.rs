//! Scalar operations for quantities.
//!
//! This module provides multiplication and division operations between
//! quantities and scalar values (f64).

use crate::core::*;
use std::ops::{Mul, Div};

// Note: Basic scalar operations are already implemented in core/quantity.rs
// This file is reserved for any additional scalar operation functionality
// that might be needed in the future.

// Re-export the core implementations for clarity
pub use crate::core::quantity::Quantity;

// Additional scalar operations could be implemented here, such as:
// - Commutative scalar multiplication (f64 * Quantity)
// - Power operations (quantity^n)
// - Root operations (nth_root)

// Commutative scalar multiplication: f64 * Quantity = Quantity
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Mul<Quantity<Unit, L, M, T, K, I, J, N>> for f64
{
    type Output = Quantity<Unit, L, M, T, K, I, J, N>;

    fn mul(self, quantity: Quantity<Unit, L, M, T, K, I, J, N>) -> Self::Output {
        quantity * self
    }
}