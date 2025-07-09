// Dimensional trait with associated constants for automatic composition
pub trait Dimension {
    const L: i8;      // Length (m)
    const M: i8;      // Mass (kg)
    const T: i8;      // Time (s)
    const THETA: i8;  // Temperature (K)
    const I: i8;      // Current (A)
    const J: i8;      // Luminous Intensity (cd)
    const N: i8;      // Amount of Substance (mol)
}
