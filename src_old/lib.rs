//! # Physics Units
//!
//! A type-safe, high-performance unit system with dimensional analysis for scientific computing.
//!
//! This crate provides a completely redesigned unit system that solves the major
//! problems of traditional approaches while maintaining full type safety and adding
//! dimensional analysis capabilities.
//!
//! ## Key Features
//!
//! - **🚀 Hub-and-Spoke Conversions**: O(n) complexity instead of O(n²)
//! - **🛡️ Compile-Time Dimensional Safety**: Prevents unit mixing errors
//! - **🏭 Macro-Generated Boilerplate**: Minimal code for new units
//! - **🎯 Astronomy-Focused**: Built specifically for stellar simulation
//! - **🔧 Variadic Syntax**: Support for flexible multi-unit syntax
//!
//! ## Quick Start
//!
//! ```rust
//! use physics_units::*;
//!
//! // Create quantities with specific units
//! let distance = Distance::<AstronomicalUnit>::new(1.5);
//! let mass = Mass::<SolarMass>::new(0.7);
//! let time = Time::<Gigayear>::new(6.0);
//!
//! // Convert between units (hub-and-spoke)
//! let distance_m = distance.convert_to::<Meter>();
//! let mass_earth = mass.convert_to::<EarthMass>();
//!
//! // Type-safe arithmetic
//! let total_distance = distance + Distance::<AstronomicalUnit>::new(0.5);
//!
//! // Display with proper symbols
//! println!("Distance: {}", distance); // "1.5 AU"
//! println!("Mass: {}", mass);         // "0.7 M☉"
//! ```
//!
//! ## Variadic Multi-Unit Syntax
//!
//! The crate also supports an experimental variadic syntax for more intuitive quantity definitions:
//!
//! ```rust
//! use physics_units::variadic::*;
//!
//! // Intuitive multi-unit syntax
//! let velocity = Velocity::<Meter, Second>::new(10.0);           // 10 m/s
//! let acceleration = Acceleration::<Meter, Second>::new(9.81);   // 9.81 m/s²
//! let force = Force::<Kilogram, Meter, Second>::new(98.1);       // 98.1 kg⋅m/s²
//! let energy = Energy::<Kilogram, Meter, Second>::new(500.0);    // 500 kg⋅m²/s²
//! ```
//!
//! ## Architecture
//!
//! The system is built around several key components:
//!
//! - **[`core`]**: Core types (`Quantity`, `Dimensions`) and traits (`ToSI`, `FromSI`)
//! - **[`constants`]**: Centralized physical constants for conversions
//! - **[`macros`]**: Code generation macros for unit systems
//! - **[`quantities`]**: Pre-defined quantity types and unit systems
//! - **[`variadic`]**: Experimental multi-unit syntax support

// Core modules - new modular structure
pub mod core;
pub mod arithmetic;
pub mod variadic;

// Legacy modules (will be phased out)
pub mod composition;
pub mod constants;
pub mod dimensions;
pub mod macros;
pub mod modular;
pub mod multi_syntax;
pub mod prefix;
pub mod quantities;
pub mod variadic_syntax;

// Re-export core functionality
pub use core::*;
pub use arithmetic::*;
pub use variadic::*;

// Legacy re-exports for compatibility
pub use composition::*;
pub use dimensions::*;
pub use modular::*;
pub use prefix::*;
pub use quantities::*;

/// Experimental variadic multi-unit syntax support.
pub mod variadic_support {
    pub use crate::variadic::*;
    pub use crate::variadic_syntax::*;
}

/// Multi-unit syntax support (dual and triple unit combinations).
pub mod multi {
    pub use crate::multi_syntax::*;
}