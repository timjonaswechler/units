//! Macros for generating quantity types and convenience methods

/// Generate convenience constructor methods for a quantity type
/// 
/// # Example
/// 
/// ```rust
/// // After defining Length and units Meter, Kilometer...
/// impl_quantity_constructors!(Length, Meter, Kilometer, Centimeter);
/// 
/// // This generates:
/// // impl<U> Length<U> {
/// //     pub fn meter(value: f64) -> Length<Meter> { Length::new(value) }
/// //     pub fn kilometer(value: f64) -> Length<Kilometer> { Length::new(value) }
/// //     // ...
/// // }
/// ```
#[macro_export]
macro_rules! impl_quantity_constructors {
    ($quantity:ident, $($unit:ident),+ $(,)?) => {
        impl<U> $quantity<U> {
            $(
                pub fn $unit(value: f64) -> $quantity<$unit> {
                    $quantity::new(value)
                }
            )+
        }
    };
}

/// Generate type aliases for common prefixed units
/// 
/// # Example
/// 
/// ```rust
/// define_prefixed_aliases! {
///     Meter => [Kilo, Milli, Micro],
///     Gram => [Kilo, Milli],
/// }
/// 
/// // Generates:
/// // pub type Kilometer = Prefixed<Kilo, Meter>;
/// // pub type Millimeter = Prefixed<Milli, Meter>;
/// // pub type Kilogram = Prefixed<Kilo, Gram>;
/// // ...
/// ```
#[macro_export]
macro_rules! define_prefixed_aliases {
    ($($unit:ident => [$($prefix:ident),+ $(,)?]),+ $(,)?) => {
        $(
            $(
                paste::paste! {
                    pub type [<$prefix $unit>] = $crate::prefix::Prefixed<$crate::prefix::$prefix, $unit>;
                }
            )+
        )+
    };
}

/// Generate type aliases for composed units
/// 
/// # Example
/// 
/// ```rust  
/// define_composed_aliases! {
///     Velocity => {
///         MeterPerSecond = (Meter, Second),
///         KilometerPerHour = (Prefixed<Kilo, Meter>, Hour),
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_composed_aliases {
    ($quantity:ident => {
        $($alias:ident = $composition:ty),+ $(,)?
    }) => {
        $(
            pub type $alias = $composition;
        )+
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    // Mock units for testing
    crate::define_base_unit!(MockMeter, "mm", 1.0);
    crate::define_base_unit!(MockSecond, "ms", 1.0);
    
    type MockLength = Length<MockMeter>;
    
    // Test constructor generation
    crate::impl_quantity_constructors!(Length, MockMeter);

    #[test]
    fn test_quantity_constructors() {
        let length = Length::<MockMeter>::MockMeter(5.0);
        assert_eq!(length.value(), 5.0);
    }
}