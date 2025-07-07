//! Dimensional analysis trait system
//!
//! This module defines the core trait for dimensional analysis using the 7 SI base dimensions.
//! All physical quantities can be expressed as combinations of these base dimensions.

/// Core trait for dimensional analysis using SI base dimensions
/// 
/// Every physical unit can be expressed as a combination of the 7 SI base dimensions:
/// - L: Length (meter)
/// - M: Mass (kilogram)  
/// - T: Time (second)
/// - Θ: Temperature (kelvin)
/// - I: Electric current (ampere)
/// - J: Luminous intensity (candela)
/// - N: Amount of substance (mole)
///
/// The dimensional exponents are stored as `i8` constants, supporting exponents from -128 to 127,
/// which is more than sufficient for all known physics applications.
///
/// # Examples
///
/// ```rust
/// use physics_units::core::DimensionExtractor;
/// 
/// // Base unit: meter has dimension L¹
/// struct Meter;
/// impl DimensionExtractor for Meter {
///     const L: i8 = 1;
/// }
/// 
/// // Derived unit: newton has dimension L¹M¹T⁻²
/// struct Newton;
/// impl DimensionExtractor for Newton {
///     const L: i8 = 1;
///     const M: i8 = 1; 
///     const T: i8 = -2;
/// }
/// ```
pub trait DimensionExtractor {
    /// Length dimension exponent (meter)
    const L: i8 = 0;
    
    /// Mass dimension exponent (kilogram)
    const M: i8 = 0;
    
    /// Time dimension exponent (second)
    const T: i8 = 0;
    
    /// Temperature dimension exponent (kelvin)
    const THETA: i8 = 0;
    
    /// Electric current dimension exponent (ampere)
    const I: i8 = 0;
    
    /// Luminous intensity dimension exponent (candela)
    const J: i8 = 0;
    
    /// Amount of substance dimension exponent (mole)
    const N: i8 = 0;
}