use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::dimensions::Dimensions;
use super::traits::{ToSI, FromSI, UnitSymbol};

/// A physical quantity with compile-time unit and dimensional type safety.
///
/// This is the core type that represents a physical quantity (like distance, mass, time)
/// with a specific unit and dimensional information tracked at compile time.
///
/// # Type Parameters
///
/// - `Unit`: The specific unit type (e.g., `Meter`, `AstronomicalUnit`, `Kilogram`)
/// - `L, M, T, K, I, J, N`: Dimensional exponents for the seven SI base dimensions
///
/// # Examples
///
/// ```rust
/// use star_sim::physics::units::*;
///
/// // Distance in astronomical units
/// let distance: Distance<AstronomicalUnit> = Distance::new(1.5);
///
/// // Mass in earth masses  
/// let mass: Mass<EarthMass> = Mass::new(0.8);
///
/// // Convert between units
/// let distance_meters = distance.convert_to::<Meter>();
/// assert_eq!(distance_meters.value(), 1.5 * 149_597_870_700.0);
/// ```
///
/// # Dimensional Safety
///
/// The type system prevents mixing incompatible units:
///
/// ```compile_fail
/// let distance = Distance::<Meter>::new(100.0);
/// let mass = Mass::<Kilogram>::new(5.0);
/// let invalid = distance + mass; // Compile error!
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Quantity<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> {
    /// The numerical value of this quantity in the specified unit
    pub value: f64,
    /// Phantom data to track the unit type at compile time
    _unit: PhantomData<Unit>,
    /// Phantom data to track the dimensional information at compile time
    _dims: PhantomData<Dimensions<L, M, T, K, I, J, N>>,
}

impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Quantity<Unit, L, M, T, K, I, J, N>
{
    /// Create a new quantity with the specified value and unit.
    ///
    /// # Parameters
    ///
    /// - `value`: The numerical value in the specified unit
    ///
    /// # Returns
    ///
    /// A new `Quantity` with the given value and unit type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use star_sim::physics::units::*;
    ///
    /// let distance = Distance::<AstronomicalUnit>::new(1.5);
    /// let mass = Mass::<SolarMass>::new(0.7);
    /// let time = Time::<Gigayear>::new(6.0);
    /// ```
    pub fn new(value: f64) -> Self {
        Self {
            value,
            _unit: PhantomData,
            _dims: PhantomData,
        }
    }

    /// Get the numerical value of this quantity in its current unit.
    ///
    /// # Returns
    ///
    /// The numerical value as a `f64`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use star_sim::physics::units::*;
    ///
    /// let distance = Distance::<AstronomicalUnit>::new(1.5);
    /// assert_eq!(distance.value(), 1.5);
    /// ```
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Convert this quantity to a different unit of the same physical dimension.
    ///
    /// This method uses the hub-and-spoke conversion system: it converts the current
    /// quantity to SI units via `ToSI`, then creates a new quantity in the target
    /// unit via `FromSI`.
    ///
    /// # Type Parameters
    ///
    /// - `ToUnit`: The target unit type to convert to
    ///
    /// # Returns
    ///
    /// A new `Quantity` with the same physical value but expressed in the target unit.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use star_sim::physics::units::*;
    ///
    /// let distance_au = Distance::<AstronomicalUnit>::new(1.0);
    /// let distance_m = distance_au.convert_to::<Meter>();
    /// assert_eq!(distance_m.value(), 149_597_870_700.0);
    ///
    /// let mass_earth = Mass::<EarthMass>::new(1.0);
    /// let mass_kg = mass_earth.convert_to::<Kilogram>();
    /// assert_eq!(mass_kg.value(), 5.972e24);
    /// ```
    ///
    /// # Compile-Time Safety
    ///
    /// This conversion is only possible between units of the same physical dimension.
    /// Attempting to convert between incompatible dimensions will result in a compile error:
    ///
    /// ```compile_fail
    /// let distance = Distance::<Meter>::new(100.0);
    /// let invalid = distance.convert_to::<Kilogram>(); // Compile error!
    /// ```
    pub fn convert_to<ToUnit>(self) -> Quantity<ToUnit, L, M, T, K, I, J, N>
    where
        Self: ToSI,
        Quantity<ToUnit, L, M, T, K, I, J, N>: FromSI,
    {
        let si_value = self.to_si();
        Quantity::<ToUnit, L, M, T, K, I, J, N>::from_si(si_value)
    }
}

// Convenience methods specifically for dimensionless quantities
impl<Unit> Quantity<Unit, 0, 0, 0, 0, 0, 0, 0> {
    /// Convert to any dimensionless unit type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let division = distance1 / distance2;  // Dimensionless but with Meter unit type
    /// let ratio: Ratio<Unit> = division.to_dimensionless();  // Convert to proper ratio
    /// let percentage: Ratio<Percent> = division.to_dimensionless();  // Or to percentage
    /// ```
    pub fn to_dimensionless<ToUnit>(self) -> Quantity<ToUnit, 0, 0, 0, 0, 0, 0, 0> {
        Quantity::<ToUnit, 0, 0, 0, 0, 0, 0, 0>::new(self.value)
    }
}

impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Default for Quantity<Unit, L, M, T, K, I, J, N>
{
    fn default() -> Self {
        Self::new(0.0)
    }
}

// Addition (same dimensions)
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Add for Quantity<Unit, L, M, T, K, I, J, N>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

// Subtraction (same dimensions)
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Sub for Quantity<Unit, L, M, T, K, I, J, N>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

// Multiplication with scalar
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Mul<f64> for Quantity<Unit, L, M, T, K, I, J, N>
{
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self::new(self.value * scalar)
    }
}

// Division by scalar
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Div<f64> for Quantity<Unit, L, M, T, K, I, J, N>
{
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self::new(self.value / scalar)
    }
}

// Negation
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Neg for Quantity<Unit, L, M, T, K, I, J, N>
{
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.value)
    }
}

/// Trait for enabling automatic unit conversion via assignment.
///
/// This trait allows you to write:
/// ```rust
/// let distance_au = Distance::<AstronomicalUnit>::new(1.5);  
/// let distance_meters: Distance<Meter> = distance_au;
/// ```
/// 
/// This works by implementing `Into` for specific unit conversion pairs.
pub trait AutoConvert<ToQuantity> {
    fn convert(self) -> ToQuantity;
}

impl<
    FromUnit,
    ToUnit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> AutoConvert<Quantity<ToUnit, L, M, T, K, I, J, N>> for Quantity<FromUnit, L, M, T, K, I, J, N>
where
    Quantity<FromUnit, L, M, T, K, I, J, N>: ToSI,
    Quantity<ToUnit, L, M, T, K, I, J, N>: FromSI,
{
    fn convert(self) -> Quantity<ToUnit, L, M, T, K, I, J, N> {
        self.convert_to::<ToUnit>()
    }
}

// Display implementation for quantities with units
impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> Quantity<Unit, L, M, T, K, I, J, N>
where
    Unit: UnitSymbol,
{
    fn format_unit_with_dimensions() -> String {
        let base_symbol = Unit::symbol();
        
        // Handle simple cases first
        if L == 1 && M == 0 && T == 0 && K == 0 && I == 0 && J == 0 && N == 0 {
            return base_symbol.to_string();
        }
        
        // Check if this matches a predefined composite unit
        match (L, M, T, K, I, J, N) {
            (2, 0, 0, 0, 0, 0, 0) => format!("{}²", base_symbol),
            (3, 0, 0, 0, 0, 0, 0) => format!("{}³", base_symbol),
            (0, 1, 0, 0, 0, 0, 0) => base_symbol.to_string(),
            (0, 0, 1, 0, 0, 0, 0) => base_symbol.to_string(),
            (0, 0, 0, 1, 0, 0, 0) => base_symbol.to_string(),
            (0, 0, 0, 0, 1, 0, 0) => base_symbol.to_string(),
            (0, 0, 0, 0, 0, 1, 0) => base_symbol.to_string(),
            (0, 0, 0, 0, 0, 0, 1) => base_symbol.to_string(),
            _ => {
                // For complex dimensions, build the unit string
                let mut unit_parts = Vec::new();
                let mut negative_parts = Vec::new();
                
                if L != 0 {
                    let length_unit = "m";
                    if L > 0 {
                        unit_parts.push(format_dimension_part(length_unit, L));
                    } else {
                        negative_parts.push(format_dimension_part(length_unit, -L));
                    }
                }
                
                if M != 0 {
                    let mass_unit = "kg";
                    if M > 0 {
                        unit_parts.push(format_dimension_part(mass_unit, M));
                    } else {
                        negative_parts.push(format_dimension_part(mass_unit, -M));
                    }
                }
                
                if T != 0 {
                    let time_unit = "s";
                    if T > 0 {
                        unit_parts.push(format_dimension_part(time_unit, T));
                    } else {
                        negative_parts.push(format_dimension_part(time_unit, -T));
                    }
                }
                
                if K != 0 {
                    let temp_unit = "K";
                    if K > 0 {
                        unit_parts.push(format_dimension_part(temp_unit, K));
                    } else {
                        negative_parts.push(format_dimension_part(temp_unit, -K));
                    }
                }
                
                if I != 0 {
                    let current_unit = "A";
                    if I > 0 {
                        unit_parts.push(format_dimension_part(current_unit, I));
                    } else {
                        negative_parts.push(format_dimension_part(current_unit, -I));
                    }
                }
                
                if J != 0 {
                    let luminosity_unit = "cd";
                    if J > 0 {
                        unit_parts.push(format_dimension_part(luminosity_unit, J));
                    } else {
                        negative_parts.push(format_dimension_part(luminosity_unit, -J));
                    }
                }
                
                if N != 0 {
                    let substance_unit = "mol";
                    if N > 0 {
                        unit_parts.push(format_dimension_part(substance_unit, N));
                    } else {
                        negative_parts.push(format_dimension_part(substance_unit, -N));
                    }
                }
                
                let mut result = unit_parts.join("⋅");
                if !negative_parts.is_empty() {
                    if result.is_empty() {
                        result = format!("1/{}", negative_parts.join("⋅"));
                    } else {
                        result = format!("{}/{}", result, negative_parts.join("⋅"));
                    }
                }
                
                result
            }
        }
    }
}

fn format_dimension_part(unit: &str, exponent: i8) -> String {
    if exponent == 1 {
        unit.to_string()
    } else {
        match exponent {
            2 => format!("{}²", unit),
            3 => format!("{}³", unit),
            4 => format!("{}⁴", unit),
            5 => format!("{}⁵", unit),
            6 => format!("{}⁶", unit),
            7 => format!("{}⁷", unit),
            8 => format!("{}⁸", unit),
            9 => format!("{}⁹", unit),
            _ => format!("{}^{}", unit, exponent),
        }
    }
}

impl<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> fmt::Display for Quantity<Unit, L, M, T, K, I, J, N>
where
    Unit: UnitSymbol,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Check if this is a dimensionless quantity
        if L == 0 && M == 0 && T == 0 && K == 0 && I == 0 && J == 0 && N == 0 {
            // For dimensionless quantities, don't show the unit
            write!(f, "{}", self.value)
        } else {
            let unit_display = Self::format_unit_with_dimensions();
            write!(f, "{} {}", self.value, unit_display)
        }
    }
}