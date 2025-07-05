//! Scientific notation formatting for extreme values

use std::fmt;

/// Trait for scientific notation formatting
pub trait ScientificFormat {
    /// Format in scientific notation if magnitude is extreme
    fn format_scientific(&self, threshold: f64) -> String;
    
    /// Always format in scientific notation
    fn format_always_scientific(&self, precision: usize) -> String;
    
    /// Check if value should use scientific notation
    fn should_use_scientific(&self, threshold: f64) -> bool;
}

impl ScientificFormat for f64 {
    fn format_scientific(&self, threshold: f64) -> String {
        if self.should_use_scientific(threshold) {
            self.format_always_scientific(3)
        } else {
            format!("{}", self)
        }
    }
    
    fn format_always_scientific(&self, precision: usize) -> String {
        format!("{:.prec$e}", self, prec = precision)
    }
    
    fn should_use_scientific(&self, threshold: f64) -> bool {
        let abs_val = self.abs();
        abs_val != 0.0 && (abs_val >= threshold || abs_val < 1.0 / threshold)
    }
}

/// Scientific notation thresholds for different contexts
pub struct ScientificThresholds;

impl ScientificThresholds {
    /// Conservative threshold (10^6)
    pub const CONSERVATIVE: f64 = 1e6;
    
    /// Standard threshold (10^4)
    pub const STANDARD: f64 = 1e4;
    
    /// Aggressive threshold (10^3)
    pub const AGGRESSIVE: f64 = 1e3;
    
    /// Astronomical threshold (10^9) - for space-scale quantities
    pub const ASTRONOMICAL: f64 = 1e9;
    
    /// Microscopic threshold (10^12) - for atomic-scale quantities
    pub const MICROSCOPIC: f64 = 1e12;
}

/// Custom scientific formatter with configurable precision and thresholds
pub struct ScientificFormatter {
    pub threshold: f64,
    pub precision: usize,
    pub force_scientific: bool,
}

impl ScientificFormatter {
    pub fn new(threshold: f64, precision: usize) -> Self {
        Self {
            threshold,
            precision,
            force_scientific: false,
        }
    }
    
    pub fn conservative() -> Self {
        Self::new(ScientificThresholds::CONSERVATIVE, 3)
    }
    
    pub fn standard() -> Self {
        Self::new(ScientificThresholds::STANDARD, 3)
    }
    
    pub fn aggressive() -> Self {
        Self::new(ScientificThresholds::AGGRESSIVE, 2)
    }
    
    pub fn astronomical() -> Self {
        Self::new(ScientificThresholds::ASTRONOMICAL, 2)
    }
    
    pub fn microscopic() -> Self {
        Self::new(ScientificThresholds::MICROSCOPIC, 3)
    }
    
    pub fn always_scientific(precision: usize) -> Self {
        Self {
            threshold: 0.0,
            precision,
            force_scientific: true,
        }
    }
    
    pub fn format(&self, value: f64) -> String {
        if self.force_scientific || value.should_use_scientific(self.threshold) {
            value.format_always_scientific(self.precision)
        } else {
            // Use regular formatting but limit decimal places for readability
            if value.fract() == 0.0 && value.abs() < 1e10 {
                format!("{:.0}", value)
            } else if value.abs() >= 1.0 {
                format!("{:.3}", value)
            } else {
                format!("{:.6}", value)
            }
        }
    }
}

impl Default for ScientificFormatter {
    fn default() -> Self {
        Self::standard()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scientific_format() {
        assert_eq!(1234.0.format_scientific(1e4), "1234");
        assert_eq!(12345.0.format_scientific(1e4), "1.235e4");
        assert_eq!(0.001.format_scientific(1e4), "1.000e-3");
        assert_eq!(0.5.format_scientific(1e4), "0.5");
    }

    #[test]
    fn test_should_use_scientific() {
        assert!(!1234.0.should_use_scientific(1e4));
        assert!(12345.0.should_use_scientific(1e4));
        assert!(0.0001.should_use_scientific(1e4));
        assert!(!0.5.should_use_scientific(1e4));
    }

    #[test]
    fn test_formatter_presets() {
        let conservative = ScientificFormatter::conservative();
        let standard = ScientificFormatter::standard();
        let aggressive = ScientificFormatter::aggressive();
        
        let value = 12345.0;
        
        assert_eq!(conservative.format(value), "12345.000");
        assert_eq!(standard.format(value), "1.235e4");
        assert_eq!(aggressive.format(value), "1.23e4");
    }

    #[test]
    fn test_always_scientific() {
        let formatter = ScientificFormatter::always_scientific(2);
        
        assert_eq!(formatter.format(123.0), "1.23e2");
        assert_eq!(formatter.format(0.5), "5.00e-1");
        assert_eq!(formatter.format(1.0), "1.00e0");
    }

    #[test]
    fn test_astronomical_formatter() {
        let formatter = ScientificFormatter::astronomical();
        
        // Should not use scientific for millions
        assert_eq!(formatter.format(1e6), "1000000.000");
        
        // Should use scientific for billions
        assert_eq!(formatter.format(1e10), "1.00e10");
    }
}