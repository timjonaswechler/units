//! Physical constants with type-safe units
//!
//! This module provides fundamental physical constants using our type-safe unit system.
//! All constants are defined with their proper units and dimensional analysis.

pub mod fundamental;
pub mod astronomical;
pub mod atomic;
pub mod electromagnetic;
pub mod thermodynamic;
pub mod nuclear;
pub mod mathematical;

// Re-export all constants
pub use fundamental::*;
pub use astronomical::*;
pub use atomic::*;
pub use electromagnetic::*;
pub use thermodynamic::*;
pub use nuclear::*;
pub use mathematical::*;

/// Trait for physical constants with uncertainty
pub trait PhysicalConstant<T> {
    /// Get the value of the constant
    fn value() -> T;
    
    /// Get the relative uncertainty (if known)
    fn uncertainty() -> Option<f64> { None }
    
    /// Get the year of measurement/definition
    fn year() -> Option<u16> { None }
    
    /// Get reference/source
    fn reference() -> &'static str { "CODATA 2018" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_constant_dimensional_consistency() {
        // Test that speed of light has correct dimensions
        let c = SPEED_OF_LIGHT;
        let distance = Distance::<Meter>::new(299792458.0);
        let time = Time::<Second>::new(1.0);
        let expected_speed = distance / time;
        
        // Should be dimensionally consistent
        assert!((c.value() - expected_speed.value()).abs() < 1e-6);
    }
    
    #[test]
    fn test_planck_units_consistency() {
        // Test Planck length derived from other constants
        let h_bar = REDUCED_PLANCK_CONSTANT;
        let c = SPEED_OF_LIGHT;
        let g = GRAVITATIONAL_CONSTANT;
        
        // l_p = sqrt(ℏG/c³)
        let length_factor = (h_bar * g / (c.value().powi(3))).sqrt();
        let planck_length = PLANCK_LENGTH;
        
        assert!((planck_length.value() - length_factor).abs() / length_factor < 1e-10);
    }
}