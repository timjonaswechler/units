//! Arithmetic operations for quantities
//!
//! This module implements automatic dimensional arithmetic operations:
//! - Multiplication creates dimensional composition
//! - Division creates dimensional ratios  
//! - Addition/Subtraction requires dimensional compatibility

pub mod ops;
pub mod conversion;

// Re-export operations (they're automatically available via traits)