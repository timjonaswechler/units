//! Core dimensional analysis system
//!
//! This module contains the fundamental types and traits for the dimensional analysis system:
//! 
//! - [`DimensionExtractor`] - Trait for extracting dimensional information from unit types
//! - [`Quantity`] - Core type representing a value with dimensional units
//! - Compositional operators for building complex units

pub mod dimension;
pub mod quantity;
pub mod composition;

// Re-export core types
pub use dimension::DimensionExtractor;
pub use quantity::Quantity;
pub use composition::{Per, Exponent, DimensionlessUnit};