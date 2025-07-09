//! Test for dimensional validation system
//!
//! This module demonstrates the type safety provided by our dimensional validation system.

#[cfg(test)]
mod tests {
    use crate::quantities::{Distance, Time, Velocity, Meter, Second};

    #[test]
    fn test_valid_distance_creation() {
        // This should work - Meter has correct dimensions for Distance
        let distance = Distance::<Meter>::new(100.0);
        assert_eq!(distance.value(), 100.0);
        println!("✅ Successfully created distance: {} meters", distance.value());
    }

    #[test]
    fn test_valid_time_creation() {
        // This should work - Second has correct dimensions for Time
        let time = Time::<Second>::new(10.0);
        assert_eq!(time.value(), 10.0);
        println!("✅ Successfully created time: {} seconds", time.value());
    }

    #[test]
    #[should_panic(expected = "DIMENSIONAL MISMATCH")]
    fn test_invalid_distance_creation() {
        // This should panic - Second has wrong dimensions for Distance
        // Second has dimensions [L=0, M=0, T=1, ...]
        // Distance expects [L=1, M=0, T=0, ...]
        let _distance = Distance::<Second>::new(100.0);
    }

    #[test]
    #[should_panic(expected = "DIMENSIONAL MISMATCH")]
    fn test_invalid_time_creation() {
        // This should panic - Meter has wrong dimensions for Time
        // Meter has dimensions [L=1, M=0, T=0, ...]
        // Time expects [L=0, M=0, T=1, ...]
        let _time = Time::<Meter>::new(10.0);
    }

    #[test]
    fn test_quantity_info() {
        // Test the helper methods
        assert_eq!(Distance::<Meter>::dimensions(), [1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(Time::<Second>::dimensions(), [0, 0, 1, 0, 0, 0, 0]);
        
        assert_eq!(Distance::<Meter>::quantity_name(), "Distance");
        assert_eq!(Time::<Second>::quantity_name(), "Time");
        
        println!("✅ Distance dimensions: {:?}", Distance::<Meter>::dimensions());
        println!("✅ Time dimensions: {:?}", Time::<Second>::dimensions());
    }

    #[test]
    fn test_unit_conversion() {
        use crate::quantities::AstronomicalUnit;
        
        // Create a distance in meters
        let distance_m = Distance::<Meter>::new(1.495978707e11);
        
        // Convert to AU (this should work as both have L=1 dimensions)
        let distance_au = distance_m.to::<AstronomicalUnit>();
        
        // The conversion should give approximately 1 AU
        println!("✅ {} meters = {} AU", distance_m.value(), distance_au.value());
        assert!((distance_au.value() - 1.0).abs() < 0.01);
    }
}

#[cfg(test)]
mod demonstration {
    //! This module shows examples of the type safety in action

    use crate::quantities::{Distance, Time, Meter, Second, AstronomicalUnit};

    #[test]
    fn demonstrate_type_safety() {
        println!("\n🔬 DIMENSIONAL VALIDATION DEMONSTRATION");
        println!("======================================");
        
        // ✅ Valid constructions
        println!("\n✅ Valid quantity creations:");
        let distance = Distance::<Meter>::new(100.0);
        let time = Time::<Second>::new(10.0);
        println!("  Distance<Meter>: {} m", distance.value());
        println!("  Time<Second>: {} s", time.value());
        
        // ✅ Valid unit conversions
        println!("\n✅ Valid unit conversions:");
        let distance_au = Distance::<AstronomicalUnit>::new(1.0);
        let distance_m = distance_au.to::<Meter>();
        println!("  1 AU = {:.2e} meters", distance_m.value());
        
        // 🚨 The following would panic at runtime due to dimensional mismatches:
        // let invalid_distance = Distance::<Second>::new(100.0);  // Second is T=1, Distance needs L=1
        // let invalid_time = Time::<Meter>::new(10.0);            // Meter is L=1, Time needs T=1
        
        println!("\n✅ All type-safe operations completed successfully!");
        println!("🚨 Try uncommenting the invalid lines to see dimensional validation in action!");
    }
}