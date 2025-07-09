#![allow(non_snake_case)]
#![allow(dead_code)]
//! Distance and length units for stellar system calculations.
//!
//! This module provides a comprehensive set of distance units optimized for astronomical
//! calculations, from planetary scales to galactic distances.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **Meter** (`m`) - SI base unit for length
//! - **AstronomicalUnit** (`AU`) - Earth-Sun distance (≈149.6 million km)
//! - **LightYear** (`ly`) - Distance light travels in one year
//! - **Parsec** (`pc`) - Distance at which 1 AU subtends 1 arcsecond
//!
//! ## Astronomical Object Scales
//! - **EarthRadius** (`R⊕`) - Mean radius of Earth (≈6,371 km)
//! - **SunRadius** (`R☉`) - Mean radius of the Sun (≈696,000 km)
//!
//! ## Prefixed Units
//! - **KiloParsec** (`kpc`) - 1,000 parsecs for galactic distances
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Planetary orbit at 1.5 AU from the star
//! let orbit_distance = Distance::<AstronomicalUnit>::new(1.5);
//! println!("Orbital distance: {}", orbit_distance);
//!
//! // Convert to meters for physics calculations
//! let distance_m = orbit_distance.convert_to::<Meter>();
//! println!("Distance in meters: {}", distance_m);
//!
//! // Compare planetary sizes
//! let earth_radius = Distance::<EarthRadius>::new(1.0);
//! let jupiter_radius = Distance::<EarthRadius>::new(11.2); // Jupiter ≈ 11.2 Earth radii
//!
//! // Stellar distances
//! let nearby_star = Distance::<Parsec>::new(4.24); // Proxima Centauri
//! let galactic_center = Distance::<KiloParsec>::new(8.2); // Distance to Galactic center
//! ```
//!
//! # Conversion Hierarchy
//!
//! All conversions use meters as the hub unit:
//! - Other units → Meters → Target unit
//! - Enables O(n) conversion complexity instead of O(n²)
//! - Maintains precision through IEEE 754 double precision
use crate::composition::Prefixed;
use crate::features::DefaultFloat;
use crate::prefix::Kilo;
use crate::{define_quantity, define_units};

// Conversion constants
const METERS_PER_AU: DefaultFloat = 1.495978707e11;
const METERS_PER_EARTH_RADIUS: DefaultFloat = 6.3781e6;
const METERS_PER_SUN_RADIUS: DefaultFloat = 6.96e8;
const METERS_PER_LIGHT_YEAR: DefaultFloat = 9.4607304725808e15;
const METERS_PER_PARSEC: DefaultFloat = 3.0856775814913673e16;

define_quantity!(
    Distance,
    L = 1,
    M = 0,
    T = 0,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Length

define_units! {
    dimension: { L = 1, M = 0, T = 0, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: Meter = 1.0,
    units: {
        AstronomicalUnit = METERS_PER_AU,
        EarthRadius = METERS_PER_EARTH_RADIUS,
        SunRadius = METERS_PER_SUN_RADIUS,
        LightYear = METERS_PER_LIGHT_YEAR,
        Parsec = METERS_PER_PARSEC,
    }
}

// ================================================================================================
// CONVENIENCE TYPE ALIASES FOR COMMON PREFIXED UNITS
// ================================================================================================

pub type KiloParsec = Prefixed<Kilo, Parsec>;

// ================================================================================================
// AUTOMATIC UNIT CONVERSIONS
// ================================================================================================

// Enable automatic conversions between all distance units using AutoConvert trait
// Usage: let distance_meters: Distance<Meter> = distance_au.convert();
