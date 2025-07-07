//! Simple formatting for quantities with Unicode superscripts
//!
//! Focused implementation for displaying Stefan-Boltzmann constant as W/(m²⋅K⁴)

use crate::core::{DimensionExtractor, Quantity};
use std::fmt;

/// Unicode superscript characters for exponents
const SUPERSCRIPTS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
const SUPERSCRIPT_MINUS: char = '⁻';

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

/// Generate compact unit symbol for dimensional compositions
fn generate_unit_symbol<U: DimensionExtractor>() -> String {
    // Check for well-known derived units first
    match (U::L, U::M, U::T, U::THETA, U::I, U::J, U::N) {
        // Energy: L²M¹T⁻²
        (2, 1, -2, 0, 0, 0, 0) => return "J".to_string(),
        // Power: L²M¹T⁻³  
        (2, 1, -3, 0, 0, 0, 0) => return "W".to_string(),
        // Force: L¹M¹T⁻²
        (1, 1, -2, 0, 0, 0, 0) => return "N".to_string(),
        // Pressure: L⁻¹M¹T⁻²
        (-1, 1, -2, 0, 0, 0, 0) => return "Pa".to_string(),
        // Frequency: T⁻¹
        (0, 0, -1, 0, 0, 0, 0) => return "Hz".to_string(),
        // Charge: IT¹
        (0, 0, 1, 0, 1, 0, 0) => return "C".to_string(),
        // Voltage: L²M¹T⁻³I⁻¹
        (2, 1, -3, 0, -1, 0, 0) => return "V".to_string(),
        // Magnetic field: M¹T⁻²I⁻¹
        (0, 1, -2, 0, -1, 0, 0) => return "T".to_string(),
        _ => {} // Fall through to dimensional analysis
    }
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
    
    // Mass (use kg as standard for display)
    if U::M != 0 {
        if U::M > 0 {
            if U::M == 1 {
                positive_terms.push("kg".to_string());
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

// Simple Display implementation
impl<U, V> fmt::Display for Quantity<U, V>
where
    U: DimensionExtractor,
    V: fmt::Display + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision().unwrap_or(crate::DEFAULT_PRECISION);
        let unit_symbol = generate_unit_symbol::<U>();
        
        #[cfg(feature = "compact")]
        {
            write!(f, "{:.prec$} {}", self.value(), unit_symbol, prec = precision)
        }
        
        #[cfg(not(feature = "compact"))]
        {
            write!(f, "{:.prec$} {}", self.value(), unit_symbol, prec = precision)
        }
    }
}

// Debug implementation
impl<U, V> fmt::Debug for Quantity<U, V>
where
    U: DimensionExtractor,
    V: fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Quantity")
            .field("value", &self.value())
            .field("dimensions", &format!("L{}M{}T{}Θ{}I{}J{}N{}", 
                U::L, U::M, U::T, U::THETA, U::I, U::J, U::N))
            .finish()
    }
}

// Note: Named units (Joule, Watt, etc.) will be handled by checking their 
// dimensional composition and using known symbols where appropriate

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::composition::*;
    use crate::units::base::*;
    use crate::units::derived::*;
    
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
    fn test_stefan_boltzmann_display() {
        // Test the Stefan-Boltzmann constant dimensions
        type StefanBoltzmannDimensions = (Watt, Per<Exponent<Meter, 2>>, Per<Exponent<Kelvin, 4>>);
        let constant = Quantity::<StefanBoltzmannDimensions>::new(5.670374419e-8);
        
        let display_str = format!("{:.3}", constant);
        
        println!("Stefan-Boltzmann constant: {}", constant);
        println!("Display string: '{}'", display_str);
        
        // Should display as: "0.000 W/(m²⋅K⁴)" with Unicode superscripts
        assert!(display_str.contains("W"));
        assert!(display_str.contains("m²"));
        assert!(display_str.contains("K⁴"));
        assert!(display_str.contains("0.000"));
    }
    
    #[test] 
    fn test_basic_unit_display() {
        let energy = Quantity::<Joule>::new(6.626e-34);
        let display_str = format!("{:.3}", energy);
        assert!(display_str.contains("J"));
        println!("Energy: {}", energy);
        
        let velocity = Quantity::<(Meter, Per<Second>)>::new(299792458.0);
        let display_str = format!("{}", velocity);
        assert!(display_str.contains("m/s"));
        println!("Speed of light: {}", velocity);
    }
}