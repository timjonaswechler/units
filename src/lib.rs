//! Typed scalar SI values and unit-scale conversion for the star simulation.
//!
//! Quantities store one canonical SI `f64`. Units and prefixes are zero-sized
//! types used only at construction and conversion boundaries.
//!
//! With the optional `serde` feature, quantities serialize transparently as
//! their bare canonical SI `f64`. The selected data format controls non-finite
//! behavior: RON can roundtrip non-finite values, while JSON serializes them as
//! `null` and cannot deserialize that value back into a quantity. This format
//! limitation does not change the quantity API, which accepts every `f64`.
//!
//! ```
//! use units::{Kilogram, Mass, SolarMass};
//!
//! let sun = Mass::new::<SolarMass>(1.0);
//! let kilograms: f64 = sun.to::<Kilogram>();
//! let raw_si: f64 = sun.si();
//! let restored = Mass::from_si(raw_si);
//!
//! assert_eq!(restored, sun);
//! assert_eq!(kilograms, raw_si);
//! ```
//!
//! Units cannot cross quantity boundaries:
//!
//! ```compile_fail
//! use units::{Mass, Second};
//!
//! let _ = Mass::new::<Second>(1.0);
//! ```
//!
//! ```compile_fail
//! use units::{Mass, Second};
//!
//! let mass = Mass::from_si(1.0);
//! let _ = mass.to::<Second>();
//! ```
//!
//! Units must opt in to prefixes:
//!
//! ```compile_fail
//! use units::{Kilo, Mass, Prefixed, SolarMass};
//!
//! type KiloSolarMass = Prefixed<Kilo, SolarMass>;
//! let _ = Mass::new::<KiloSolarMass>(1.0);
//! ```
//!
//! Prefixes cannot be nested:
//!
//! ```compile_fail
//! use units::{Kilo, Kilogram, Mass, Prefixed};
//!
//! type KiloKilogram = Prefixed<Kilo, Kilogram>;
//! let _ = Mass::new::<KiloKilogram>(1.0);
//! ```
//!
//! Quantities intentionally have no arithmetic operators:
//!
//! ```compile_fail
//! use units::{Mass, SolarMass};
//!
//! let left = Mass::new::<SolarMass>(1.0);
//! let right = Mass::new::<SolarMass>(1.0);
//! let _ = left + right;
//! ```

#![forbid(unsafe_code)]

mod core;
mod macros;
mod prefixes;
mod quantities;

pub use core::{Prefix, PrefixableUnit, Prefixed, Unit};
pub use prefixes::*;
pub use quantities::*;
