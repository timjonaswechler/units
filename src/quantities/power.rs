#![allow(non_snake_case)]
//! Power units for energy transfer rates in stellar systems.
//!
//! This module provides power units for modeling energy output, luminosity,
//! and power consumption in astronomical contexts.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **Watt** (`W`) - SI base unit for power (kg⋅m²⋅s⁻³)
//! - **SolarLuminosity** (`L☉`) - Solar luminosity as reference unit (≈3.828×10²⁶ W)
//!
//! # Physical Applications
//!
//! - **Stellar luminosity** and energy output
//! - **Planetary energy budgets** and heat flow
//! - **Accretion disk power** and radiative efficiency
//! - **Tidal heating rates** in satellite systems
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Solar power output
//! let solar_power = Power::<SolarLuminosity>::new(1.0);
//! println!("Solar luminosity: {:.1} L☉", solar_power.value());
//!
//! // Convert to watts for calculations
//! let solar_watts: Power<Watt> = solar_power.into();
//! println!("Solar power: {:.3e} W", solar_watts.value());
//!
//! // Red dwarf star luminosity
//! let red_dwarf = Power::<SolarLuminosity>::new(0.1); // 10% of solar luminosity
//! println!("Red dwarf luminosity: {:.1} L☉", red_dwarf.value());
//! ```

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Conversion constants
const WATTS_PER_SOLAR_LUMINOSITY: DefaultFloat = 3.828e26;

define_quantity!(
    Power,
    L = 2,
    M = 1,
    T = -3,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Mass×Length²/Time³

define_units! {
    dimension: { L = 2, M = 1, T = -3, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: Watt = 1.0,
    units: {
        SolarLuminosity = WATTS_PER_SOLAR_LUMINOSITY,
    }
}
