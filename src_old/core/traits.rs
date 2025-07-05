/// Trait for converting quantities to their equivalent value in SI base units.
///
/// This trait enables the hub-and-spoke conversion system where all unit conversions
/// go through SI base units as an intermediate step, reducing conversion complexity
/// from O(n²) to O(n).
///
/// # Implementation
///
/// For each unit type, implement this trait to specify how to convert to the
/// corresponding SI base unit (meters, kilograms, seconds, etc.).
///
/// # Examples
///
/// ```rust
/// impl ToSI for Distance<AstronomicalUnit> {
///     fn to_si(&self) -> f64 {
///         self.value * 149_597_870_700.0  // Convert AU to meters
///     }
/// }
/// ```
pub trait ToSI {
    /// Convert this quantity to its equivalent value in SI base units.
    ///
    /// # Returns
    ///
    /// The numerical value in the appropriate SI base unit (meters for distance,
    /// kilograms for mass, seconds for time, etc.).
    fn to_si(&self) -> f64;
}

/// Trait for creating quantities from values in SI base units.
///
/// This is the inverse of `ToSI` and completes the hub-and-spoke conversion system.
/// It allows creating a quantity of a specific unit from a value in SI base units.
///
/// # Examples
///
/// ```rust
/// impl FromSI for Distance<AstronomicalUnit> {
///     fn from_si(meters: f64) -> Self {
///         Self::new(meters / 149_597_870_700.0)  // Convert meters to AU
///     }
/// }
/// ```
pub trait FromSI: Sized {
    /// Create a new quantity from a value in SI base units.
    ///
    /// # Parameters
    ///
    /// - `value`: The numerical value in the appropriate SI base unit
    ///
    /// # Returns
    ///
    /// A new quantity with the value converted to this unit type.
    fn from_si(value: f64) -> Self;
}

/// Trait for providing human-readable unit symbols.
///
/// This trait allows quantities to display themselves with appropriate unit symbols
/// when formatted. Supports Unicode symbols for astronomical units.
///
/// # Examples
///
/// ```rust
/// impl UnitSymbol for AstronomicalUnit {
///     fn symbol() -> &'static str {
///         "AU"
///     }
/// }
///
/// impl UnitSymbol for EarthMass {
///     fn symbol() -> &'static str {
///         "M⊕"  // Unicode symbol for Earth mass
///     }
/// }
/// ```
pub trait UnitSymbol {
    /// Returns the standard symbol for this unit.
    ///
    /// # Returns
    ///
    /// A string slice containing the unit symbol (e.g., "m", "kg", "AU", "M☉").
    fn symbol() -> &'static str;
}