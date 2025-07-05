//! Display styles and formatting modes for quantities

use super::{ScientificFormatter, PrecisionFormatter, SmartUnitSelector};
use crate::core::UnitComposition;

/// Display style configurations
#[derive(Debug, Clone)]
pub enum DisplayStyle {
    /// Compact: "42.5 m"
    Compact,
    /// Verbose: "42.5 meters"
    Verbose,
    /// Scientific: "4.25e1 m"
    Scientific,
    /// Engineering: "42.5 m" (3 sig figs, appropriate units)
    Engineering,
    /// Pretty: "42.5 m" with Unicode symbols
    Pretty,
    /// LaTeX: "42.5\\,\\text{m}"
    LaTeX,
    /// ASCII-only: "42.5 m" (no Unicode)
    ASCII,
}

/// Context-aware formatting for different domains
#[derive(Debug, Clone)]
pub enum FormattingContext {
    /// General purpose display
    General,
    /// Scientific papers and publications
    Scientific,
    /// Engineering applications
    Engineering,
    /// Astronomy and astrophysics
    Astronomy,
    /// User interfaces
    UI,
    /// Debug output
    Debug,
    /// Export to other formats
    Export,
}

/// Comprehensive formatter combining all formatting features
pub struct QuantityFormatter {
    pub style: DisplayStyle,
    pub context: FormattingContext,
    pub scientific_formatter: ScientificFormatter,
    pub precision_formatter: PrecisionFormatter,
    pub auto_unit_selection: bool,
    pub show_unit_name: bool,
    pub use_unicode: bool,
}

impl QuantityFormatter {
    pub fn new(style: DisplayStyle, context: FormattingContext) -> Self {
        let (scientific_formatter, precision_formatter, auto_unit_selection, show_unit_name, use_unicode) = 
            match (&style, &context) {
                (DisplayStyle::Scientific, _) => (
                    ScientificFormatter::always_scientific(3),
                    PrecisionFormatter::scientific(),
                    false,
                    false,
                    true,
                ),
                (DisplayStyle::Engineering, _) => (
                    ScientificFormatter::standard(),
                    PrecisionFormatter::engineering(),
                    true,
                    false,
                    true,
                ),
                (DisplayStyle::Verbose, _) => (
                    ScientificFormatter::conservative(),
                    PrecisionFormatter::display(),
                    true,
                    true,
                    true,
                ),
                (DisplayStyle::ASCII, _) => (
                    ScientificFormatter::standard(),
                    PrecisionFormatter::display(),
                    true,
                    false,
                    false,
                ),
                (_, FormattingContext::Scientific) => (
                    ScientificFormatter::standard(),
                    PrecisionFormatter::scientific(),
                    false,
                    false,
                    true,
                ),
                (_, FormattingContext::Engineering) => (
                    ScientificFormatter::conservative(),
                    PrecisionFormatter::engineering(),
                    true,
                    false,
                    true,
                ),
                (_, FormattingContext::Astronomy) => (
                    ScientificFormatter::astronomical(),
                    PrecisionFormatter::significant(3),
                    true,
                    false,
                    true,
                ),
                (_, FormattingContext::UI) => (
                    ScientificFormatter::standard(),
                    PrecisionFormatter::display(),
                    true,
                    false,
                    true,
                ),
                (_, FormattingContext::Debug) => (
                    ScientificFormatter::conservative(),
                    PrecisionFormatter::fixed(6),
                    false,
                    false,
                    false,
                ),
                _ => (
                    ScientificFormatter::default(),
                    PrecisionFormatter::default(),
                    true,
                    false,
                    true,
                ),
            };
        
        Self {
            style,
            context,
            scientific_formatter,
            precision_formatter,
            auto_unit_selection,
            show_unit_name,
            use_unicode,
        }
    }
    
    /// Predefined formatters for common use cases
    pub fn compact() -> Self {
        Self::new(DisplayStyle::Compact, FormattingContext::General)
    }
    
    pub fn scientific() -> Self {
        Self::new(DisplayStyle::Scientific, FormattingContext::Scientific)
    }
    
    pub fn engineering() -> Self {
        Self::new(DisplayStyle::Engineering, FormattingContext::Engineering)
    }
    
    pub fn astronomy() -> Self {
        Self::new(DisplayStyle::Pretty, FormattingContext::Astronomy)
    }
    
    pub fn ui() -> Self {
        Self::new(DisplayStyle::Pretty, FormattingContext::UI)
    }
    
    pub fn debug() -> Self {
        Self::new(DisplayStyle::ASCII, FormattingContext::Debug)
    }
    
    pub fn latex() -> Self {
        Self::new(DisplayStyle::LaTeX, FormattingContext::Export)
    }
    
    /// Format a value with unit symbol
    pub fn format_with_unit<U>(&self, value: f64, _unit: std::marker::PhantomData<U>) -> String 
    where
        U: UnitComposition,
    {
        let unit_symbol = U::symbol();
        self.format_value_and_symbol(value, &unit_symbol)
    }
    
    /// Format a value with a given unit symbol
    pub fn format_value_and_symbol(&self, value: f64, unit_symbol: &str) -> String {
        let formatted_value = match self.style {
            DisplayStyle::Scientific => self.scientific_formatter.format(value),
            DisplayStyle::Engineering => {
                if value.abs() >= self.scientific_formatter.threshold || 
                   value.abs() < 1.0 / self.scientific_formatter.threshold {
                    self.scientific_formatter.format(value)
                } else {
                    self.precision_formatter.format(value)
                }
            },
            _ => self.precision_formatter.format(value),
        };
        
        let processed_symbol = self.process_unit_symbol(unit_symbol);
        
        match self.style {
            DisplayStyle::LaTeX => format!("{}\\,\\text{{{}}}", formatted_value, processed_symbol),
            DisplayStyle::Verbose if self.show_unit_name => {
                let unit_name = self.symbol_to_name(&processed_symbol);
                format!("{} {}", formatted_value, unit_name)
            },
            _ => format!("{} {}", formatted_value, processed_symbol),
        }
    }
    
    fn process_unit_symbol(&self, symbol: &str) -> String {
        if !self.use_unicode {
            // Convert Unicode symbols to ASCII equivalents
            symbol
                .replace("☉", "_sun")
                .replace("⊕", "_earth")
                .replace("☽", "_moon")
                .replace("μ", "u")
                .replace("°", "deg")
                .replace("′", "'")
                .replace("″", "\"")
                .replace("℃", "C")
                .replace("℉", "F")
                .replace("Ω", "ohm")
                .replace("π", "pi")
                .replace("²", "^2")
                .replace("³", "^3")
                .replace("⁻¹", "^-1")
                .replace("⁻²", "^-2")
                .replace("⁻³", "^-3")
                .replace("⋅", "*")
        } else {
            symbol.to_string()
        }
    }
    
    fn symbol_to_name(&self, symbol: &str) -> String {
        match symbol {
            "m" => "meters",
            "kg" => "kilograms", 
            "s" => "seconds",
            "A" => "amperes",
            "K" => "kelvin",
            "mol" => "moles",
            "cd" => "candela",
            "m/s" => "meters per second",
            "m/s²" => "meters per second squared",
            "kg⋅m/s²" => "newtons",
            "N" => "newtons",
            "J" => "joules",
            "W" => "watts",
            "Pa" => "pascals",
            "Hz" => "hertz",
            "C" => "coulombs",
            "V" => "volts",
            "Ω" => "ohms",
            "F" => "farads",
            "H" => "henries",
            "T" => "tesla",
            "Wb" => "webers",
            "lm" => "lumens",
            "lx" => "lux",
            "Bq" => "becquerels",
            "Gy" => "grays",
            "Sv" => "sieverts",
            "kat" => "katals",
            _ => symbol, // Fallback to symbol
        }.to_string()
    }
}

impl Default for QuantityFormatter {
    fn default() -> Self {
        Self::compact()
    }
}

/// Trait to enable custom formatting on quantities
pub trait FormattedDisplay {
    /// Format with default formatter
    fn format_default(&self) -> String;
    
    /// Format with custom formatter
    fn format_with(&self, formatter: &QuantityFormatter) -> String;
    
    /// Format for scientific context
    fn format_scientific(&self) -> String {
        self.format_with(&QuantityFormatter::scientific())
    }
    
    /// Format for engineering context
    fn format_engineering(&self) -> String {
        self.format_with(&QuantityFormatter::engineering())
    }
    
    /// Format for astronomy context
    fn format_astronomy(&self) -> String {
        self.format_with(&QuantityFormatter::astronomy())
    }
    
    /// Format for UI display
    fn format_ui(&self) -> String {
        self.format_with(&QuantityFormatter::ui())
    }
    
    /// Format for LaTeX
    fn format_latex(&self) -> String {
        self.format_with(&QuantityFormatter::latex())
    }
    
    /// Format for debug output
    fn format_debug(&self) -> String {
        self.format_with(&QuantityFormatter::debug())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_styles() {
        let formatter_compact = QuantityFormatter::compact();
        let formatter_scientific = QuantityFormatter::scientific();
        let formatter_engineering = QuantityFormatter::engineering();
        
        let value = 12345.67;
        let symbol = "m";
        
        let compact = formatter_compact.format_value_and_symbol(value, symbol);
        let scientific = formatter_scientific.format_value_and_symbol(value, symbol);
        let engineering = formatter_engineering.format_value_and_symbol(value, symbol);
        
        // Compact should be simple
        assert!(compact.contains("12345") || compact.contains("12.3"));
        
        // Scientific should use exponential notation
        assert!(scientific.contains("e"));
        
        // All should contain the unit
        assert!(compact.contains("m"));
        assert!(scientific.contains("m"));
        assert!(engineering.contains("m"));
    }

    #[test]
    fn test_context_specific_formatting() {
        let astronomy = QuantityFormatter::astronomy();
        let ui = QuantityFormatter::ui();
        let debug = QuantityFormatter::debug();
        
        let large_value = 1.496e11; // 1 AU in meters
        let symbol = "m";
        
        let astro_format = astronomy.format_value_and_symbol(large_value, symbol);
        let ui_format = ui.format_value_and_symbol(large_value, symbol);
        let debug_format = debug.format_value_and_symbol(large_value, symbol);
        
        // All should handle large values appropriately
        assert!(!astro_format.is_empty());
        assert!(!ui_format.is_empty());
        assert!(!debug_format.is_empty());
    }

    #[test]
    fn test_unicode_handling() {
        let unicode_formatter = QuantityFormatter::compact();
        let ascii_formatter = QuantityFormatter::new(DisplayStyle::ASCII, FormattingContext::General);
        
        let symbol = "μm";
        let value = 1.0;
        
        let unicode_result = unicode_formatter.format_value_and_symbol(value, symbol);
        let ascii_result = ascii_formatter.format_value_and_symbol(value, symbol);
        
        assert!(unicode_result.contains("μ"));
        assert!(ascii_result.contains("u"));
        assert!(!ascii_result.contains("μ"));
    }

    #[test]
    fn test_latex_formatting() {
        let latex = QuantityFormatter::latex();
        let result = latex.format_value_and_symbol(42.5, "m/s");
        
        assert!(result.contains("\\text{"));
        assert!(result.contains("\\,"));
    }

    #[test]
    fn test_symbol_to_name_conversion() {
        let verbose = QuantityFormatter::new(DisplayStyle::Verbose, FormattingContext::General);
        let verbose_formatter = QuantityFormatter {
            show_unit_name: true,
            ..verbose
        };
        
        let result = verbose_formatter.format_value_and_symbol(1.0, "m");
        assert!(result.contains("meters"));
        
        let result2 = verbose_formatter.format_value_and_symbol(1.0, "N");
        assert!(result2.contains("newtons"));
    }
}