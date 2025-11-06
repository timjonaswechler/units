//! # Units - A Type-Safe Physical Units Library for Rust
//!
//! This library provides compile-time dimensional analysis and unit conversions
//! with zero runtime overhead.
//!
//! ## Features
//!
//! - **Type-safe**: Compile-time checking of dimensional correctness
//! - **Zero-cost**: All checks happen at compile time
//! - **Extensible**: Easy to add new units and quantities
//! - **Temperature-aware**: Proper handling of absolute vs. relative temperatures
//!
//! ## Example
//!
//! ```rust
//! use units::prelude::*;
//!
//! let distance = Value::<Length, Meter>::new(100.0);
//! let time = Value::<Time, Second>::new(10.0);
//! let velocity = distance / time; // Type: Value<Velocity, ...>
//! ```

// Module declarations
pub mod dimension;
pub mod quantity;
pub mod unit;
pub mod value;
pub mod prefix;
pub mod operators;

// Quantity definitions
pub mod quantities;

// Re-exports for convenience
pub mod prelude {
    pub use crate::dimension::Dimension;
    pub use crate::quantity::Quantity;
    pub use crate::unit::Unit;
    pub use crate::value::Value;
    pub use crate::prefix::Prefix;

    // Common quantities
    pub use crate::quantities::length::{Length, Meter, Kilometer, Centimeter, Millimeter};
    pub use crate::quantities::time::{Time, Second, Minute, Hour};
    pub use crate::quantities::mass::{Mass, Kilogram, Gram};
}
