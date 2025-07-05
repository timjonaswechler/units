//! Variadic unit support module.
//!
//! This module provides the infrastructure for variadic unit types and their
//! conversion calculations, including advanced composition features.

pub mod unit_factors;
pub mod composition;

pub use unit_factors::*;
pub use composition::*;