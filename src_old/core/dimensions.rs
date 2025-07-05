/// Represents physical dimensions using const generics for compile-time dimensional analysis.
///
/// This type encodes the seven fundamental SI dimensions as compile-time constants,
/// enabling automatic tracking of physical dimensions through calculations.
///
/// # Dimensions
///
/// - `L`: Length (meters)
/// - `M`: Mass (kilograms)
/// - `T`: Time (seconds)
/// - `K`: Temperature (kelvin)
/// - `I`: Electric Current (amperes)
/// - `J`: Luminous Intensity (candela)
/// - `N`: Amount of Substance (moles)
///
/// # Examples
///
/// ```rust
/// // Velocity has dimensions [Length¹ Time⁻¹]
/// type VelocityDims = Dimensions<1, 0, -1, 0, 0, 0, 0>;
///
/// // Force has dimensions [Length¹ Mass¹ Time⁻²]
/// type ForceDims = Dimensions<1, 1, -2, 0, 0, 0, 0>;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions<
    const L: i8, // Length
    const M: i8, // Mass
    const T: i8, // Time
    const K: i8, // Temperature
    const I: i8, // Current
    const J: i8, // Luminous Intensity
    const N: i8, // Amount of substance
>;