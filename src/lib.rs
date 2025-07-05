//! # Physics Units - Type-Safe Unit System
//!
//! A comprehensive, macro-driven physics unit system with compile-time dimensional analysis.
//! 
//! ## Features
//! 
//! - **Type Safety**: Compile-time prevention of unit errors
//! - **Triple Syntax Support**: Tuple, Alias, and Prefix approaches
//! - **Dimensional Analysis**: Automatic result types from operations
//! - **Zero-Cost Abstractions**: Performance equivalent to raw f64
//! - **Extensible**: Macro-based generation for easy additions
//!
//! ## Usage Examples
//! 
//! ```rust
//! use physics_units::*;
//! 
//! // Basic units
//! let distance = Distance::<Meter>::new(100.0);
//! let time = Time::<Second>::new(10.0);
//! 
//! // Tuple syntax (maximum flexibility)
//! let velocity = Velocity::<(Meter, Second)>::new(10.0);
//! 
//! // Alias syntax (elegant)
//! let velocity = Velocity::<MeterPerSecond>::new(10.0);
//! 
//! // Prefix syntax (original flexibility)
//! let distance = Distance::<Prefixed<Kilo, Meter>>::new(1.0);
//! 
//! // Dimensional analysis
//! let result_velocity = distance / time; // Auto-inferred type
//! ```

pub mod core;
pub mod prefix;
pub mod arithmetic;
pub mod macros;
pub mod quantities;
pub mod aliases;
pub mod constants;
pub mod formatting;

// Core exports
pub use core::*;
pub use prefix::*;
pub use arithmetic::*;
pub use macros::*;

// Quantity exports
pub use quantities::*;

// Alias exports (commented out to avoid conflicts)
// pub use aliases::*;

// Constants
pub use constants::*;

// Formatting
pub use formatting::*;

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::core::*;
    pub use crate::prefix::*;
    pub use crate::quantities::*;
    // pub use crate::aliases::*;
    pub use crate::arithmetic::*;
    pub use crate::formatting::*;
}