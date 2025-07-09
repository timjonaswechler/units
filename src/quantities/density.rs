#![allow(non_snake_case)]
#![allow(dead_code)]
//! Density units for material properties in stellar systems.
//!
//! This module provides density units essential for calculating mass distributions,
//! material properties, and structural characteristics of celestial bodies.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **KilogramPerCubicMeter** (`kg/m³`) - SI base unit for density
//! - **GramPerCubicCentimeter** (`g/cm³`) - Common laboratory density unit
//!
//! # Physical Applications
//!
//! - **Planetary interior modeling** and composition analysis
//! - **Stellar structure calculations** and equation of state
//! - **Interstellar medium** and nebular densities
//! - **Asteroid and comet** bulk density determination
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Water density (Earth reference)
//! let water_density = Density::<GramPerCubicCentimeter>::new(1.0);
//! println!("Water density: {:.1} g/cm³", water_density.value());
//!
//! // Convert to SI units for calculations
//! let water_si: Density<KilogramPerCubicMeter> = water_density.into();
//! println!("Water in SI: {:.0} kg/m³", water_si.value());
//!
//! // Neutron star density (extremely high)
//! let neutron_star_density = Density::<KilogramPerCubicMeter>::new(5e17);
//! println!("Neutron star density: {:.2e} kg/m³", neutron_star_density.value());
//! ```

use crate::{define_quantity, define_units};

define_quantity!(
    Density,
    L = -3,
    M = 1,
    T = 0,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Mass/Length³

// Define Density units (Mass/Length³)
define_units! {
    dimension :{L = -3, M = 1, T = 0, THETA = 0, I = 0, J = 0, N = 0},
    base_unit: KilogramPerCubicMeter = 1.0,
    units: {
        GramPerCubicCentimeter = 1000.0,
    }
}
