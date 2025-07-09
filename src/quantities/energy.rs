#![allow(non_snake_case)]
//! Energy units for stellar physics and astrophysical calculations.
//!
//! This module provides energy units spanning from atomic scales to stellar energies,
//! essential for modeling stellar fusion, planetary binding energies, and particle interactions.
//!
//! # Available Units
//!
//! ## Standard Energy Units
//! - **Joule** (`J`) - SI base unit for energy
//! - **Erg** (`erg`) - CGS unit, common in astrophysics (10⁻⁷ J)
//! - **ElectronVolt** (`eV`) - Atomic and particle physics energy scale
//!
//! # Applications
//!
//! ## Stellar Physics
//! - Nuclear fusion energy release
//! - Stellar binding energies
//! - Gravitational potential energy
//!
//! ## Planetary Science
//! - Orbital kinetic energy
//! - Atmospheric escape energies
//! - Impact energies
//!
//! ## Particle Physics
//! - Ionization energies
//! - Photon energies
//! - Particle rest masses (via E=mc²)
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Stellar energy scales
//! let hydrogen_fusion = Energy::<ElectronVolt>::new(13.6); // Hydrogen ionization
//! let stellar_luminosity_per_second = Energy::<Joule>::new(3.828e26); // Sun's power output
//!
//! // Particle energies
//! let visible_photon = Energy::<ElectronVolt>::new(2.5); // ~500 nm light
//! let x_ray_photon = Energy::<ElectronVolt>::new(10000.0); // 10 keV X-ray
//!
//! // Convert between units
//! let fusion_joules = hydrogen_fusion.convert_to::<Joule>();
//! let stellar_erg = stellar_luminosity_per_second.convert_to::<Erg>();
//!
//! println!("H fusion: {}", hydrogen_fusion); // "13.6 eV"
//! println!("In Joules: {}", fusion_joules);  // "2.176e-18 J"
//! ```
//!
//! # Conversion Hierarchy
//!
//! All conversions use Joules as the hub unit:
//! - Other units → Joules → Target unit
//! - Maintains precision across the vast energy scale range

use crate::{define_quantity, define_units, features::DefaultFloat};

const JOULES_PER_ERG: DefaultFloat = 1e-7; // 1 erg = 10⁻⁷ Joules
const JOULES_PER_EV: DefaultFloat = 1.602176634e-19; // 1 eV = 1.602176634 × 10⁻¹⁹ Joules

define_quantity!(Energy, L = 2, M = 1, T = -2, THETA = 0, I = 0, J = 0, N = 0); // Mass×Length²/Time²

define_units! {
    dimension: {L = 2, M = 1, T = -2, THETA = 0, I = 0, J = 0, N = 0},
    base_unit: Joule = 1.0,
    units: {
        Erg = JOULES_PER_ERG,
        ElectronVolt = JOULES_PER_EV,
    }
}
