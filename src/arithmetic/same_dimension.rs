//! Arithmetic operations for quantities of the same dimension

use crate::core::{Quantity, UnitComposition};
use std::ops::{Add, Sub, AddAssign, SubAssign};

// Note: Same-dimension arithmetic is now handled by mixed_units.rs
// which supports both same and different units through the unified implementation.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantities::*;

    #[test]
    fn test_same_unit_addition() {
        let d1 = Distance::<Meter>::new(100.0);
        let d2 = Distance::<Meter>::new(50.0);
        let sum = d1 + d2;  // Result will be in SI base units ()
        
        assert_eq!(sum.value(), 150.0);  // 100 m + 50 m = 150 m (SI)
    }

    #[test]
    fn test_same_unit_subtraction() {
        let d1 = Distance::<Meter>::new(100.0);
        let d2 = Distance::<Meter>::new(30.0);
        let diff = d1 - d2;  // Result will be in SI base units ()
        
        assert_eq!(diff.value(), 70.0);  // 100 m - 30 m = 70 m (SI)
    }

    #[test]
    fn test_add_assign() {
        let mut d = Distance::<Meter>::new(100.0);
        d += Distance::<Meter>::new(50.0);
        
        assert_eq!(d.value(), 150.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut d = Distance::<Meter>::new(100.0);
        d -= Distance::<Meter>::new(30.0);
        
        assert_eq!(d.value(), 70.0);
    }
}