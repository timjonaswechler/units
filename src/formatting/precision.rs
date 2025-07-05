//! Precision control for quantity display

use std::fmt;

/// Trait for precision-controlled formatting
pub trait PrecisionFormat {
    /// Format with specific decimal places
    fn format_precision(&self, precision: usize) -> String;
    
    /// Format with significant figures
    fn format_significant(&self, sig_figs: usize) -> String;
    
    /// Auto-adjust precision based on magnitude
    fn format_auto_precision(&self) -> String;
}

impl PrecisionFormat for f64 {
    fn format_precision(&self, precision: usize) -> String {
        format!("{:.prec$}", self, prec = precision)
    }
    
    fn format_significant(&self, sig_figs: usize) -> String {
        if *self == 0.0 {
            return "0".to_string();
        }
        
        let magnitude = self.abs().log10().floor() as i32;
        let precision = if magnitude >= 0 {
            if sig_figs as i32 > magnitude + 1 {
                sig_figs - (magnitude as usize + 1)
            } else {
                0
            }
        } else {
            sig_figs + (-magnitude - 1) as usize
        };
        
        format!("{:.prec$}", self, prec = precision)
    }
    
    fn format_auto_precision(&self) -> String {
        let abs_val = self.abs();
        
        if abs_val == 0.0 {
            return "0".to_string();
        }
        
        // Auto-adjust precision based on magnitude
        let precision = if abs_val >= 1000.0 {
            0  // No decimals for large numbers
        } else if abs_val >= 10.0 {
            1  // One decimal for medium numbers
        } else if abs_val >= 1.0 {
            2  // Two decimals for small numbers
        } else if abs_val >= 0.01 {
            3  // Three decimals for very small numbers
        } else {
            6  // Six decimals for tiny numbers
        };
        
        format!("{:.prec$}", self, prec = precision)
    }
}

/// Precision modes for different use cases
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrecisionMode {
    /// Fixed number of decimal places
    Fixed(usize),
    /// Fixed number of significant figures
    Significant(usize),
    /// Auto-adjust precision based on magnitude
    Auto,
    /// Engineering precision (3 sig figs)
    Engineering,
    /// Scientific precision (4 sig figs)
    Scientific,
    /// Display precision (appropriate for UI)
    Display,
}

impl PrecisionMode {
    pub fn format(&self, value: f64) -> String {
        match self {
            PrecisionMode::Fixed(precision) => value.format_precision(*precision),
            PrecisionMode::Significant(sig_figs) => value.format_significant(*sig_figs),
            PrecisionMode::Auto => value.format_auto_precision(),
            PrecisionMode::Engineering => value.format_significant(3),
            PrecisionMode::Scientific => value.format_significant(4),
            PrecisionMode::Display => {
                // For display, use auto precision but cap at 3 decimals
                let auto = value.format_auto_precision();
                if auto.contains('.') {
                    let parts: Vec<&str> = auto.split('.').collect();
                    if parts.len() == 2 && parts[1].len() > 3 {
                        return format!("{:.3}", value);
                    }
                }
                auto
            }
        }
    }
}

impl Default for PrecisionMode {
    fn default() -> Self {
        PrecisionMode::Display
    }
}

/// Precision formatter with contextual awareness
pub struct PrecisionFormatter {
    pub mode: PrecisionMode,
    pub min_precision: usize,
    pub max_precision: usize,
}

impl PrecisionFormatter {
    pub fn new(mode: PrecisionMode) -> Self {
        Self {
            mode,
            min_precision: 0,
            max_precision: 6,
        }
    }
    
    pub fn fixed(precision: usize) -> Self {
        Self::new(PrecisionMode::Fixed(precision))
    }
    
    pub fn significant(sig_figs: usize) -> Self {
        Self::new(PrecisionMode::Significant(sig_figs))
    }
    
    pub fn auto() -> Self {
        Self::new(PrecisionMode::Auto)
    }
    
    pub fn engineering() -> Self {
        Self::new(PrecisionMode::Engineering)
    }
    
    pub fn scientific() -> Self {
        Self::new(PrecisionMode::Scientific)
    }
    
    pub fn display() -> Self {
        Self::new(PrecisionMode::Display)
    }
    
    pub fn with_limits(mut self, min: usize, max: usize) -> Self {
        self.min_precision = min;
        self.max_precision = max;
        self
    }
    
    pub fn format(&self, value: f64) -> String {
        let result = self.mode.format(value);
        
        // Apply precision limits if needed
        if let Some(dot_pos) = result.find('.') {
            let decimal_places = result.len() - dot_pos - 1;
            if decimal_places > self.max_precision {
                return format!("{:.prec$}", value, prec = self.max_precision);
            }
            if decimal_places < self.min_precision {
                return format!("{:.prec$}", value, prec = self.min_precision);
            }
        }
        
        result
    }
}

impl Default for PrecisionFormatter {
    fn default() -> Self {
        Self::display()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_precision() {
        assert_eq!(3.14159.format_precision(2), "3.14");
        assert_eq!(3.14159.format_precision(4), "3.1416");
        assert_eq!(1000.0.format_precision(0), "1000");
    }

    #[test]
    fn test_format_significant() {
        assert_eq!(3.14159.format_significant(3), "3.14");
        assert_eq!(31.4159.format_significant(3), "31.4");
        assert_eq!(314.159.format_significant(3), "314");
        assert_eq!(0.00314159.format_significant(3), "0.00314");
    }

    #[test]
    fn test_format_auto_precision() {
        assert_eq!(1234.5.format_auto_precision(), "1235");
        assert_eq!(123.45.format_auto_precision(), "123.5");
        assert_eq!(12.345.format_auto_precision(), "12.35");
        assert_eq!(1.2345.format_auto_precision(), "1.23");
        assert_eq!(0.12345.format_auto_precision(), "0.123");
        assert_eq!(0.0012345.format_auto_precision(), "0.001235");
    }

    #[test]
    fn test_precision_modes() {
        let value = 3.14159;
        
        assert_eq!(PrecisionMode::Fixed(2).format(value), "3.14");
        assert_eq!(PrecisionMode::Significant(3).format(value), "3.14");
        assert_eq!(PrecisionMode::Engineering.format(value), "3.14");
        assert_eq!(PrecisionMode::Scientific.format(value), "3.142");
    }

    #[test]
    fn test_precision_formatter_limits() {
        let formatter = PrecisionFormatter::auto().with_limits(1, 3);
        
        // Should enforce minimum precision
        assert!(formatter.format(5.0).contains('.'));
        
        // Should enforce maximum precision
        let result = formatter.format(3.123456789);
        let decimal_places = result.split('.').nth(1).map(|s| s.len()).unwrap_or(0);
        assert!(decimal_places <= 3);
    }

    #[test]
    fn test_zero_handling() {
        assert_eq!(0.0.format_significant(3), "0");
        assert_eq!(0.0.format_auto_precision(), "0");
        assert_eq!(PrecisionMode::Display.format(0.0), "0");
    }
}