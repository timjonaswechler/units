//! Energy units

use crate::*;

// Re-export the type alias from core
pub use crate::core::Energy;

// Define composed energy units as aliases
define_composed_unit!(Joule, "J", 1.0);  // kg⋅m²/s² in SI
define_composed_unit!(Calorie, "cal", 4.184);
define_composed_unit!(Kilocalorie, "kcal", 4184.0);
define_composed_unit!(KilowattHour, "kWh", 3_600_000.0);
define_composed_unit!(Erg, "erg", 1e-7);
define_composed_unit!(FootPound, "ft⋅lbf", 1.355_818);
define_composed_unit!(ElectronVolt, "eV", 1.602_176_634e-19);

// Generate prefixed aliases
define_prefixed_aliases! {
    Joule => [Kilo, Mega, Giga, Milli, Micro],
    ElectronVolt => [Kilo, Mega, Giga],
}

// Convenience constructors
impl_quantity_constructors!(
    Energy,
    Joule, Calorie, Kilocalorie, KilowattHour, Erg, FootPound, ElectronVolt
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_units() {
        let energy1 = Energy::<Joule>::new(1000.0);
        let energy2 = Energy::<(Kilogram, Meter, Second)>::new(1000.0);
        
        assert_eq!(energy1.value(), 1000.0);
        assert_eq!(energy2.value(), 1000.0);
    }

    #[test]
    fn test_energy_conversions() {
        let energy_j = Energy::<Joule>::new(1.0);
        let energy_cal: Energy<Calorie> = energy_j.convert_to();
        
        // 1 J ≈ 0.239 cal
        assert!((energy_cal.value() - 0.239).abs() < 0.01);
    }

    #[test]
    fn test_kilocalorie_conversion() {
        let energy_kcal = Energy::<Kilocalorie>::new(1.0);
        let energy_j: Energy<Joule> = energy_kcal.convert_to();
        
        // 1 kcal = 4184 J
        assert!((energy_j.value() - 4184.0).abs() < 0.1);
    }

    #[test]
    fn test_kwh_conversion() {
        let energy_kwh = Energy::<KilowattHour>::new(1.0);
        let energy_j: Energy<Joule> = energy_kwh.convert_to();
        
        // 1 kWh = 3,600,000 J
        assert!((energy_j.value() - 3_600_000.0).abs() < 1.0);
    }

    #[test]
    fn test_electron_volt_conversion() {
        let energy_ev = Energy::<ElectronVolt>::new(1.0);
        let energy_j: Energy<Joule> = energy_ev.convert_to();
        
        // 1 eV ≈ 1.602e-19 J
        assert!((energy_j.value() - 1.602_176_634e-19).abs() < 1e-27);
    }
}