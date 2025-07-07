//! Unit definitions
//!
//! This module contains all unit definitions organized into:
//! - Base SI units (meter, gram, second, etc.)
//! - Derived units (newton, joule, watt, etc.)  
//! - Metric prefixes (kilo, mega, milli, etc.)

pub mod base;
pub mod derived;
pub mod prefixes;

// Re-export commonly used units
pub use base::*;
pub use derived::*;
pub use prefixes::{
    Prefix, Prefixed,
    // All prefix types
    Yotta, Zetta, Exa, Peta, Tera, Giga, Mega, Kilo, Hecto, Deca,
    Deci, Centi, Milli, Micro, Nano, Pico, Femto, Atto, Zepto, Yocto,
    // Common prefixed units
    Kilogram, Kilometer, Centimeter, Millimeter, Micrometer, Nanometer,
    Milligram, Microgram, Microsecond, Nanosecond, Millisecond,
};