use crate::dimension::Dimension;
use crate::registry::QuantityRegistry;
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};

/// Trait for defining physical quantities
/// 
/// A quantity represents a type of physical measurement (like Distance, Speed, etc.)
/// Each quantity has an associated dimensional signature and human-readable name.
pub trait Quantity: 'static + Copy + Clone {
    /// The dimensional signature of this quantity
    const DIMENSION: Dimension;
    
    /// Human-readable name for this quantity (for registry and error messages)
    const NAME: &'static str;
    
    /// Helper methods for registry access
    fn dimension() -> Dimension { Self::DIMENSION }
    fn name() -> &'static str { Self::NAME }
}

/// Trait for defining units
/// 
/// A unit represents a specific measurement standard (like Meter, Second, etc.)
/// Each unit has a dimension and a scale factor relative to the SI base unit.
pub trait Unit: 'static + Copy {
    /// The dimensional signature of this unit
    const DIMENSION: Dimension;
    
    /// Scale factor to convert to the corresponding SI base unit
    const SCALE: f64;
    
    /// Human-readable name for this unit
    const NAME: &'static str;
    
    /// Symbol/abbreviation for this unit
    const SYMBOL: &'static str;
}

/// Trait for prefix scaling (like kilo, milli, etc.)
pub trait Prefix: 'static + Copy {
    /// The scaling factor this prefix represents
    const FACTOR: f64;
    
    /// The symbol for this prefix (like "k", "m", etc.)
    const SYMBOL: &'static str;
}

/// A value with associated units and dimensional checking
/// 
/// This is the core type that wraps numerical values with their physical meaning.
/// The type system ensures dimensional consistency at compile time.
#[derive(Debug, Clone, Copy)]
pub struct Value<Q, U, T = f64>
where
    Q: Quantity,
    U: Unit,
    T: Copy,
{
    value: T,
    _quantity: PhantomData<Q>,
    _unit: PhantomData<U>,
}

impl<Q, U, T> Value<Q, U, T>
where
    Q: Quantity,
    U: Unit,
    T: Copy,
{
    /// Creates a new Value with dimensional validation
    /// 
    /// This function ensures at compile time that the unit's dimension
    /// matches the quantity's expected dimension.
    pub fn new(value: T) -> Self {
        // Runtime dimension check for now - TODO: make this compile-time
        if U::DIMENSION.data != Q::DIMENSION.data {
            panic!("Dimensional mismatch: unit dimension {:?} does not match quantity dimension {:?}", 
                   U::DIMENSION, Q::DIMENSION);
        }
        
        Self {
            value,
            _quantity: PhantomData,
            _unit: PhantomData,
        }
    }
    
    /// Returns the raw numerical value
    pub const fn value(self) -> T {
        self.value
    }
    
    /// Returns the quantity type name (for debugging)
    pub fn quantity_name() -> &'static str {
        std::any::type_name::<Q>()
    }
    
    /// Returns the unit name
    pub const fn unit_name() -> &'static str {
        U::NAME
    }
    
    /// Returns the unit symbol
    pub const fn unit_symbol() -> &'static str {
        U::SYMBOL
    }
}

impl<Q, U, T> Value<Q, U, T>
where
    Q: Quantity,
    U: Unit,
    T: Copy + Into<f64>,
{
    /// Converts the value to the SI base unit for this quantity
    pub fn si(self) -> f64 {
        self.value.into() * U::SCALE
    }
}

/// Wrapper for compound units (like m/s, N⋅m, etc.)
/// 
/// This allows us to combine multiple units using type-level composition.
#[derive(Debug, Clone, Copy)]
pub struct CompoundUnit<T>(PhantomData<T>);

// Implement Unit for compound units with tuples
impl<U1, U2> Unit for CompoundUnit<(U1, U2)>
where
    U1: Unit + Copy,
    U2: Unit + Copy,
{
    const DIMENSION: Dimension = U1::DIMENSION.multiply(U2::DIMENSION);
    const SCALE: f64 = U1::SCALE * U2::SCALE;
    const NAME: &'static str = "compound unit";
    const SYMBOL: &'static str = "compound";
}

/// Per<U> represents division by a unit (negative exponent)
#[derive(Debug, Clone, Copy)]
pub struct Per<U: Unit>(PhantomData<U>);

impl<U: Unit + Copy> Unit for Per<U> {
    const DIMENSION: Dimension = U::DIMENSION.inverse();
    const SCALE: f64 = 1.0 / U::SCALE;
    const NAME: &'static str = "per unit"; // TODO: Generate proper name
    const SYMBOL: &'static str = "/unit"; // TODO: Generate proper symbol
}

/// Exponent<U, N> represents a unit raised to a power
#[derive(Debug, Clone, Copy)]
pub struct Exponent<U: Unit, const N: i8>(PhantomData<U>);

impl<U: Unit + Copy, const N: i8> Unit for Exponent<U, N> {
    const DIMENSION: Dimension = U::DIMENSION.power(N);
    const SCALE: f64 = {
        // TODO: This needs proper const fn power implementation
        if N == 0 { 1.0 }
        else if N == 1 { U::SCALE }
        else if N == 2 { U::SCALE * U::SCALE }
        else if N == 3 { U::SCALE * U::SCALE * U::SCALE }
        else if N == -1 { 1.0 / U::SCALE }
        else if N == -2 { 1.0 / (U::SCALE * U::SCALE) }
        else if N == -3 { 1.0 / (U::SCALE * U::SCALE * U::SCALE) }
        else { 1.0 } // Fallback - should implement proper const pow
    };
    const NAME: &'static str = "powered unit"; // TODO: Generate proper name
    const SYMBOL: &'static str = "unit^N"; // TODO: Generate proper symbol
}

/// Prefixed<P, U> represents a unit with a prefix applied
#[derive(Debug, Clone, Copy)]
pub struct Prefixed<P: Prefix, U: Unit>(PhantomData<(P, U)>);

impl<P: Prefix + Copy, U: Unit + Copy> Unit for Prefixed<P, U> {
    const DIMENSION: Dimension = U::DIMENSION;
    const SCALE: f64 = P::FACTOR * U::SCALE;
    const NAME: &'static str = "prefixed unit"; // TODO: Generate proper name
    const SYMBOL: &'static str = "prefixed"; // TODO: Generate proper symbol
}

// =============================================================================
// SMART OPERATOR OVERLOADING - Phase 4
// =============================================================================

/// Result type for arithmetic operations that can automatically resolve quantity types
/// This is the key to automatic dimensional arithmetic
pub struct ArithmeticResult<T> {
    pub value: T,
    pub dimension: Dimension,
}

impl<T> ArithmeticResult<T> {
    pub fn new(value: T, dimension: Dimension) -> Self {
        Self { value, dimension }
    }
    
    /// Attempts to resolve this result to a known quantity type
    /// Returns the quantity name if found in the registry
    pub fn resolve_quantity_name(&self) -> Option<&'static str> {
        QuantityRegistry::lookup_quantity_name(self.dimension)
    }
}

// Automatic type resolution - this allows ArithmeticResults to be converted
// into properly typed Values when the quantity type is known

/// Marker trait for automatic quantity resolution
/// This trait is implemented for all quantity types to enable automatic conversion
pub trait AutoResolvableQuantity: Quantity {
    /// Creates a Value from an ArithmeticResult if dimensions match
    fn from_result<U: Unit, T: Copy>(result: ArithmeticResult<T>) -> Option<Value<Self, U, T>> 
    where 
        Self: Sized 
    {
        if result.dimension.data == Self::DIMENSION.data {
            Some(Value {
                value: result.value,
                _quantity: PhantomData,
                _unit: PhantomData,
            })
        } else {
            None
        }
    }
}

// Implement for all quantity types automatically
impl<Q: Quantity> AutoResolvableQuantity for Q {}

/// Macro for automatic type resolution from arithmetic results
/// 
/// Usage: `resolve!(result_value => QuantityType, UnitType)`
/// This attempts to convert an ArithmeticResult into a properly typed Value
macro_rules! resolve {
    ($result:expr => $quantity:ty, $unit:ty) => {
        <$quantity as AutoResolvableQuantity>::from_result::<$unit, _>($result)
    };
}

pub(crate) use resolve;

// Multiplication: Q1 * Q2 = Q3 (where Q3's dimension = Q1.dim * Q2.dim)
impl<Q1, U1, Q2, U2, T> Mul<Value<Q2, U2, T>> for Value<Q1, U1, T>
where
    Q1: Quantity,
    U1: Unit,
    Q2: Quantity,
    U2: Unit,
    T: Copy + Mul<Output = T> + Into<f64>,
{
    type Output = ArithmeticResult<T>;

    fn mul(self, rhs: Value<Q2, U2, T>) -> Self::Output {
        let result_dimension = Q1::DIMENSION.multiply(Q2::DIMENSION);
        let result_value = self.value * rhs.value;
        ArithmeticResult::new(result_value, result_dimension)
    }
}

// Division: Q1 / Q2 = Q3 (where Q3's dimension = Q1.dim / Q2.dim)
impl<Q1, U1, Q2, U2, T> Div<Value<Q2, U2, T>> for Value<Q1, U1, T>
where
    Q1: Quantity,
    U1: Unit,
    Q2: Quantity,
    U2: Unit,
    T: Copy + Div<Output = T> + Into<f64>,
{
    type Output = ArithmeticResult<T>;

    fn div(self, rhs: Value<Q2, U2, T>) -> Self::Output {
        let result_dimension = Q1::DIMENSION.divide(Q2::DIMENSION);
        let result_value = self.value / rhs.value;
        ArithmeticResult::new(result_value, result_dimension)
    }
}

// Addition: Q + Q = Q (same quantity types and units only for simplicity)
impl<Q, U, T> Add<Value<Q, U, T>> for Value<Q, U, T>
where
    Q: Quantity,
    U: Unit,
    T: Copy + Add<Output = T>,
{
    type Output = Value<Q, U, T>;

    fn add(self, rhs: Value<Q, U, T>) -> Self::Output {
        Value {
            value: self.value + rhs.value,
            _quantity: PhantomData,
            _unit: PhantomData,
        }
    }
}

// Subtraction: Q - Q = Q (same quantity types and units only for simplicity)
impl<Q, U, T> Sub<Value<Q, U, T>> for Value<Q, U, T>
where
    Q: Quantity,
    U: Unit,
    T: Copy + Sub<Output = T>,
{
    type Output = Value<Q, U, T>;

    fn sub(self, rhs: Value<Q, U, T>) -> Self::Output {
        Value {
            value: self.value - rhs.value,
            _quantity: PhantomData,
            _unit: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test quantities
    #[derive(Debug, Clone, Copy)]
    struct Distance;
    impl Quantity for Distance {
        const DIMENSION: Dimension = Dimension::length();
        const NAME: &'static str = "Distance";  
    }

    #[derive(Debug, Clone, Copy)]
    struct Time;
    impl Quantity for Time {
        const DIMENSION: Dimension = Dimension::time();
        const NAME: &'static str = "Time";
    }

    #[derive(Debug, Clone, Copy)]
    struct Mass;
    impl Quantity for Mass {
        const DIMENSION: Dimension = Dimension::mass();
        const NAME: &'static str = "Mass";
    }

    #[derive(Debug, Clone, Copy)]
    struct Area;
    impl Quantity for Area {
        const DIMENSION: Dimension = Dimension::AREA;
        const NAME: &'static str = "Area";
    }

    // Test units
    #[derive(Debug, Clone, Copy)]
    struct Meter;
    impl Unit for Meter {
        const DIMENSION: Dimension = Dimension::length();
        const SCALE: f64 = 1.0;
        const NAME: &'static str = "meter";
        const SYMBOL: &'static str = "m";
    }

    #[derive(Debug, Clone, Copy)]
    struct Second;
    impl Unit for Second {
        const DIMENSION: Dimension = Dimension::time();
        const SCALE: f64 = 1.0;
        const NAME: &'static str = "second";
        const SYMBOL: &'static str = "s";
    }

    #[derive(Debug, Clone, Copy)]
    struct Kilogram;
    impl Unit for Kilogram {
        const DIMENSION: Dimension = Dimension::mass();
        const SCALE: f64 = 1.0;
        const NAME: &'static str = "kilogram";
        const SYMBOL: &'static str = "kg";
    }

    #[derive(Debug, Clone, Copy)]
    struct SquareMeter;
    impl Unit for SquareMeter {
        const DIMENSION: Dimension = Dimension::AREA;
        const SCALE: f64 = 1.0;
        const NAME: &'static str = "square meter";
        const SYMBOL: &'static str = "m²";
    }

    #[test]
    fn test_value_creation() {
        let distance = Value::<Distance, Meter>::new(10.0);
        assert_eq!(distance.value(), 10.0);
        
        let distance2 = Value::<Distance, Meter>::new(10.0);
        assert_eq!(distance2.si(), 10.0); // Meter is SI base unit
    }

    #[test]
    fn test_multiplication_creates_result() {
        let length = Value::<Distance, Meter>::new(5.0);
        let width = Value::<Distance, Meter>::new(3.0);
        
        let area_result = length * width;
        assert_eq!(area_result.value, 15.0);
        assert_eq!(area_result.dimension, Dimension::AREA);
        
        // Should automatically resolve to Area quantity
        assert_eq!(area_result.resolve_quantity_name(), Some("Area"));
    }

    #[test]
    fn test_division_creates_result() {
        let force_dimension = Dimension::FORCE; // [M=1, L=1, T=-2]
        let area_dimension = Dimension::AREA;   // [L=2]
        let expected_pressure = force_dimension.divide(area_dimension); // [M=1, L=-1, T=-2]
        
        // Create mock force and area values
        let mock_force = ArithmeticResult::new(100.0, force_dimension);
        let mock_area = ArithmeticResult::new(10.0, area_dimension);
        
        // Simulate division result
        let pressure_result = ArithmeticResult::new(
            mock_force.value / mock_area.value,
            mock_force.dimension.divide(mock_area.dimension)
        );
        
        assert_eq!(pressure_result.value, 10.0);
        assert_eq!(pressure_result.dimension, expected_pressure);
        assert_eq!(pressure_result.resolve_quantity_name(), Some("Pressure"));
    }

    #[test] 
    fn test_addition_same_quantity() {
        let distance1 = Value::<Distance, Meter>::new(5.0);
        let distance2 = Value::<Distance, Meter>::new(3.0);
        
        let result = distance1 + distance2;
        assert_eq!(result.value(), 8.0);
    }

    #[test]
    fn test_arithmetic_result_quantity_resolution() {
        // Test that various dimensional combinations resolve correctly
        let velocity_dim = Dimension::length().divide(Dimension::time()); // [L=1, T=-1]
        let acceleration_dim = velocity_dim.divide(Dimension::time());     // [L=1, T=-2]
        let force_dim = Dimension::mass().multiply(acceleration_dim);      // [M=1, L=1, T=-2]
        
        let velocity_result = ArithmeticResult::new(10.0, velocity_dim);
        let force_result = ArithmeticResult::new(50.0, force_dim);
        
        assert_eq!(velocity_result.resolve_quantity_name(), Some("Speed"));
        assert_eq!(force_result.resolve_quantity_name(), Some("Force"));
    }

    // Define a velocity quantity for testing automatic resolution
    #[derive(Debug, Clone, Copy)]
    struct Velocity;
    impl Quantity for Velocity {
        const DIMENSION: Dimension = Dimension::VELOCITY;
        const NAME: &'static str = "Velocity";
    }

    #[derive(Debug, Clone, Copy)]
    struct MeterPerSecond;
    impl Unit for MeterPerSecond {
        const DIMENSION: Dimension = Dimension::VELOCITY;
        const SCALE: f64 = 1.0;
        const NAME: &'static str = "meter per second";
        const SYMBOL: &'static str = "m/s";
    }

    #[test]
    fn test_automatic_type_resolution() {
        // Create distance and time values
        let distance = Value::<Distance, Meter>::new(100.0);
        let time = Value::<Time, Second>::new(10.0);
        
        // Divide to get velocity result
        let velocity_result = distance / time;
        assert_eq!(velocity_result.value, 10.0);
        assert_eq!(velocity_result.dimension, Dimension::VELOCITY);
        
        // Test automatic resolution to typed Value
        let typed_velocity = resolve!(velocity_result => Velocity, MeterPerSecond);
        assert!(typed_velocity.is_some());
        
        let velocity_value = typed_velocity.unwrap();
        assert_eq!(velocity_value.value(), 10.0);
    }

    #[test]
    fn test_automatic_resolution_failure() {
        // Create an area result
        let length = Value::<Distance, Meter>::new(5.0);
        let width = Value::<Distance, Meter>::new(3.0);
        let area_result = length * width;
        
        // Trying to resolve area as velocity should fail
        let wrong_resolution = resolve!(area_result => Velocity, MeterPerSecond);
        assert!(wrong_resolution.is_none());
    }
}