use crate::dimension::Dimension;

/// Trait for defining physical quantities
///
/// A quantity represents a type of physical measurement (like Length, Time, Mass, etc.)
/// Each quantity has an associated dimensional signature.
///
/// # Type Safety
///
/// The Quantity trait uses associated constants to ensure dimensional correctness
/// at compile time. This allows the type system to prevent invalid operations
/// like adding a length to a mass.
///
/// # Example
///
/// ```rust
/// use units::quantity::Quantity;
/// use units::dimension::Dimension;
///
/// #[derive(Debug, Clone, Copy)]
/// pub struct Length;
///
/// impl Quantity for Length {
///     const DIMENSION: Dimension = Dimension::length();
///     const NAME: &'static str = "Length";
/// }
/// ```
pub trait Quantity: 'static + Copy + Clone + Sized {
    /// The dimensional signature of this quantity
    const DIMENSION: Dimension;

    /// Human-readable name for this quantity (for error messages and debugging)
    const NAME: &'static str;

    /// Helper method to get dimension at runtime
    fn dimension() -> Dimension {
        Self::DIMENSION
    }

    /// Helper method to get name at runtime
    fn name() -> &'static str {
        Self::NAME
    }
}

/// Marker trait for quantities that represent temperature differences
///
/// This is used to distinguish between absolute temperatures (which cannot be added)
/// and temperature differences (which can be added).
pub trait TemperatureDifferenceQuantity: Quantity {}

/// Marker trait for quantities that represent absolute temperatures
///
/// Absolute temperatures have special addition/subtraction rules:
/// - Cannot add two absolute temperatures
/// - Can subtract absolute temperature from absolute temperature to get difference
/// - Can add/subtract difference to/from absolute temperature
pub trait AbsoluteTemperatureQuantity: Quantity {}

/// Marker trait for quantities that can be added to themselves
///
/// Most quantities can be added (Length + Length, Time + Time, etc.)
/// but absolute temperatures cannot be added together.
/// This trait enables the generic Add/Sub implementations.
pub trait CanAddSameQuantity: Quantity {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct TestQuantity;

    impl Quantity for TestQuantity {
        const DIMENSION: Dimension = Dimension::length();
        const NAME: &'static str = "TestQuantity";
    }

    #[test]
    fn test_quantity_dimension() {
        assert_eq!(TestQuantity::dimension(), Dimension::length());
    }

    #[test]
    fn test_quantity_name() {
        assert_eq!(TestQuantity::name(), "TestQuantity");
    }
}
