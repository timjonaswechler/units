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
//! let meters = Value::<Length, Meter>::new(100.0);
//! let centimeters = Value::<Length, Centimeter>::new(50.0);
//! let total = meters + centimeters;
//!
//! assert_eq!(total.get(), 100.5);
//! ```

// Module declarations
pub mod dimension;
pub mod macros;
pub mod operators;
pub mod prefix;
pub mod quantity;
pub mod scalar;
pub mod unit;
pub mod value;

// Quantity definitions
pub mod quantities;

// Re-exports for convenience
pub mod prelude {
    pub use crate::dimension::Dimension;
    pub use crate::prefix::{Giga, Kilo, Mega, Micro, Milli, Nano, Prefix};
    pub use crate::quantity::Quantity;
    pub use crate::unit::Unit;
    pub use crate::value::Value;

    // Macros
    pub use crate::{define_quantity, define_quantity_with_units, define_units};

    // Common quantities
    pub use crate::quantities::length::{Centimeter, Kilometer, Length, Meter, Millimeter};
    pub use crate::quantities::mass::{Gram, Kilogram, Mass};
    pub use crate::quantities::temperature::{
        AbsoluteTemperature, Celsius, CelsiusDelta, Fahrenheit, FahrenheitDelta, Kelvin,
        KelvinDelta, TemperatureDifference,
    };
    pub use crate::quantities::time::{Hour, Minute, Second, Time};
}
