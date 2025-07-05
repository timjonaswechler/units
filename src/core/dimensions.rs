//! Dimensional constants and type definitions

use crate::core::Quantity;

/// Length dimension: L¹
pub type Length<U> = Quantity<U, 1, 0, 0, 0, 0, 0, 0>;

/// Mass dimension: M¹  
pub type Mass<U> = Quantity<U, 0, 1, 0, 0, 0, 0, 0>;

/// Time dimension: T¹
pub type Time<U> = Quantity<U, 0, 0, 1, 0, 0, 0, 0>;

/// Temperature dimension: K¹
pub type Temperature<U> = Quantity<U, 0, 0, 0, 1, 0, 0, 0>;

/// Electric current dimension: I¹
pub type Current<U> = Quantity<U, 0, 0, 0, 0, 1, 0, 0>;

/// Luminous intensity dimension: J¹
pub type LuminousIntensity<U> = Quantity<U, 0, 0, 0, 0, 0, 1, 0>;

/// Amount of substance dimension: N¹
pub type AmountOfSubstance<U> = Quantity<U, 0, 0, 0, 0, 0, 0, 1>;

/// Angle dimension: dimensionless (radian-based angular measurements)
pub type Angle<U> = Quantity<U, 0, 0, 0, 0, 0, 0, 0>;

// Common alias
pub type Distance<U> = Length<U>;

// Derived dimensions

/// Area dimension: L²
pub type Area<U> = Quantity<U, 2, 0, 0, 0, 0, 0, 0>;

/// Volume dimension: L³
pub type Volume<U> = Quantity<U, 3, 0, 0, 0, 0, 0, 0>;

/// Velocity dimension: L¹T⁻¹
pub type Velocity<U> = Quantity<U, 1, 0, -1, 0, 0, 0, 0>;

/// Acceleration dimension: L¹T⁻²
pub type Acceleration<U> = Quantity<U, 1, 0, -2, 0, 0, 0, 0>;

/// Force dimension: L¹M¹T⁻² (Newton's second law: F = ma)
pub type Force<U> = Quantity<U, 1, 1, -2, 0, 0, 0, 0>;

/// Energy dimension: L²M¹T⁻² (E = mc², kinetic energy, etc.)
pub type Energy<U> = Quantity<U, 2, 1, -2, 0, 0, 0, 0>;

/// Power dimension: L²M¹T⁻³ (P = E/t)
pub type Power<U> = Quantity<U, 2, 1, -3, 0, 0, 0, 0>;

/// Pressure dimension: L⁻¹M¹T⁻² (P = F/A)
pub type Pressure<U> = Quantity<U, -1, 1, -2, 0, 0, 0, 0>;

/// Frequency dimension: T⁻¹
pub type Frequency<U> = Quantity<U, 0, 0, -1, 0, 0, 0, 0>;

/// Density dimension: L⁻³M¹
pub type Density<U> = Quantity<U, -3, 1, 0, 0, 0, 0, 0>;

/// Momentum dimension: L¹M¹T⁻¹
pub type Momentum<U> = Quantity<U, 1, 1, -1, 0, 0, 0, 0>;

/// Angular velocity dimension: T⁻¹ (radians per time)
pub type AngularVelocity<U> = Quantity<U, 0, 0, -1, 0, 0, 0, 0>;

/// Angular acceleration dimension: T⁻² (radians per time squared)
pub type AngularAcceleration<U> = Quantity<U, 0, 0, -2, 0, 0, 0, 0>;

/// Electric charge dimension: I¹T¹
pub type ElectricCharge<U> = Quantity<U, 0, 0, 1, 0, 1, 0, 0>;

/// Voltage dimension: L²M¹T⁻³I⁻¹
pub type Voltage<U> = Quantity<U, 2, 1, -3, 0, -1, 0, 0>;

/// Resistance dimension: L²M¹T⁻³I⁻²
pub type Resistance<U> = Quantity<U, 2, 1, -3, 0, -2, 0, 0>;

/// Capacitance dimension: L⁻²M⁻¹T⁴I²
pub type Capacitance<U> = Quantity<U, -2, -1, 4, 0, 2, 0, 0>;

/// Magnetic field dimension: M¹T⁻²I⁻¹
pub type MagneticField<U> = Quantity<U, 0, 1, -2, 0, -1, 0, 0>;

/// Magnetic flux dimension: L²M¹T⁻²I⁻¹
pub type MagneticFlux<U> = Quantity<U, 2, 1, -2, 0, -1, 0, 0>;

/// Luminosity dimension: L²M¹T⁻³ (power/energy flux, same as Power)
pub type Luminosity<U> = Quantity<U, 2, 1, -3, 0, 0, 0, 0>;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestUnit;

    #[test]
    fn test_base_dimensions() {
        let _length: Length<TestUnit> = Length::new(1.0);
        let _mass: Mass<TestUnit> = Mass::new(1.0);
        let _time: Time<TestUnit> = Time::new(1.0);
    }

    #[test]
    fn test_derived_dimensions() {
        let _velocity: Velocity<TestUnit> = Velocity::new(1.0);
        let _force: Force<TestUnit> = Force::new(1.0);
        let _energy: Energy<TestUnit> = Energy::new(1.0);
    }
}