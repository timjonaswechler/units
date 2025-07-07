//! Astronomical constants
//!
//! Constants related to astronomy, astrophysics, and celestial mechanics.

use crate::core::Quantity;
use crate::units::base::*;
use crate::units::Kilogram;

/// Astronomical unit (AU)
/// 
/// The average distance from Earth to the Sun.
/// 
/// **Value**: 1.495978707×10¹¹ m (exact)
/// **Dimensions**: L¹
pub const ASTRONOMICAL_UNIT: Quantity<Meter> = Quantity::new(1.495978707e11);

/// Parsec (pc)
/// 
/// The distance at which one astronomical unit subtends an angle of one arcsecond.
/// 
/// **Value**: 3.0856775814913673×10¹⁶ m
/// **Dimensions**: L¹
pub const PARSEC: Quantity<Meter> = Quantity::new(3.0856775814913673e16);

/// Light year (ly)
/// 
/// The distance that light travels in vacuum in one Julian year.
/// 
/// **Value**: 9.4607304725808×10¹⁵ m
/// **Dimensions**: L¹
pub const LIGHT_YEAR: Quantity<Meter> = Quantity::new(9.4607304725808e15);

/// Solar mass (M☉)
/// 
/// The mass of the Sun, used as a unit for stellar masses.
/// 
/// **Value**: 1.98847×10³⁰ kg
/// **Dimensions**: M¹
pub const SOLAR_MASS: Quantity<Kilogram> = Quantity::new(1.98847e30);

/// Earth mass (M⊕)
/// 
/// **Value**: 5.9722×10²⁴ kg
/// **Dimensions**: M¹
pub const EARTH_MASS: Quantity<Kilogram> = Quantity::new(5.9722e24);

/// Jupiter mass (M♃)
/// 
/// **Value**: 1.8982×10²⁷ kg
/// **Dimensions**: M¹
pub const JUPITER_MASS: Quantity<Kilogram> = Quantity::new(1.8982e27);

/// Solar radius (R☉)
/// 
/// **Value**: 6.957×10⁸ m
/// **Dimensions**: L¹
pub const SOLAR_RADIUS: Quantity<Meter> = Quantity::new(6.957e8);

/// Earth radius (R⊕)
/// 
/// **Value**: 6.3781×10⁶ m (volumetric mean radius)
/// **Dimensions**: L¹
pub const EARTH_RADIUS: Quantity<Meter> = Quantity::new(6.3781e6);