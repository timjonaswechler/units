//! Intelligent unit selection for optimal display

use std::collections::HashMap;

/// Trait for intelligent unit selection
pub trait UnitSelection {
    /// Get the best unit for displaying this value
    fn best_unit_for_value(&self, value: f64) -> Option<(&'static str, f64, f64)>;
    
    /// Get available units with their conversion factors
    fn available_units(&self) -> Vec<(&'static str, f64)>;
}

/// Common unit scales for different quantities
pub struct UnitScales;

impl UnitScales {
    /// Distance scales from smallest to largest
    pub const DISTANCE: &'static [(&'static str, f64)] = &[
        ("pm", 1e-12),    // Picometer
        ("nm", 1e-9),     // Nanometer  
        ("μm", 1e-6),     // Micrometer
        ("mm", 1e-3),     // Millimeter
        ("cm", 1e-2),     // Centimeter
        ("m", 1.0),       // Meter (base)
        ("km", 1e3),      // Kilometer
        ("AU", 1.496e11), // Astronomical Unit
        ("ly", 9.461e15), // Light Year
        ("pc", 3.086e16), // Parsec
    ];
    
    /// Mass scales from smallest to largest
    pub const MASS: &'static [(&'static str, f64)] = &[
        ("μg", 1e-9),     // Microgram
        ("mg", 1e-6),     // Milligram
        ("g", 1e-3),      // Gram
        ("kg", 1.0),      // Kilogram (base)
        ("t", 1e3),       // Tonne
        ("M⊕", 5.972e24), // Earth mass
        ("M☉", 1.989e30), // Solar mass
    ];
    
    /// Time scales from smallest to largest
    pub const TIME: &'static [(&'static str, f64)] = &[
        ("ps", 1e-12),    // Picosecond
        ("ns", 1e-9),     // Nanosecond
        ("μs", 1e-6),     // Microsecond
        ("ms", 1e-3),     // Millisecond
        ("s", 1.0),       // Second (base)
        ("min", 60.0),    // Minute
        ("h", 3600.0),    // Hour
        ("d", 86400.0),   // Day
        ("yr", 31557600.0), // Year
    ];
    
    /// Energy scales from smallest to largest
    pub const ENERGY: &'static [(&'static str, f64)] = &[
        ("eV", 1.602e-19), // Electron volt
        ("meV", 1.602e-22), // Milli-electron volt
        ("μJ", 1e-6),      // Microjoule
        ("mJ", 1e-3),      // Millijoule
        ("J", 1.0),        // Joule (base)
        ("kJ", 1e3),       // Kilojoule
        ("MJ", 1e6),       // Megajoule
        ("GJ", 1e9),       // Gigajoule
        ("cal", 4.184),    // Calorie
        ("kcal", 4184.0),  // Kilocalorie
        ("kWh", 3.6e6),    // Kilowatt-hour
    ];
    
    /// Power scales from smallest to largest
    pub const POWER: &'static [(&'static str, f64)] = &[
        ("μW", 1e-6),      // Microwatt
        ("mW", 1e-3),      // Milliwatt
        ("W", 1.0),        // Watt (base)
        ("kW", 1e3),       // Kilowatt
        ("MW", 1e6),       // Megawatt
        ("GW", 1e9),       // Gigawatt
        ("hp", 745.7),     // Horsepower
        ("L☉", 3.828e26),  // Solar luminosity
    ];
}

/// Smart unit selector that chooses the most appropriate unit for a value
pub struct SmartUnitSelector {
    units: Vec<(&'static str, f64)>,
    prefer_round_numbers: bool,
    min_display_value: f64,
    max_display_value: f64,
}

impl SmartUnitSelector {
    pub fn new(units: &[(&'static str, f64)]) -> Self {
        let mut sorted_units = units.to_vec();
        sorted_units.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        
        Self {
            units: sorted_units,
            prefer_round_numbers: true,
            min_display_value: 0.1,
            max_display_value: 1000.0,
        }
    }
    
    pub fn for_distance() -> Self {
        Self::new(UnitScales::DISTANCE)
    }
    
    pub fn for_mass() -> Self {
        Self::new(UnitScales::MASS)
    }
    
    pub fn for_time() -> Self {
        Self::new(UnitScales::TIME)
    }
    
    pub fn for_energy() -> Self {
        Self::new(UnitScales::ENERGY)
    }
    
    pub fn for_power() -> Self {
        Self::new(UnitScales::POWER)
    }
    
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.min_display_value = min;
        self.max_display_value = max;
        self
    }
    
    pub fn prefer_exact_matches(mut self, prefer: bool) -> Self {
        self.prefer_round_numbers = prefer;
        self
    }
    
    /// Find the best unit for a given value in SI base units
    pub fn select_best_unit(&self, si_value: f64) -> (&'static str, f64, f64) {
        if si_value == 0.0 {
            return self.units.first()
                .map(|(symbol, factor)| (*symbol, 0.0, *factor))
                .unwrap_or(("", 0.0, 1.0));
        }
        
        let abs_value = si_value.abs();
        let mut best_unit = self.units[0];
        let mut best_score = f64::INFINITY;
        
        for &(symbol, factor) in &self.units {
            let converted_value = abs_value / factor;
            
            // Skip if converted value is too extreme
            if converted_value < 1e-6 || converted_value > 1e6 {
                continue;
            }
            
            let score = self.calculate_score(converted_value);
            
            if score < best_score {
                best_score = score;
                best_unit = (symbol, factor);
            }
        }
        
        let (symbol, factor) = best_unit;
        let converted_value = si_value / factor;
        
        (symbol, converted_value, factor)
    }
    
    fn calculate_score(&self, value: f64) -> f64 {
        let abs_value = value.abs();
        
        // Base score: how far from optimal range [min_display_value, max_display_value]
        let mut score = if abs_value < self.min_display_value {
            self.min_display_value / abs_value
        } else if abs_value > self.max_display_value {
            abs_value / self.max_display_value
        } else {
            1.0 // In optimal range
        };
        
        // Bonus for round numbers if preferred
        if self.prefer_round_numbers {
            let log_value = abs_value.log10();
            let fractional_part = log_value.fract().abs();
            
            // Prefer powers of 10 and simple multiples
            if fractional_part < 0.05 || fractional_part > 0.95 {
                score *= 0.8; // 20% bonus
            } else if (fractional_part - 0.301).abs() < 0.05 || (fractional_part - 0.699).abs() < 0.05 {
                // Powers of 2 (log10(2) ≈ 0.301)
                score *= 0.9; // 10% bonus
            }
        }
        
        score
    }
}

/// Unit recommendation system for different contexts
pub struct UnitRecommendation {
    quantity_type: String,
    context: String,
    recommended_units: Vec<(&'static str, f64)>,
}

impl UnitRecommendation {
    pub fn for_context(quantity_type: &str, context: &str) -> Self {
        let recommended_units = match (quantity_type, context) {
            ("distance", "astronomy") => vec![
                ("pm", 1e-12), ("nm", 1e-9), ("μm", 1e-6), ("mm", 1e-3),
                ("m", 1.0), ("km", 1e3), ("AU", 1.496e11), ("ly", 9.461e15), ("pc", 3.086e16)
            ],
            ("distance", "engineering") => vec![
                ("μm", 1e-6), ("mm", 1e-3), ("cm", 1e-2), ("m", 1.0), ("km", 1e3)
            ],
            ("distance", "microscopy") => vec![
                ("pm", 1e-12), ("nm", 1e-9), ("μm", 1e-6), ("mm", 1e-3)
            ],
            ("mass", "astronomy") => vec![
                ("kg", 1.0), ("t", 1e3), ("M⊕", 5.972e24), ("M☉", 1.989e30)
            ],
            ("mass", "chemistry") => vec![
                ("μg", 1e-9), ("mg", 1e-6), ("g", 1e-3), ("kg", 1.0)
            ],
            ("time", "physics") => vec![
                ("ps", 1e-12), ("ns", 1e-9), ("μs", 1e-6), ("ms", 1e-3), ("s", 1.0)
            ],
            ("time", "everyday") => vec![
                ("ms", 1e-3), ("s", 1.0), ("min", 60.0), ("h", 3600.0), ("d", 86400.0)
            ],
            _ => vec![("", 1.0)], // Default to base unit
        };
        
        Self {
            quantity_type: quantity_type.to_string(),
            context: context.to_string(),
            recommended_units,
        }
    }
    
    pub fn select_unit(&self, si_value: f64) -> (&'static str, f64, f64) {
        let selector = SmartUnitSelector::new(&self.recommended_units);
        selector.select_best_unit(si_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_unit_selection() {
        let selector = SmartUnitSelector::for_distance();
        
        // Atomic scale
        let (unit, value, _) = selector.select_best_unit(1e-10);
        assert_eq!(unit, "nm");
        assert!((value - 0.1).abs() < 1e-10);
        
        // Human scale
        let (unit, value, _) = selector.select_best_unit(1.8);
        assert_eq!(unit, "m");
        assert!((value - 1.8).abs() < 1e-10);
        
        // City scale
        let (unit, value, _) = selector.select_best_unit(5000.0);
        assert_eq!(unit, "km");
        assert!((value - 5.0).abs() < 1e-10);
        
        // Astronomical scale
        let (unit, value, _) = selector.select_best_unit(1.496e11);
        assert_eq!(unit, "AU");
        assert!((value - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_mass_unit_selection() {
        let selector = SmartUnitSelector::for_mass();
        
        // Laboratory scale
        let (unit, value, _) = selector.select_best_unit(0.001);
        assert_eq!(unit, "g");
        assert!((value - 1.0).abs() < 1e-10);
        
        // Human scale
        let (unit, value, _) = selector.select_best_unit(70.0);
        assert_eq!(unit, "kg");
        assert!((value - 70.0).abs() < 1e-10);
        
        // Vehicle scale
        let (unit, value, _) = selector.select_best_unit(1500.0);
        assert_eq!(unit, "t");
        assert!((value - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_unit_recommendation_astronomy() {
        let rec = UnitRecommendation::for_context("distance", "astronomy");
        
        // Should prefer AU for solar system distances
        let (unit, value, _) = rec.select_unit(7.785e11); // Jupiter distance
        assert_eq!(unit, "AU");
        assert!((value - 5.2).abs() < 0.1);
    }

    #[test]
    fn test_unit_recommendation_engineering() {
        let rec = UnitRecommendation::for_context("distance", "engineering");
        
        // Should prefer mm for small engineering dimensions
        let (unit, value, _) = rec.select_unit(0.025);
        assert_eq!(unit, "mm");
        assert!((value - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_zero_value_handling() {
        let selector = SmartUnitSelector::for_distance();
        let (unit, value, _) = selector.select_best_unit(0.0);
        assert_eq!(value, 0.0);
        assert!(!unit.is_empty());
    }

    #[test]
    fn test_prefer_round_numbers() {
        let selector = SmartUnitSelector::for_distance()
            .prefer_exact_matches(true);
        
        // Should prefer unit that gives round number
        let (unit, value, _) = selector.select_best_unit(1000.0);
        assert_eq!(unit, "km");
        assert!((value - 1.0).abs() < 1e-10);
    }
}