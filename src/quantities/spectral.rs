#![allow(non_snake_case)]
//! Spectral units for astrophysics and electromagnetic radiation analysis.
//!
//! This module defines units for wavelength, frequency, wavenumber, and related spectral quantities
//! used in astronomical observations, stellar spectroscopy, and electromagnetic radiation analysis.
//! It provides convenient conversion functions between different spectral representations.
//!
//! # Available Units
//!
//! ## Wavelength
//! - **WavelengthMeter** (`m`) - SI base unit for wavelength
//! - **Angstrom** (`Å`) - Common unit in spectroscopy (10⁻¹⁰ m)
//!
//! ## Wavenumber
//! - **PerMeter** (`m⁻¹`) - SI base unit for wavenumber
//! - **PerAngstrom** (`Å⁻¹`) - Wavenumber per Angstrom
//!
//! # Physical Applications
//!
//! - **Stellar spectroscopy** and absorption line analysis
//! - **Photometry and radiometry** across electromagnetic spectrum
//! - **Interstellar extinction** and reddening calculations
//! - **Blackbody radiation** and Wien's displacement law
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Hydrogen Lyman-alpha line
//! let lyman_alpha = Wavelength::<Angstrom>::new(1215.67);
//! println!("Lyman-α wavelength: {:.2} Å", lyman_alpha.value());
//!
//! // Convert to meters for calculations
//! let lyman_meters: Wavelength<WavelengthMeter> = lyman_alpha.into();
//! println!("Lyman-α in meters: {:.2e} m", lyman_meters.value());
//!
//! // Calculate frequency and photon energy
//! let frequency = lyman_meters.to_frequency();
//! let photon_energy = lyman_meters.to_photon_energy();
//! println!("Frequency: {:.2e} Hz", frequency);
//! println!("Photon energy: {:.2e} J", photon_energy);
//!
//! // Wavenumber calculations
//! let wavenumber = Wavenumber::<PerAngstrom>::new(8.225e-4); // 1/λ for Lyman-α
//! println!("Wavenumber: {:.2e} Å⁻¹", wavenumber.value());
//!
//! // Using helper functions for conversions
//! let freq_hz = wavelength_to_frequency(500e-9); // 500 nm green light
//! let wavelength_m = frequency_to_wavelength(freq_hz);
//! println!("Green light frequency: {:.2e} Hz", freq_hz);
//! println!("Wavelength check: {:.0} nm", wavelength_m * 1e9);
//! ```

use crate::features::DefaultFloat;
use crate::{define_quantity, define_units};

// Physical constants
const SPEED_OF_LIGHT: DefaultFloat = 299792458.0; // m/s
const PLANCK_CONSTANT: DefaultFloat = 6.62607015e-34; // J⋅s

// Wavelength is just Distance, but we define it separately for clarity
define_quantity!(
    Wavelength,
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
    base_unit: WavelengthMeter = 1.0,
    units: {
        Angstrom = 1e-10,
    }
}

// Frequency already defined in dimensions.rs, but we can add spectral-specific units
// We'll extend the existing Frequency dimension

// Wavenumber (1/Length) - common in spectroscopy
define_quantity!(
    Wavenumber,
    L = -1,
    M = 0,
    T = 0,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // 1/Length

define_units! {
    dimension: { L = -1, M = 0, T = 0, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: PerMeter = 1.0,
    units: {
        PerAngstrom = 1e10,
    }
}
