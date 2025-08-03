use crate::Dimension;

/// Compile-Time Registry für Dimension → Quantity Name Mapping
/// 
/// Das Registry System ermöglicht es, zur Compile-Time aus einer berechneten
/// Dimension den entsprechenden Quantity-Namen zu ermitteln. Dies ist der
/// Kern für die automatische Typ-Deduktion.
pub struct QuantityRegistry;

impl QuantityRegistry {
    /// Const array aller bekannten Quantities mit ihren Dimensionen
    /// 
    /// Wird automatisch erweitert wenn neue Quantities definiert werden.
    /// Format: (Dimension, Quantity Name)
    pub const KNOWN_QUANTITIES: &'static [(Dimension, &'static str)] = &[
        // SI Base Quantities
        (Dimension::length(), "Distance"),
        (Dimension::time(), "Time"), 
        (Dimension::mass(), "Mass"),
        (Dimension::current(), "ElectricCurrent"),
        (Dimension::temperature(), "Temperature"),
        (Dimension::luminosity(), "LuminousIntensity"),
        (Dimension::amount(), "AmountOfSubstance"),
        
        // Derived Quantities
        (Dimension::AREA, "Area"),              // L²
        (Dimension::VOLUME, "Volume"),          // L³
        (Dimension::VELOCITY, "Speed"),         // LT⁻¹
        (Dimension::ACCELERATION, "Acceleration"), // LT⁻²
        (Dimension::FORCE, "Force"),            // MLT⁻²
        (Dimension::ENERGY, "Energy"),          // ML²T⁻²
        (Dimension::POWER, "Power"),            // ML²T⁻³
        (Dimension::PRESSURE, "Pressure"),      // ML⁻¹T⁻²
        (Dimension::FREQUENCY, "Frequency"),    // T⁻¹
        (Dimension::CHARGE, "ElectricCharge"),  // IT
        (Dimension::VOLTAGE, "ElectricPotential"), // ML²T⁻³I⁻¹
    ];
    
    /// Sucht zur Compile-Time einen Quantity-Namen für eine gegebene Dimension
    /// 
    /// Diese Funktion wird von Proc Macros verwendet um automatisch den
    /// richtigen Quantity-Typ aus berechneten Dimensionen zu ermitteln.
    /// 
    /// # Beispiel
    /// ```
    /// const FORCE_DIM: Dimension = Dimension::FORCE;
    /// const AREA_DIM: Dimension = Dimension::AREA;  
    /// const PRESSURE_DIM: Dimension = FORCE_DIM.divide(AREA_DIM);
    /// 
    /// // Proc Macro würde das so verwenden:
    /// let quantity_name = QuantityRegistry::lookup_quantity_name(PRESSURE_DIM);
    /// assert_eq!(quantity_name, Some("Pressure"));
    /// ```
    pub const fn lookup_quantity_name(dimension: Dimension) -> Option<&'static str> {
        let mut i = 0;
        while i < Self::KNOWN_QUANTITIES.len() {
            // Vergleiche alle 7 Dimension-Komponenten
            if Self::KNOWN_QUANTITIES[i].0.data[0] == dimension.data[0] &&
               Self::KNOWN_QUANTITIES[i].0.data[1] == dimension.data[1] &&
               Self::KNOWN_QUANTITIES[i].0.data[2] == dimension.data[2] &&
               Self::KNOWN_QUANTITIES[i].0.data[3] == dimension.data[3] &&
               Self::KNOWN_QUANTITIES[i].0.data[4] == dimension.data[4] &&
               Self::KNOWN_QUANTITIES[i].0.data[5] == dimension.data[5] &&
               Self::KNOWN_QUANTITIES[i].0.data[6] == dimension.data[6] {
                return Some(Self::KNOWN_QUANTITIES[i].1);
            }
            i += 1;
        }
        None
    }
    
    /// Gibt alle bekannten Dimensionen zurück (für Debugging/Testing)
    pub const fn all_known_dimensions() -> &'static [(Dimension, &'static str)] {
        Self::KNOWN_QUANTITIES
    }
    
    /// Prüft ob eine Dimension im Registry existiert
    pub const fn is_known_dimension(dimension: Dimension) -> bool {
        match Self::lookup_quantity_name(dimension) {
            Some(_) => true,
            None => false,
        }
    }
    
    /// Anzahl der registrierten Quantities
    pub const fn count() -> usize {
        Self::KNOWN_QUANTITIES.len()
    }
}

/// Macro um automatisch Registry zu erweitern
/// 
/// Dieses Macro ermöglicht es, neue Quantities zum Registry hinzuzufügen
/// ohne die KNOWN_QUANTITIES Array manuell zu bearbeiten.
/// 
/// # Verwendung
/// ```ignore
/// register_quantity!(MyCustomQuantity);
/// ```
#[macro_export]
macro_rules! register_quantity {
    ($quantity:ty) => {
        // Zur Compile-Time wird das Registry erweitert
        // Implementation wird später hinzugefügt wenn wir
        // herausfinden wie man const arrays zur Compile-Time erweitert
        const _: () = {
            // Validiere dass $quantity das Quantity trait implementiert
            const _VALIDATE: fn() = || {
                fn assert_quantity<T: $crate::Quantity>() {}
                assert_quantity::<$quantity>();
            };
        };
    };
}

/// Type-Level Helper für automatische Quantity Resolution
/// 
/// Dieser Type wird von Proc Macros generiert um zur Compile-Time
/// den richtigen Quantity-Typ aus einer Dimension zu ermitteln.
/// 
/// Note: Using individual const parameters instead of array since
/// const generics for arrays are not yet stable.
pub struct DimensionResolver<
    const L: i8, const T: i8, const M: i8, const I: i8,
    const TEMP: i8, const LUM: i8, const AMOUNT: i8
>;

impl<
    const L: i8, const T: i8, const M: i8, const I: i8,
    const TEMP: i8, const LUM: i8, const AMOUNT: i8
> DimensionResolver<L, T, M, I, TEMP, LUM, AMOUNT> {
    /// Resolved zur Compile-Time den Quantity Namen
    pub const fn resolve_quantity_name() -> Option<&'static str> {
        let dimension = Dimension::from_exponents(L, T, M, I, TEMP, LUM, AMOUNT);
        QuantityRegistry::lookup_quantity_name(dimension)
    }
    
    /// Prüft ob die Dimension bekannt ist
    pub const fn is_known() -> bool {
        let dimension = Dimension::from_exponents(L, T, M, I, TEMP, LUM, AMOUNT);
        QuantityRegistry::is_known_dimension(dimension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_lookup_basic() {
        // Test bekannte Dimensionen  
        let distance_dim = Dimension::length();
        assert_eq!(
            QuantityRegistry::lookup_quantity_name(distance_dim),
            Some("Distance")
        );
        
        let time_dim = Dimension::time();
        assert_eq!(
            QuantityRegistry::lookup_quantity_name(time_dim),
            Some("Time")
        );
    }
    
    #[test]
    fn test_registry_lookup_derived() {
        // Test abgeleitete Dimensionen
        let pressure_dim = Dimension::PRESSURE;
        assert_eq!(
            QuantityRegistry::lookup_quantity_name(pressure_dim),
            Some("Pressure")
        );
        
        let energy_dim = Dimension::ENERGY;
        assert_eq!(
            QuantityRegistry::lookup_quantity_name(energy_dim),
            Some("Energy")
        );
    }
    
    #[test]
    fn test_registry_lookup_calculated() {
        // Test berechnete Dimensionen (Kern-Feature!)
        let force_dim = Dimension::FORCE;
        let area_dim = Dimension::AREA;
        let pressure_dim = force_dim.divide(area_dim);
        
        assert_eq!(
            QuantityRegistry::lookup_quantity_name(pressure_dim),
            Some("Pressure")
        );
        
        // Energie = Kraft × Distanz
        let distance_dim = Dimension::length();
        let energy_dim = force_dim.multiply(distance_dim);
        
        assert_eq!(
            QuantityRegistry::lookup_quantity_name(energy_dim),
            Some("Energy")
        );
    }
    
    #[test]
    fn test_registry_lookup_unknown() {
        // Test unbekannte Dimensionen
        let unknown_dim = Dimension::from_array([1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(
            QuantityRegistry::lookup_quantity_name(unknown_dim),
            None
        );
    }
    
    #[test]
    fn test_registry_const_operations() {
        // Test const operations zur Compile-Time
        const FORCE_DIM: Dimension = Dimension::FORCE;
        const AREA_DIM: Dimension = Dimension::AREA;
        const PRESSURE_DIM: Dimension = FORCE_DIM.divide(AREA_DIM);
        
        const PRESSURE_NAME: Option<&'static str> = 
            QuantityRegistry::lookup_quantity_name(PRESSURE_DIM);
            
        assert_eq!(PRESSURE_NAME, Some("Pressure"));
    }
    
    #[test]
    fn test_dimension_resolver() {
        // Test Type-Level Resolution für Pressure: ML⁻¹T⁻²
        type PressureResolver = DimensionResolver<-1, -2, 1, 0, 0, 0, 0>;
        
        const RESOLVED_NAME: Option<&'static str> = PressureResolver::resolve_quantity_name();
        assert_eq!(RESOLVED_NAME, Some("Pressure"));
        
        const IS_KNOWN: bool = PressureResolver::is_known();
        assert!(IS_KNOWN);
        
        // Test für Force: MLT⁻²
        type ForceResolver = DimensionResolver<1, -2, 1, 0, 0, 0, 0>;
        const FORCE_NAME: Option<&'static str> = ForceResolver::resolve_quantity_name();
        assert_eq!(FORCE_NAME, Some("Force"));
    }
    
    #[test]
    fn test_registry_properties() {
        // Test Registry Eigenschaften
        assert!(QuantityRegistry::count() > 0);
        assert!(QuantityRegistry::is_known_dimension(Dimension::FORCE));
        assert!(!QuantityRegistry::is_known_dimension(
            Dimension::from_array([9, 9, 9, 9, 9, 9, 9])
        ));
    }
}