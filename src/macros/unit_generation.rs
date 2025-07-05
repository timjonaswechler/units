//! Macros for generating base units

/// Generate a base unit with UnitComposition implementation
/// 
/// # Example
/// 
/// ```rust
/// define_base_unit!(Meter, "m", 1.0);
/// define_base_unit!(Kilometer, "km", 1000.0);
/// ```
#[macro_export]
macro_rules! define_base_unit {
    ($name:ident, $symbol:expr, $si_factor:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name;
        
        impl $crate::core::UnitComposition for $name {
            #[inline]
            fn to_si_factor() -> f64 { 
                $si_factor 
            }
            
            #[inline]
            fn from_si_factor() -> f64 { 
                1.0 / $si_factor 
            }
            
            fn symbol() -> String { 
                $symbol.to_string() 
            }
        }
    };
}

/// Generate multiple base units for a dimension
/// 
/// # Example
/// 
/// ```rust
/// define_units_for_dimension! {
///     Length => {
///         Meter = "m", 1.0,
///         Kilometer = "km", 1000.0,
///         Centimeter = "cm", 0.01,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_units_for_dimension {
    ($dimension:ident => {
        $($unit:ident = $symbol:expr, $si_factor:expr),+ $(,)?
    }) => {
        $(
            $crate::define_base_unit!($unit, $symbol, $si_factor);
        )+
    };
}

/// Generate composed unit aliases  
/// 
/// # Example
/// 
/// ```rust
/// define_composed_unit!(MeterPerSecond, "m/s", 1.0);
/// ```
#[macro_export]
macro_rules! define_composed_unit {
    ($name:ident, $symbol:expr, $si_factor:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name;
        
        impl $crate::core::UnitComposition for $name {
            #[inline]
            fn to_si_factor() -> f64 { 
                $si_factor 
            }
            
            #[inline]
            fn from_si_factor() -> f64 { 
                1.0 / $si_factor 
            }
            
            fn symbol() -> String { 
                $symbol.to_string() 
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    define_base_unit!(TestMeter, "tm", 1.0);
    define_base_unit!(TestKilometer, "tkm", 1000.0);
    
    define_composed_unit!(TestMeterPerSecond, "tm/s", 1.0);

    #[test]
    fn test_base_unit_generation() {
        use crate::core::UnitComposition;
        
        assert_eq!(TestMeter::to_si_factor(), 1.0);
        assert_eq!(TestMeter::symbol(), "tm");
        
        assert_eq!(TestKilometer::to_si_factor(), 1000.0);
        assert_eq!(TestKilometer::symbol(), "tkm");
    }

    #[test]
    fn test_composed_unit_generation() {
        use crate::core::UnitComposition;
        
        assert_eq!(TestMeterPerSecond::to_si_factor(), 1.0);
        assert_eq!(TestMeterPerSecond::symbol(), "tm/s");
    }
}