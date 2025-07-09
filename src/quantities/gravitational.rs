#![allow(non_snake_case)]
//! Gravitational units for celestial mechanics and orbital dynamics.
//!
//! This module defines units related to gravitational parameters (GM) and orbital mechanics,
//! which are fundamental for astronomical calculations and spacecraft trajectory analysis.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **CubicMeterPerSecondSquared** (`m³/s²`) - SI base unit for gravitational parameter
//! - **SolarGravitationalParameter** (`GM☉`) - Solar gravitational parameter (≈1.327×10²⁰ m³/s²)
//! - **EarthGravitationalParameter** (`GM⊕`) - Earth gravitational parameter (≈3.986×10¹⁴ m³/s²)
//!
//! # Physical Applications
//!
//! - **Orbital mechanics** and Kepler's laws calculations
//! - **Spacecraft trajectory** planning and analysis
//! - **Planetary system dynamics** and stability studies
//! - **Tidal force calculations** and synchronization effects
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Solar gravitational parameter
//! let solar_gm = GravitationalParameter::<SolarGravitationalParameter>::new(1.0);
//! println!("Solar GM: {:.1} GM☉", solar_gm.value());
//!
//! // Convert to SI units for orbital calculations
//! let solar_gm_si: GravitationalParameter<CubicMeterPerSecondSquared> = solar_gm.into();
//! println!("Solar GM in SI: {:.3e} m³/s²", solar_gm_si.value());
//!
//! // Earth's gravitational parameter
//! let earth_gm = GravitationalParameter::<EarthGravitationalParameter>::new(1.0);
//! println!("Earth GM: {:.1} GM⊕", earth_gm.value());
//!
//! // Using convenience type aliases
//! let solar_system = SolarGM::new(1.0);
//! let earth_system = EarthGM::new(1.0);
//! println!("Solar system GM: {} GM☉", solar_system.value());
//! println!("Earth system GM: {} GM⊕", earth_system.value());
//! ```

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Conversion constants
const SOLAR_GRAVITATIONAL_PARAMETER: DefaultFloat = 1.32712440042e20;
const EARTH_GRAVITATIONAL_PARAMETER: DefaultFloat = 3.986004418e14;

// Define gravitational parameter quantity (Length³/Time²)
define_quantity!(
    GravitationalParameter,
    L = 3,
    M = 0,
    T = -2,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Length³/Time²

define_units! {
    dimension: { L = 3, M = 0, T = -2, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: CubicMeterPerSecondSquared = 1.0,
    units: {
        SolarGravitationalParameter = SOLAR_GRAVITATIONAL_PARAMETER,
        EarthGravitationalParameter = EARTH_GRAVITATIONAL_PARAMETER,
    }
}
