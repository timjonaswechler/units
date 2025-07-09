#![allow(non_snake_case)]
#![allow(dead_code)]
//! Angular velocity units for rotational motion in stellar systems.
//!
//! This module provides angular velocity units for modeling rotational properties
//! of celestial bodies, from planetary rotation to stellar spin rates.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **RadianPerSecond** (`rad/s`) - SI base unit for angular velocity
//! - **DegreePerSecond** (`°/s`) - Angular velocity in degrees per second
//!
//! # Physical Applications
//!
//! - **Planetary rotation periods** and spin rates
//! - **Stellar rotation** and differential rotation
//! - **Binary orbital motion** and synchronous rotation
//! - **Pulsar spin frequencies** and period evolution
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Earth's rotation rate
//! let earth_rotation = AngularVelocity::<DegreePerSecond>::new(0.004178); // ~15°/hour
//! println!("Earth rotation: {:.6} °/s", earth_rotation.value());
//!
//! // Convert to radians for physics calculations
//! let earth_rad_per_sec: AngularVelocity<RadianPerSecond> = earth_rotation.into();
//! println!("In rad/s: {:.2e}", earth_rad_per_sec.value());
//!
//! // Fast-spinning neutron star (pulsar)
//! let pulsar_spin = AngularVelocity::<RadianPerSecond>::new(1000.0); // 1000 rad/s
//! println!("Pulsar spin rate: {:.0} rad/s", pulsar_spin.value());
//! ```

use crate::{define_quantity, define_units};
#[cfg(feature = "f32")]
const RADIANS_PER_DEGREE: f32 = std::f32::consts::PI / 180.0;
#[cfg(feature = "f64")]
const RADIANS_PER_DEGREE: f64 = std::f64::consts::PI / 180.0;
#[cfg(feature = "f128")]
const RADIANS_PER_DEGREE: f128 = std::f128::consts::PI / 180.0;

define_quantity!(
    AngularVelocity,
    L = 0,
    M = 0,
    T = -1,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // 1/Time

// Define AngularVelocity units (angle/time)
define_units! {
    dimension: {L = 0, M = 0, T = -1, THETA = 0, I = 0, J = 0, N = 0},
        base_unit: RadianPerSecond = 1.0,
        units: {

            DegreePerSecond = RADIANS_PER_DEGREE,

    }
}
