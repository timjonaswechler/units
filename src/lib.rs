pub(crate) mod composition;
pub(crate) mod features;
pub(crate) mod macros;
pub(crate) mod prefix;
pub(crate) mod quantities;
pub mod dimension;
pub mod core;
pub mod si_units;
pub mod si_quantities;
pub mod registry;
pub mod expression;

// Re-export proc macros from units-macros
pub use units_macros::unit;

// Re-export core types
pub use core::{Value, Quantity, Unit, Prefix, Per, Exponent, Prefixed, CompoundUnit};
pub use dimension::Dimension;

// Re-export SI units and quantities
pub use si_units::*;
pub use si_quantities::*;

// Re-export registry
pub use registry::QuantityRegistry;

pub mod prelude {
    pub use crate::composition::Exponent;
    pub use crate::composition::Per;
    pub use crate::composition::Prefixed;
    // use crate::define_prefix;
    // use crate::define_quantity;
    // use crate::define_units;
    pub use crate::prefix::*;
    pub use crate::quantities::*;
}
