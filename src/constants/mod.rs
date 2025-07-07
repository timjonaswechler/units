//! Physical constants
//!
//! Comprehensive collection of physical constants with proper dimensional types.
//! All constants use the new compositional unit syntax and include proper documentation
//! with dimensional analysis and usage examples.

pub mod fundamental;
pub mod gravitational;
pub mod astronomical;
pub mod atomic;

// Re-export all constants for convenience
pub use fundamental::*;
pub use gravitational::*;
pub use astronomical::*;
pub use atomic::*;