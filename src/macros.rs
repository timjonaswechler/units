/// Macros for easy definition of quantities and units

/// Define a physical quantity with its dimensional signature
///
/// # Syntax
///
/// ```rust
/// use units::define_quantity;
/// use units::dimension::Dimension;
///
/// // Simple quantity
/// define_quantity!(Length, Dimension::length());
///
/// // Derived quantity
/// define_quantity!(Velocity, Dimension::length().divide(Dimension::time()));
/// ```
#[macro_export]
macro_rules! define_quantity {
    // Simple form: Name and Dimension expression
    ($name:ident, $dimension:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl $crate::quantity::Quantity for $name {
            const DIMENSION: $crate::dimension::Dimension = $dimension;
            const NAME: &'static str = stringify!($name);
        }

        impl $crate::quantity::CanAddSameQuantity for $name {}
    };

    // Alternative form: Just name (dimension must be set manually)
    ($name:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;
    };
}

/// Define units for a quantity with automatic Unit trait implementation
///
/// # Syntax
///
/// ```rust
/// use units::{define_quantity, define_units};
/// use units::dimension::Dimension;
///
/// define_quantity!(Length, Dimension::length());
///
/// define_units! {
///     quantity: Length,
///     base_unit: Meter = 1.0,
///     units: {
///         Kilometer = 1000.0,
///         Centimeter = 0.01,
///         Mile = 1609.344,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_units {
    (
        quantity: $quantity:ty,
        base_unit: $base_name:ident = $base_factor:expr,
        units: {
            $($unit_name:ident = $factor:expr),* $(,)?
        }
    ) => {
        // Define base unit
        #[derive(Debug, Clone, Copy)]
        pub struct $base_name;

        impl $crate::unit::Unit for $base_name {
            type BaseQuantity = $quantity;
            const SYMBOL: &'static str = stringify!($base_name);
            const TO_SI: f64 = $base_factor;
            const OFFSET: f64 = 0.0;
        }

        // Define other units
        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $unit_name;

            impl $crate::unit::Unit for $unit_name {
                type BaseQuantity = $quantity;
                const SYMBOL: &'static str = stringify!($unit_name);
                const TO_SI: f64 = $factor;
                const OFFSET: f64 = 0.0;
            }
        )*
    };

    // Alternative syntax without explicit quantity (for backward compatibility)
    (
        base_unit: $base_name:ident = $base_factor:expr,
        units: {
            $($unit_name:ident = $factor:expr),* $(,)?
        }
    ) => {
        // Just define the structs, user must implement Unit trait manually
        #[derive(Debug, Clone, Copy)]
        pub struct $base_name;

        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $unit_name;
        )*
    };
}

/// Convenience macro to define both quantity and units in one go
///
/// # Example
///
/// ```rust
/// use units::define_quantity_with_units;
/// use units::dimension::Dimension;
///
/// define_quantity_with_units! {
///     quantity: Velocity,
///     dimension: Dimension::length().divide(Dimension::time()),
///     base_unit: MeterPerSecond = 1.0,
///     units: {
///         KilometerPerHour = 0.277778,
///         MilesPerHour = 0.44704,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_quantity_with_units {
    (
        quantity: $quantity:ident,
        dimension: $dimension:expr,
        base_unit: $base_name:ident = $base_factor:expr,
        units: {
            $($unit_name:ident = $factor:expr),* $(,)?
        }
    ) => {
        $crate::define_quantity!($quantity, $dimension);

        $crate::define_units! {
            quantity: $quantity,
            base_unit: $base_name = $base_factor,
            units: {
                $($unit_name = $factor),*
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::dimension::Dimension;
    use crate::quantity::Quantity;
    use crate::unit::Unit;
    use crate::value::Value;

    define_quantity!(TestLength, Dimension::length());

    define_units! {
        quantity: TestLength,
        base_unit: TestMeter = 1.0,
        units: {
            TestKilometer = 1000.0,
            TestCentimeter = 0.01,
        }
    }

    #[test]
    fn test_macro_quantity() {
        assert_eq!(TestLength::DIMENSION, Dimension::length());
        assert_eq!(TestLength::NAME, "TestLength");
    }

    #[test]
    fn test_macro_units() {
        let m = Value::<TestLength, TestMeter>::new(1000.0);
        let km = m.convert::<TestKilometer>();
        assert_eq!(km.get(), 1.0);
    }

    #[test]
    fn test_combined_macro() {
        define_quantity_with_units! {
            quantity: TestVelocity,
            dimension: Dimension::length().divide(Dimension::time()),
            base_unit: TestMeterPerSecond = 1.0,
            units: {
                TestKilometerPerHour = 0.277778,
            }
        }

        let mps = Value::<TestVelocity, TestMeterPerSecond>::new(10.0);
        let kph = mps.convert::<TestKilometerPerHour>();
        assert!((kph.get() - 36.0).abs() < 0.01);
    }
}
