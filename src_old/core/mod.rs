//! Core infrastructure for the type-safe unit system with dimensional analysis.
//!
//! This module provides the foundational types and traits for a compile-time unit system
//! that prevents unit mixing errors and supports automatic dimensional analysis.
//!
//! # Features
//!
//! - **Type Safety**: Prevents mixing incompatible units at compile time
//! - **Hub-and-Spoke Conversions**: O(n) conversion complexity instead of O(n²)
//! - **Dimensional Analysis**: Track physical dimensions through calculations
//! - **Serialization**: Full serde support for data persistence
//!
//! # Examples
//!
//! ```rust
//! use star_sim::physics::units::*;
//!
//! // Create quantities with specific units
//! let distance = Distance::<AstronomicalUnit>::new(1.5);
//! let mass = Mass::<EarthMass>::new(0.8);
//!
//! // Convert between units
//! let distance_m = distance.convert_to::<Meter>();
//! let mass_kg = mass.convert_to::<Kilogram>();
//!
//! // Perform calculations with dimensional safety
//! let velocity = calculate_velocity(distance_m, Time::<Second>::new(3600.0));
//! ```

pub mod dimensions;
pub mod quantity;
pub mod traits;

pub use dimensions::Dimensions;
pub use quantity::{Quantity, AutoConvert};
pub use traits::{ToSI, FromSI, UnitSymbol};