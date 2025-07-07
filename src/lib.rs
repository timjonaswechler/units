//! # Physics Units Library
//!
//! A type-safe physics units library with dimensional analysis for scientific computing.
//! 
//! ## Features
//! 
//! - **Dimensional Safety**: Compile-time checking prevents unit conversion errors
//! - **Intuitive Syntax**: `Velocity<(Meter, Per<Second>)>` matches physics notation  
//! - **Zero-Cost Abstractions**: Runtime performance identical to raw floating point
//! - **Complete Flexibility**: Support for any unit combination needed in physics
//! - **Domain Optimization**: Configurable precision and value types via features
//! 
//! ## Quick Start
//! 
//! ```rust
//! use physics_units::prelude::*;
//! 
//! // Create quantities with proper units
//! let distance = Distance::new(10.0, Meter);
//! let time = Time::new(2.0, Second);
//! 
//! // Automatic dimensional arithmetic
//! let velocity = distance / time;  // Type: Quantity<(Meter, Per<Second>)>
//! 
//! // Physics constants with proper dimensions
//! let energy = PLANCK_CONSTANT * frequency;  // E = hν
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]

use static_assertions::*;
use std::mem;

// Feature flag validation - prevent conflicting configurations
#[cfg(all(feature = "f32", feature = "f64"))]
compile_error!("Cannot enable both f32 and f64 features");

#[cfg(all(feature = "f32", feature = "f128"))]
compile_error!("Cannot enable both f32 and f128 features");

#[cfg(all(feature = "f64", feature = "f128"))]
compile_error!("Cannot enable both f64 and f128 features");

#[cfg(not(any(feature = "f32", feature = "f64", feature = "f128")))]
compile_error!("Must enable one of: f32, f64, f128");

// Value type selection based on features
#[cfg(feature = "f64")]
/// Default floating point type for quantities
pub type DefaultFloat = f64;

#[cfg(feature = "f32")]
/// Default floating point type for quantities
pub type DefaultFloat = f32;

#[cfg(feature = "f128")]
/// Default floating point type for quantities
pub type DefaultFloat = f128;

// Precision configuration based on features
#[cfg(feature = "precision-3")]
/// Default precision for display formatting
pub const DEFAULT_PRECISION: usize = 3;

#[cfg(feature = "precision-6")]
/// Default precision for display formatting
pub const DEFAULT_PRECISION: usize = 6;

#[cfg(feature = "precision-9")]
/// Default precision for display formatting
pub const DEFAULT_PRECISION: usize = 9;

#[cfg(feature = "precision-12")]
/// Default precision for display formatting
pub const DEFAULT_PRECISION: usize = 12;

// If no precision feature is enabled, default to 6
#[cfg(not(any(feature = "precision-3", feature = "precision-6", feature = "precision-9", feature = "precision-12")))]
/// Default precision for display formatting
pub const DEFAULT_PRECISION: usize = 6;

// Static assertions to validate configuration at compile time
const_assert!(DEFAULT_PRECISION <= 15); // Reasonable upper bound for precision
const_assert!(DEFAULT_PRECISION >= 1);  // Minimum precision

// Validate that exactly one value type is selected
#[cfg(feature = "f32")]
const_assert_eq!(mem::size_of::<DefaultFloat>(), 4);

#[cfg(feature = "f64")]
const_assert_eq!(mem::size_of::<DefaultFloat>(), 8);

#[cfg(feature = "f128")]
const_assert_eq!(mem::size_of::<DefaultFloat>(), 16);

// Core modules
pub mod core;
pub mod units;
pub mod arithmetic;
pub mod formatting;
pub mod constants;

// Re-exports for convenience
pub use core::*;

/// Common imports for typical usage
pub mod prelude {
    pub use crate::core::{Quantity, DimensionExtractor};
    pub use crate::units::base::*;
    pub use crate::units::derived::*;
    pub use crate::constants::fundamental::*;
    pub use crate::DefaultFloat;
}