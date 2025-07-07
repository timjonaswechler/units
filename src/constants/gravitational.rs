//! Gravitational constants
//!
//! Constants related to gravitation and celestial mechanics.

use crate::core::Quantity;
use crate::core::composition::{Per, Exponent};
use crate::units::base::*;
use crate::units::Kilogram;

/// Gravitational constant (G)
/// 
/// The fundamental constant of gravity, relating gravitational force to mass and distance.
/// 
/// **Value**: 6.67430×10⁻¹¹ m³⋅kg⁻¹⋅s⁻²
/// **Dimensions**: L³M⁻¹T⁻² 
/// 
/// # Usage in Physics
/// - Newton's law of gravitation: F = Gm₁m₂/r²
/// - Schwarzschild radius: rs = 2GM/c²
/// - Orbital mechanics
pub const GRAVITATIONAL_CONSTANT: Quantity<(Exponent<Meter, 3>, Per<Kilogram>, Per<Exponent<Second, 2>>)> = 
    Quantity::new(6.67430e-11);

/// Standard gravity (g)
/// 
/// The nominal gravitational acceleration at Earth's surface.
/// 
/// **Value**: 9.80665 m/s² (conventional standard value)
/// **Dimensions**: L¹T⁻²
pub const STANDARD_GRAVITY: Quantity<(Meter, Per<Exponent<Second, 2>>)> = Quantity::new(9.80665);