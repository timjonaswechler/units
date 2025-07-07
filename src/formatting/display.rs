//! Display implementation for quantities
//!
//! Provides feature-flag controlled display formatting for quantities.

use crate::core::{DimensionExtractor, Quantity};
use crate::formatting::symbols::DimensionalSymbol;
use crate::{DefaultFloat, DEFAULT_PRECISION};
use std::fmt;

/// Helper trait to check if a unit is a named unit
pub trait IsNamedUnit {
    fn is_named_unit() -> bool { false }
    fn get_symbol() -> Option<&'static str> { None }
    fn get_name() -> Option<&'static str> { None }
    fn get_plural_name() -> Option<&'static str> { None }
}

// Default implementation for all DimensionExtractor types
impl<T: DimensionExtractor> IsNamedUnit for T {}

// Override for NamedUnit types
impl<T: NamedUnit> IsNamedUnit for T {
    fn is_named_unit() -> bool { true }
    fn get_symbol() -> Option<&'static str> { Some(T::SYMBOL) }
    fn get_name() -> Option<&'static str> { Some(T::NAME) }
    fn get_plural_name() -> Option<&'static str> { Some(T::PLURAL_NAME) }
}

impl<U, V> fmt::Display for Quantity<U, V>
where
    U: DimensionExtractor + DimensionalSymbol + IsNamedUnit,
    V: fmt::Display + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision().unwrap_or(DEFAULT_PRECISION);
        
        // Check if this is a named unit and prefer its symbol
        let unit_symbol = if U::is_named_unit() {
            U::get_symbol().unwrap_or(&U::compact_symbol())
        } else {
            &U::compact_symbol()
        };
        
        #[cfg(feature = "compact")]
        {
            write!(f, "{:.prec$} {}", self.value, unit_symbol, prec = precision)
        }
        
        #[cfg(feature = "verbose")]
        {
            if U::is_named_unit() {
                let unit_name = if format!("{:.prec$}", self.value, prec = precision) == "1" {
                    U::get_name().unwrap()
                } else {
                    U::get_plural_name().unwrap()
                };
                write!(f, "{:.prec$} {}", self.value, unit_name, prec = precision)
            } else {
                write!(f, "{:.prec$} {}", self.value, U::verbose_symbol(), prec = precision)
            }
        }
        
        #[cfg(feature = "scientific")]
        {
            write!(f, "{:.prec$E} {}", self.value, unit_symbol, prec = precision)
        }
        
        #[cfg(not(any(feature = "compact", feature = "verbose", feature = "scientific")))]
        {
            write!(f, "{:.prec$} {}", self.value, unit_symbol, prec = precision)
        }
    }
}

// Special implementations for named units (like derived units)

/// Trait for units that have a well-known symbol (like Joule, Watt, etc.)
pub trait NamedUnit: DimensionExtractor {
    /// The standard symbol for this unit
    const SYMBOL: &'static str;
    
    /// The full name for this unit
    const NAME: &'static str;
    
    /// The plural name for this unit
    const PLURAL_NAME: &'static str;
}

// Implement NamedUnit for derived units
use crate::units::derived::*;

impl NamedUnit for Joule {
    const SYMBOL: &'static str = "J";
    const NAME: &'static str = "joule";
    const PLURAL_NAME: &'static str = "joules";
}

impl NamedUnit for Watt {
    const SYMBOL: &'static str = "W";
    const NAME: &'static str = "watt";
    const PLURAL_NAME: &'static str = "watts";
}

impl NamedUnit for Newton {
    const SYMBOL: &'static str = "N";
    const NAME: &'static str = "newton";
    const PLURAL_NAME: &'static str = "newtons";
}

impl NamedUnit for Pascal {
    const SYMBOL: &'static str = "Pa";
    const NAME: &'static str = "pascal";
    const PLURAL_NAME: &'static str = "pascals";
}

impl NamedUnit for Tesla {
    const SYMBOL: &'static str = "T";
    const NAME: &'static str = "tesla";
    const PLURAL_NAME: &'static str = "teslas";
}

impl NamedUnit for Hertz {
    const SYMBOL: &'static str = "Hz";
    const NAME: &'static str = "hertz";
    const PLURAL_NAME: &'static str = "hertz";
}

// Note: NamedUnit display is handled within the main Display implementation
// by checking if U implements NamedUnit and using the symbol accordingly

// Helper for debugging - always shows dimensional breakdown
impl<U, V> fmt::Debug for Quantity<U, V>
where
    U: DimensionExtractor,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Quantity")
            .field("value", &self.value)
            .field("dimensions", &format!("L{}M{}T{}Θ{}I{}J{}N{}", 
                U::L, U::M, U::T, U::THETA, U::I, U::J, U::N))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::base::*;
    use crate::core::composition::*;
    
    #[test] 
    fn test_basic_display() {
        let energy = Quantity::<Joule>::new(6.626e-34);
        let display_str = format!("{:.3}", energy);
        
        #[cfg(feature = "compact")]
        assert!(display_str.contains("J"));
        
        #[cfg(feature = "verbose")]
        assert!(display_str.contains("joule"));
    }
    
    #[test]
    fn test_dimensional_display() {
        // Test Stefan-Boltzmann constant dimensions
        type StefanBoltzmannDimensions = (Watt, Per<Exponent<Meter, 2>>, Per<Exponent<Kelvin, 4>>);
        let constant = Quantity::<StefanBoltzmannDimensions>::new(5.670374419e-8);
        
        let display_str = format!("{:.3e}", constant);
        
        #[cfg(feature = "compact")]
        {
            assert!(display_str.contains("W"));
            assert!(display_str.contains("m²"));
            assert!(display_str.contains("K⁴"));
        }
    }
}