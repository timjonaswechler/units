//! Unit conversion implementation
//!
//! Provides safe, compile-time checked unit conversions.

use crate::core::{DimensionExtractor, Quantity};
use crate::units::prefixes::{Prefix, Prefixed};
use crate::DefaultFloat;
use std::marker::PhantomData;

impl<U, V> Quantity<U, V>
where
    U: DimensionExtractor,
    V: Copy + From<f64> + std::ops::Mul<f64, Output = V> + std::ops::Div<f64, Output = V>,
{
    /// Convert to a prefixed version of the same unit
    pub fn to_prefixed<P: Prefix>(self) -> Quantity<Prefixed<P, U>, V> {
        Quantity {
            value: self.value / P::FACTOR,
            _phantom: PhantomData,
        }
    }
    
    /// Convert from a prefixed unit to the base unit
    pub fn from_prefixed<P: Prefix>(prefixed: Quantity<Prefixed<P, U>, V>) -> Self {
        Quantity {
            value: prefixed.value * P::FACTOR,
            _phantom: PhantomData,
        }
    }
}

impl<P, U, V> Quantity<Prefixed<P, U>, V>
where
    P: Prefix,
    U: DimensionExtractor,
    V: Copy + From<f64> + std::ops::Mul<f64, Output = V>,
{
    /// Convert prefixed unit back to base unit
    pub fn to_base_unit(self) -> Quantity<U, V> {
        Quantity {
            value: self.value * P::FACTOR,
            _phantom: PhantomData,
        }
    }
    
    /// Convert between different prefixes of the same unit
    pub fn to_different_prefix<P2: Prefix>(self) -> Quantity<Prefixed<P2, U>, V> {
        // Convert to base unit first, then to target prefix
        let base = self.to_base_unit();
        base.to_prefixed::<P2>()
    }
}

/// Trait for units that can be automatically scaled for optimal display
pub trait AutoScale: DimensionExtractor {
    /// Choose the best prefix for displaying this value
    fn best_prefix_for_value(value: f64) -> &'static str;
    
    /// Convert to the best prefixed unit for display
    fn auto_scale<V>(quantity: Quantity<Self, V>) -> (f64, &'static str)
    where
        V: Copy + Into<f64>;
}

// Implement AutoScale for common units
use crate::units::base::*;

impl AutoScale for Meter {
    fn best_prefix_for_value(value: f64) -> &'static str {
        let abs_value = value.abs();
        
        if abs_value >= 1e9 {
            "Gm" // Gigameter (for astronomical scales)
        } else if abs_value >= 1e6 {
            "Mm" // Megameter  
        } else if abs_value >= 1e3 {
            "km" // Kilometer
        } else if abs_value >= 1.0 {
            "m"  // Meter
        } else if abs_value >= 1e-2 {
            "cm" // Centimeter
        } else if abs_value >= 1e-3 {
            "mm" // Millimeter
        } else if abs_value >= 1e-6 {
            "μm" // Micrometer
        } else if abs_value >= 1e-9 {
            "nm" // Nanometer
        } else {
            "pm" // Picometer
        }
    }
    
    fn auto_scale<V>(quantity: Quantity<Self, V>) -> (f64, &'static str)
    where
        V: Copy + Into<f64>
    {
        let value: f64 = quantity.value.into();
        let prefix = Self::best_prefix_for_value(value);
        
        let scaled_value = match prefix {
            "Gm" => value / 1e9,
            "Mm" => value / 1e6,
            "km" => value / 1e3,
            "m" => value,
            "cm" => value / 1e-2,
            "mm" => value / 1e-3,
            "μm" => value / 1e-6,
            "nm" => value / 1e-9,
            "pm" => value / 1e-12,
            _ => value,
        };
        
        (scaled_value, prefix)
    }
}

impl AutoScale for Gram {
    fn best_prefix_for_value(value: f64) -> &'static str {
        let abs_value = value.abs();
        
        if abs_value >= 1e6 {
            "Mg" // Megagram (tonne)
        } else if abs_value >= 1e3 {
            "kg" // Kilogram
        } else if abs_value >= 1.0 {
            "g"  // Gram
        } else if abs_value >= 1e-3 {
            "mg" // Milligram
        } else if abs_value >= 1e-6 {
            "μg" // Microgram
        } else {
            "ng" // Nanogram
        }
    }
    
    fn auto_scale<V>(quantity: Quantity<Self, V>) -> (f64, &'static str)
    where
        V: Copy + Into<f64>
    {
        let value: f64 = quantity.value.into();
        let prefix = Self::best_prefix_for_value(value);
        
        let scaled_value = match prefix {
            "Mg" => value / 1e6,
            "kg" => value / 1e3,
            "g" => value,
            "mg" => value / 1e-3,
            "μg" => value / 1e-6,
            "ng" => value / 1e-9,
            _ => value,
        };
        
        (scaled_value, prefix)
    }
}

impl AutoScale for Second {
    fn best_prefix_for_value(value: f64) -> &'static str {
        let abs_value = value.abs();
        
        if abs_value >= 3.15e7 {
            "yr" // Year (approximately)
        } else if abs_value >= 86400.0 {
            "d"  // Day  
        } else if abs_value >= 3600.0 {
            "h"  // Hour
        } else if abs_value >= 60.0 {
            "min" // Minute
        } else if abs_value >= 1.0 {
            "s"  // Second
        } else if abs_value >= 1e-3 {
            "ms" // Millisecond
        } else if abs_value >= 1e-6 {
            "μs" // Microsecond
        } else if abs_value >= 1e-9 {
            "ns" // Nanosecond
        } else {
            "ps" // Picosecond
        }
    }
    
    fn auto_scale<V>(quantity: Quantity<Self, V>) -> (f64, &'static str)
    where
        V: Copy + Into<f64>
    {
        let value: f64 = quantity.value.into();
        let prefix = Self::best_prefix_for_value(value);
        
        let scaled_value = match prefix {
            "yr" => value / 3.15e7,
            "d" => value / 86400.0,
            "h" => value / 3600.0,
            "min" => value / 60.0,
            "s" => value,
            "ms" => value / 1e-3,
            "μs" => value / 1e-6,
            "ns" => value / 1e-9,
            "ps" => value / 1e-12,
            _ => value,
        };
        
        (scaled_value, prefix)
    }
}

// Convenience functions for common conversions
impl<V> Quantity<crate::units::Meter, V>
where
    V: Copy + From<f64> + std::ops::Div<f64, Output = V>,
{
    /// Convert meters to kilometers
    pub fn to_km(self) -> Quantity<crate::units::Kilometer, V> {
        self.to_prefixed::<crate::units::prefixes::Kilo>()
    }
    
    /// Convert meters to centimeters
    pub fn to_cm(self) -> Quantity<crate::units::Centimeter, V> {
        self.to_prefixed::<crate::units::prefixes::Centi>()
    }
    
    /// Convert meters to millimeters
    pub fn to_mm(self) -> Quantity<crate::units::Millimeter, V> {
        self.to_prefixed::<crate::units::prefixes::Milli>()
    }
}

impl<V> Quantity<crate::units::Gram, V>
where
    V: Copy + From<f64> + std::ops::Mul<f64, Output = V>,
{
    /// Convert grams to kilograms
    pub fn to_kg(self) -> Quantity<crate::units::Kilogram, V> {
        self.to_prefixed::<crate::units::prefixes::Kilo>()
    }
    
    /// Convert grams to milligrams  
    pub fn to_mg(self) -> Quantity<crate::units::Milligram, V> {
        self.to_prefixed::<crate::units::prefixes::Milli>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::prefixes::*;
    use crate::units::base::*;
    
    #[test]
    fn test_prefix_conversions() {
        let distance = Quantity::<Meter>::new(5000.0);
        let km = distance.to_km();
        assert_eq!(km.value, 5.0);
        
        let back_to_meters = km.to_base_unit();
        assert_eq!(back_to_meters.value, 5000.0);
    }
    
    #[test]
    fn test_auto_scaling() {
        let distance = Quantity::<Meter>::new(1500.0);
        let (scaled_value, unit) = Meter::auto_scale(distance);
        assert_eq!(unit, "km");
        assert_eq!(scaled_value, 1.5);
        
        let small_distance = Quantity::<Meter>::new(0.005);
        let (scaled_value, unit) = Meter::auto_scale(small_distance);
        assert_eq!(unit, "mm");
        assert_eq!(scaled_value, 5.0);
    }
    
    #[test]
    fn test_mass_conversions() {
        let mass = Quantity::<Gram>::new(2500.0);
        let kg = mass.to_kg();
        assert_eq!(kg.value, 2.5);
    }
}