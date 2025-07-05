//! Force units

use crate::*;

// Re-export the type alias from core
pub use crate::core::Force;

// Define composed force units as aliases
define_composed_unit!(Newton, "N", 1.0);  // kg⋅m/s² in SI
define_composed_unit!(Dyne, "dyn", 1e-5); // 1 dyne = 10⁻⁵ N
define_composed_unit!(PoundForce, "lbf", 4.448_222);
define_composed_unit!(Kilonewton, "kN", 1000.0);

// Generate prefixed aliases
define_prefixed_aliases! {
    Newton => [Kilo, Mega, Milli, Micro],
}

// Convenience constructors
impl_quantity_constructors!(
    Force,
    Newton, Dyne, PoundForce, Kilonewton
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_force_units() {
        let force1 = Force::<Newton>::new(100.0);
        let force2 = Force::<(Kilogram, Meter, Second)>::new(100.0);
        
        assert_eq!(force1.value(), 100.0);
        assert_eq!(force2.value(), 100.0);
    }

    #[test]
    fn test_force_conversions() {
        let force_n = Force::<Newton>::new(1.0);
        let force_dyne: Force<Dyne> = force_n.convert_to();
        
        // 1 N = 100,000 dyne
        assert!((force_dyne.value() - 100_000.0).abs() < 0.1);
    }

    #[test]
    fn test_pound_force_conversion() {
        let force_lbf = Force::<PoundForce>::new(1.0);
        let force_n: Force<Newton> = force_lbf.convert_to();
        
        // 1 lbf ≈ 4.448 N
        assert!((force_n.value() - 4.448_222).abs() < 0.001);
    }
}