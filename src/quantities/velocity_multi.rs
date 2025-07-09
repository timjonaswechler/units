//! Multi-unit syntax for velocity quantities.
//!
//! This module provides the new intuitive syntax for velocity:
//! - `VelocityNew::<Meter, Second>::new(10.0)` for 10 m/s

use crate::core::*;
use crate::multi_syntax::{DualUnit};
use crate::{Meter, Second}; // Import from the main module

/// Velocity quantity with multi-unit syntax.
/// 
/// Supports syntax like `VelocityNew::<Meter, Second>::new(10.0)` for 10 m/s.
/// The dimensions are automatically inferred as Length/Time.
pub type VelocityNew<U1, U2> = Quantity<DualUnit<U1, U2>, 1, 0, -1, 0, 0, 0, 0>;

/// Acceleration quantity with multi-unit syntax.
/// 
/// Supports syntax like `AccelerationNew::<Meter, Second>::new(9.81)` for 9.81 m/s².
/// The dimensions are automatically inferred as Length/Time².
pub type AccelerationNew<U1, U2> = Quantity<DualUnit<U1, U2>, 1, 0, -2, 0, 0, 0, 0>;

// Note: ToSI/FromSI implementations are in multi_syntax.rs to avoid conflicts