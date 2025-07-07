//! Unit symbol generation for display formatting
//!
//! Generates proper unit symbols for complex dimensional compositions.

use crate::core::{DimensionExtractor, Quantity};
use crate::core::composition::{Per, Exponent};
use crate::units::base::*;
use crate::units::derived::*;
use crate::units::prefixes::*;
use std::fmt::Write;

/// Unicode superscript characters for exponents
const SUPERSCRIPTS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
const SUPERSCRIPT_MINUS: char = '⁻';

/// Trait for units that can generate their symbol representation
pub trait UnitSymbol {
    /// Get the short symbol for this unit (e.g., "m", "kg", "J")
    fn symbol() -> &'static str;
    
    /// Get the full name for this unit (e.g., "meter", "kilogram", "joule")
    fn name() -> &'static str;
    
    /// Get the plural name for this unit (e.g., "meters", "kilograms", "joules")
    fn plural_name() -> &'static str;
}

/// Convert integer to Unicode superscript representation
fn to_superscript(mut n: i8) -> String {
    if n == 0 {
        return SUPERSCRIPTS[0].to_string();
    }
    
    let mut result = String::new();
    let negative = n < 0;
    if negative {
        result.push(SUPERSCRIPT_MINUS);
        n = -n;
    }
    
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 10) as usize);
        n /= 10;
    }
    
    for &digit in digits.iter().rev() {
        result.push(SUPERSCRIPTS[digit]);
    }
    
    result
}

// Implement UnitSymbol for base units

impl UnitSymbol for Meter {
    fn symbol() -> &'static str { "m" }
    fn name() -> &'static str { "meter" }
    fn plural_name() -> &'static str { "meters" }
}

impl UnitSymbol for Gram {
    fn symbol() -> &'static str { "g" }
    fn name() -> &'static str { "gram" }
    fn plural_name() -> &'static str { "grams" }
}

impl UnitSymbol for Second {
    fn symbol() -> &'static str { "s" }
    fn name() -> &'static str { "second" }
    fn plural_name() -> &'static str { "seconds" }
}

impl UnitSymbol for Kelvin {
    fn symbol() -> &'static str { "K" }
    fn name() -> &'static str { "kelvin" }
    fn plural_name() -> &'static str { "kelvin" } // Kelvin is singular
}

impl UnitSymbol for Ampere {
    fn symbol() -> &'static str { "A" }
    fn name() -> &'static str { "ampere" }
    fn plural_name() -> &'static str { "amperes" }
}

impl UnitSymbol for Candela {
    fn symbol() -> &'static str { "cd" }
    fn name() -> &'static str { "candela" }
    fn plural_name() -> &'static str { "candelas" }
}

impl UnitSymbol for Mole {
    fn symbol() -> &'static str { "mol" }
    fn name() -> &'static str { "mole" }
    fn plural_name() -> &'static str { "moles" }
}

// Implement UnitSymbol for derived units

impl UnitSymbol for Newton {
    fn symbol() -> &'static str { "N" }
    fn name() -> &'static str { "newton" }
    fn plural_name() -> &'static str { "newtons" }
}

impl UnitSymbol for Joule {
    fn symbol() -> &'static str { "J" }
    fn name() -> &'static str { "joule" }
    fn plural_name() -> &'static str { "joules" }
}

impl UnitSymbol for Watt {
    fn symbol() -> &'static str { "W" }
    fn name() -> &'static str { "watt" }
    fn plural_name() -> &'static str { "watts" }
}

impl UnitSymbol for Pascal {
    fn symbol() -> &'static str { "Pa" }
    fn name() -> &'static str { "pascal" }
    fn plural_name() -> &'static str { "pascals" }
}

impl UnitSymbol for Coulomb {
    fn symbol() -> &'static str { "C" }
    fn name() -> &'static str { "coulomb" }
    fn plural_name() -> &'static str { "coulombs" }
}

impl UnitSymbol for Volt {
    fn symbol() -> &'static str { "V" }
    fn name() -> &'static str { "volt" }
    fn plural_name() -> &'static str { "volts" }
}

impl UnitSymbol for Ohm {
    fn symbol() -> &'static str { "Ω" }
    fn name() -> &'static str { "ohm" }
    fn plural_name() -> &'static str { "ohms" }
}

impl UnitSymbol for Farad {
    fn symbol() -> &'static str { "F" }
    fn name() -> &'static str { "farad" }
    fn plural_name() -> &'static str { "farads" }
}

impl UnitSymbol for Henry {
    fn symbol() -> &'static str { "H" }
    fn name() -> &'static str { "henry" }
    fn plural_name() -> &'static str { "henries" }
}

impl UnitSymbol for Weber {
    fn symbol() -> &'static str { "Wb" }
    fn name() -> &'static str { "weber" }
    fn plural_name() -> &'static str { "webers" }
}

impl UnitSymbol for Tesla {
    fn symbol() -> &'static str { "T" }
    fn name() -> &'static str { "tesla" }
    fn plural_name() -> &'static str { "teslas" }
}

impl UnitSymbol for Hertz {
    fn symbol() -> &'static str { "Hz" }
    fn name() -> &'static str { "hertz" }
    fn plural_name() -> &'static str { "hertz" }
}

impl UnitSymbol for Lumen {
    fn symbol() -> &'static str { "lm" }
    fn name() -> &'static str { "lumen" }
    fn plural_name() -> &'static str { "lumens" }
}

impl UnitSymbol for Lux {
    fn symbol() -> &'static str { "lx" }
    fn name() -> &'static str { "lux" }
    fn plural_name() -> &'static str { "lux" }
}

impl UnitSymbol for Becquerel {
    fn symbol() -> &'static str { "Bq" }
    fn name() -> &'static str { "becquerel" }
    fn plural_name() -> &'static str { "becquerels" }
}

impl UnitSymbol for Gray {
    fn symbol() -> &'static str { "Gy" }
    fn name() -> &'static str { "gray" }
    fn plural_name() -> &'static str { "grays" }
}

impl UnitSymbol for Sievert {
    fn symbol() -> &'static str { "Sv" }
    fn name() -> &'static str { "sievert" }
    fn plural_name() -> &'static str { "sieverts" }
}

impl UnitSymbol for Katal {
    fn symbol() -> &'static str { "kat" }
    fn name() -> &'static str { "katal" }
    fn plural_name() -> &'static str { "katals" }
}

// Special implementations for common prefixed units
// Note: We implement these directly rather than generically to avoid trait conflicts

impl UnitSymbol for Kilogram {
    fn symbol() -> &'static str { "kg" }
    fn name() -> &'static str { "kilogram" }
    fn plural_name() -> &'static str { "kilograms" }
}

impl UnitSymbol for Kilometer {
    fn symbol() -> &'static str { "km" }
    fn name() -> &'static str { "kilometer" }
    fn plural_name() -> &'static str { "kilometers" }
}

impl UnitSymbol for Centimeter {
    fn symbol() -> &'static str { "cm" }
    fn name() -> &'static str { "centimeter" }
    fn plural_name() -> &'static str { "centimeters" }
}

impl UnitSymbol for Millimeter {
    fn symbol() -> &'static str { "mm" }
    fn name() -> &'static str { "millimeter" }
    fn plural_name() -> &'static str { "millimeters" }
}

impl UnitSymbol for Micrometer {
    fn symbol() -> &'static str { "μm" }
    fn name() -> &'static str { "micrometer" }
    fn plural_name() -> &'static str { "micrometers" }
}

impl UnitSymbol for Nanometer {
    fn symbol() -> &'static str { "nm" }
    fn name() -> &'static str { "nanometer" }
    fn plural_name() -> &'static str { "nanometers" }
}

impl UnitSymbol for Milligram {
    fn symbol() -> &'static str { "mg" }
    fn name() -> &'static str { "milligram" }
    fn plural_name() -> &'static str { "milligrams" }
}

impl UnitSymbol for Microgram {
    fn symbol() -> &'static str { "μg" }
    fn name() -> &'static str { "microgram" }
    fn plural_name() -> &'static str { "micrograms" }
}

impl UnitSymbol for Microsecond {
    fn symbol() -> &'static str { "μs" }
    fn name() -> &'static str { "microsecond" }
    fn plural_name() -> &'static str { "microseconds" }
}

impl UnitSymbol for Nanosecond {
    fn symbol() -> &'static str { "ns" }
    fn name() -> &'static str { "nanosecond" }
    fn plural_name() -> &'static str { "nanoseconds" }
}

impl UnitSymbol for Millisecond {
    fn symbol() -> &'static str { "ms" }
    fn name() -> &'static str { "millisecond" }
    fn plural_name() -> &'static str { "milliseconds" }
}

// Implement UnitSymbol for compositional operators

impl<U: UnitSymbol> UnitSymbol for Per<U> {
    fn symbol() -> &'static str {
        "per_unit" // Will be handled by composition logic
    }
    
    fn name() -> &'static str {
        "per unit" // Will be handled by composition logic
    }
    
    fn plural_name() -> &'static str {
        "per units" // Will be handled by composition logic
    }
}

impl<U: UnitSymbol, const N: i8> UnitSymbol for Exponent<U, N> {
    fn symbol() -> &'static str {
        "exponent_unit" // Will be handled by composition logic
    }
    
    fn name() -> &'static str {
        "exponent unit" // Will be handled by composition logic
    }
    
    fn plural_name() -> &'static str {
        "exponent units" // Will be handled by composition logic
    }
}

/// Generate unit symbol for dimensional compositions
pub trait DimensionalSymbol {
    /// Generate compact mathematical symbol (e.g., "W/(m²⋅K⁴)")
    fn compact_symbol() -> String;
    
    /// Generate verbose description (e.g., "watts per square meter per kelvin to the fourth")
    fn verbose_symbol() -> String;
    
    /// Generate scientific notation symbol (e.g., "W⋅m^(-2)⋅K^(-4)")
    fn scientific_symbol() -> String;
}

// Base implementation using dimensional analysis
impl<U: DimensionExtractor> DimensionalSymbol for U {
    fn compact_symbol() -> String {
        let mut positive_terms = Vec::new();
        let mut negative_terms = Vec::new();
        
        // Length
        if U::L != 0 {
            if U::L > 0 {
                if U::L == 1 {
                    positive_terms.push("m".to_string());
                } else {
                    positive_terms.push(format!("m{}", to_superscript(U::L)));
                }
            } else {
                if U::L == -1 {
                    negative_terms.push("m".to_string());
                } else {
                    negative_terms.push(format!("m{}", to_superscript(-U::L)));
                }
            }
        }
        
        // Mass  
        if U::M != 0 {
            if U::M > 0 {
                if U::M == 1 {
                    positive_terms.push("kg".to_string()); // Use kg as standard for mass display
                } else {
                    positive_terms.push(format!("kg{}", to_superscript(U::M)));
                }
            } else {
                if U::M == -1 {
                    negative_terms.push("kg".to_string());
                } else {
                    negative_terms.push(format!("kg{}", to_superscript(-U::M)));
                }
            }
        }
        
        // Time
        if U::T != 0 {
            if U::T > 0 {
                if U::T == 1 {
                    positive_terms.push("s".to_string());
                } else {
                    positive_terms.push(format!("s{}", to_superscript(U::T)));
                }
            } else {
                if U::T == -1 {
                    negative_terms.push("s".to_string());
                } else {
                    negative_terms.push(format!("s{}", to_superscript(-U::T)));
                }
            }
        }
        
        // Temperature
        if U::THETA != 0 {
            if U::THETA > 0 {
                if U::THETA == 1 {
                    positive_terms.push("K".to_string());
                } else {
                    positive_terms.push(format!("K{}", to_superscript(U::THETA)));
                }
            } else {
                if U::THETA == -1 {
                    negative_terms.push("K".to_string());
                } else {
                    negative_terms.push(format!("K{}", to_superscript(-U::THETA)));
                }
            }
        }
        
        // Current
        if U::I != 0 {
            if U::I > 0 {
                if U::I == 1 {
                    positive_terms.push("A".to_string());
                } else {
                    positive_terms.push(format!("A{}", to_superscript(U::I)));
                }
            } else {
                if U::I == -1 {
                    negative_terms.push("A".to_string());
                } else {
                    negative_terms.push(format!("A{}", to_superscript(-U::I)));
                }
            }
        }
        
        // Luminous intensity
        if U::J != 0 {
            if U::J > 0 {
                if U::J == 1 {
                    positive_terms.push("cd".to_string());
                } else {
                    positive_terms.push(format!("cd{}", to_superscript(U::J)));
                }
            } else {
                if U::J == -1 {
                    negative_terms.push("cd".to_string());
                } else {
                    negative_terms.push(format!("cd{}", to_superscript(-U::J)));
                }
            }
        }
        
        // Amount of substance
        if U::N != 0 {
            if U::N > 0 {
                if U::N == 1 {
                    positive_terms.push("mol".to_string());
                } else {
                    positive_terms.push(format!("mol{}", to_superscript(U::N)));
                }
            } else {
                if U::N == -1 {
                    negative_terms.push("mol".to_string());
                } else {
                    negative_terms.push(format!("mol{}", to_superscript(-U::N)));
                }
            }
        }
        
        // Build result
        if positive_terms.is_empty() && negative_terms.is_empty() {
            return "1".to_string(); // Dimensionless
        }
        
        let mut result = String::new();
        
        if !positive_terms.is_empty() {
            result.push_str(&positive_terms.join("⋅"));
        }
        
        if !negative_terms.is_empty() {
            if !positive_terms.is_empty() {
                result.push('/');
                if negative_terms.len() > 1 {
                    result.push('(');
                    result.push_str(&negative_terms.join("⋅"));
                    result.push(')');
                } else {
                    result.push_str(&negative_terms[0]);
                }
            } else {
                result.push_str("1/(");
                result.push_str(&negative_terms.join("⋅"));
                result.push(')');
            }
        }
        
        result
    }
    
    fn verbose_symbol() -> String {
        // TODO: Implement verbose symbol generation
        "verbose unit".to_string()
    }
    
    fn scientific_symbol() -> String {
        // TODO: Implement scientific symbol generation  
        "scientific unit".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_superscript_conversion() {
        assert_eq!(to_superscript(0), "⁰");
        assert_eq!(to_superscript(1), "¹");
        assert_eq!(to_superscript(2), "²");
        assert_eq!(to_superscript(-1), "⁻¹");
        assert_eq!(to_superscript(-4), "⁻⁴");
        assert_eq!(to_superscript(12), "¹²");
    }
    
    #[test]
    fn test_unit_symbols() {
        assert_eq!(Meter::symbol(), "m");
        assert_eq!(Kilogram::symbol(), "kg");
        assert_eq!(Joule::symbol(), "J");
        assert_eq!(Watt::symbol(), "W");
    }
}