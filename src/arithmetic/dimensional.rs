//! Dimensional analysis arithmetic operations
//!
//! This module implements automatic dimensional analysis through operator overloading.
//! When you multiply or divide quantities, the resulting type is automatically inferred
//! based on the dimensional exponents.

use crate::core::{Quantity, UnitComposition};
use std::ops::{Mul, Div};

// =================================================================================================
// DIVISION OPERATIONS (Dimensional Analysis)
// =================================================================================================

/// Division: Distance / Time = Velocity
/// 
/// L¹ / T¹ = L¹T⁻¹ (Velocity)
impl<U1, U2> Div<Quantity<U2, 0, 0, 1, 0, 0, 0, 0>> for Quantity<U1, 1, 0, 0, 0, 0, 0, 0>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    type Output = Quantity<(U1, U2), 1, 0, -1, 0, 0, 0, 0>; // Velocity

    fn div(self, rhs: Quantity<U2, 0, 0, 1, 0, 0, 0, 0>) -> Self::Output {
        let distance_si = self.value * U1::to_si_factor();
        let time_si = rhs.value * U2::to_si_factor();
        let result_si = distance_si / time_si;
        
        // Result unit factor: distance_unit / time_unit
        let result_factor = U1::to_si_factor() / U2::to_si_factor();
        let result_value = result_si / result_factor;
        
        Quantity::new(result_value)
    }
}

/// Division: Velocity / Time = Acceleration
/// 
/// L¹T⁻¹ / T¹ = L¹T⁻² (Acceleration)  
impl<U1, U2> Div<Quantity<U2, 0, 0, 1, 0, 0, 0, 0>> for Quantity<U1, 1, 0, -1, 0, 0, 0, 0>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    type Output = Quantity<(U1, U2), 1, 0, -2, 0, 0, 0, 0>; // Acceleration

    fn div(self, rhs: Quantity<U2, 0, 0, 1, 0, 0, 0, 0>) -> Self::Output {
        let velocity_si = self.value * U1::to_si_factor();
        let time_si = rhs.value * U2::to_si_factor();
        let result_si = velocity_si / time_si;
        
        let result_factor = U1::to_si_factor() / U2::to_si_factor();
        let result_value = result_si / result_factor;
        
        Quantity::new(result_value)
    }
}

/// Division: Same dimensions = Dimensionless ratio
/// 
/// Generic implementation for any dimension divided by itself
impl<U1, U2, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    Div<Quantity<U2, L, M, T, K, I, J, N>> for Quantity<U1, L, M, T, K, I, J, N>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    type Output = f64; // Dimensionless result

    fn div(self, rhs: Quantity<U2, L, M, T, K, I, J, N>) -> Self::Output {
        let lhs_si = self.value * U1::to_si_factor();
        let rhs_si = rhs.value * U2::to_si_factor();
        lhs_si / rhs_si
    }
}

// =================================================================================================
// MULTIPLICATION OPERATIONS (Dimensional Analysis)
// =================================================================================================

/// Multiplication: Mass * Acceleration = Force
/// 
/// M¹ * L¹T⁻² = L¹M¹T⁻² (Force)
impl<U1, U2> Mul<Quantity<U2, 1, 0, -2, 0, 0, 0, 0>> for Quantity<U1, 0, 1, 0, 0, 0, 0, 0>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    type Output = Quantity<(U1, U2), 1, 1, -2, 0, 0, 0, 0>; // Force

    fn mul(self, rhs: Quantity<U2, 1, 0, -2, 0, 0, 0, 0>) -> Self::Output {
        let mass_si = self.value * U1::to_si_factor();
        let acceleration_si = rhs.value * U2::to_si_factor();
        let result_si = mass_si * acceleration_si;
        
        let result_factor = U1::to_si_factor() * U2::to_si_factor();
        let result_value = result_si / result_factor;
        
        Quantity::new(result_value)
    }
}

/// Multiplication: Force * Distance = Energy (Work)
/// 
/// L¹M¹T⁻² * L¹ = L²M¹T⁻² (Energy)
impl<U1, U2> Mul<Quantity<U2, 1, 0, 0, 0, 0, 0, 0>> for Quantity<U1, 1, 1, -2, 0, 0, 0, 0>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    type Output = Quantity<(U1, U2), 2, 1, -2, 0, 0, 0, 0>; // Energy

    fn mul(self, rhs: Quantity<U2, 1, 0, 0, 0, 0, 0, 0>) -> Self::Output {
        let force_si = self.value * U1::to_si_factor();
        let distance_si = rhs.value * U2::to_si_factor();
        let result_si = force_si * distance_si;
        
        let result_factor = U1::to_si_factor() * U2::to_si_factor();
        let result_value = result_si / result_factor;
        
        Quantity::new(result_value)
    }
}

/// Multiplication: Distance * Distance = Area
/// 
/// L¹ * L¹ = L² (Area)
impl<U1, U2> Mul<Quantity<U2, 1, 0, 0, 0, 0, 0, 0>> for Quantity<U1, 1, 0, 0, 0, 0, 0, 0>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    type Output = Quantity<(U1, U2), 2, 0, 0, 0, 0, 0, 0>; // Area

    fn mul(self, rhs: Quantity<U2, 1, 0, 0, 0, 0, 0, 0>) -> Self::Output {
        let lhs_si = self.value * U1::to_si_factor();
        let rhs_si = rhs.value * U2::to_si_factor();
        let result_si = lhs_si * rhs_si;
        
        let result_factor = U1::to_si_factor() * U2::to_si_factor();
        let result_value = result_si / result_factor;
        
        Quantity::new(result_value)
    }
}

/// Multiplication: Area * Distance = Volume
/// 
/// L² * L¹ = L³ (Volume)
impl<U1, U2> Mul<Quantity<U2, 1, 0, 0, 0, 0, 0, 0>> for Quantity<U1, 2, 0, 0, 0, 0, 0, 0>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    type Output = Quantity<(U1, U2), 3, 0, 0, 0, 0, 0, 0>; // Volume

    fn mul(self, rhs: Quantity<U2, 1, 0, 0, 0, 0, 0, 0>) -> Self::Output {
        let area_si = self.value * U1::to_si_factor();
        let distance_si = rhs.value * U2::to_si_factor();
        let result_si = area_si * distance_si;
        
        let result_factor = U1::to_si_factor() * U2::to_si_factor();
        let result_value = result_si / result_factor;
        
        Quantity::new(result_value)
    }
}

/// Multiplication: Mass * Velocity = Momentum
/// 
/// M¹ * L¹T⁻¹ = L¹M¹T⁻¹ (Momentum)
impl<U1, U2> Mul<Quantity<U2, 1, 0, -1, 0, 0, 0, 0>> for Quantity<U1, 0, 1, 0, 0, 0, 0, 0>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    type Output = Quantity<(U1, U2), 1, 1, -1, 0, 0, 0, 0>; // Momentum

    fn mul(self, rhs: Quantity<U2, 1, 0, -1, 0, 0, 0, 0>) -> Self::Output {
        let mass_si = self.value * U1::to_si_factor();
        let velocity_si = rhs.value * U2::to_si_factor();
        let result_si = mass_si * velocity_si;
        
        let result_factor = U1::to_si_factor() * U2::to_si_factor();
        let result_value = result_si / result_factor;
        
        Quantity::new(result_value)
    }
}

/// Multiplication: Energy / Time = Power
/// 
/// L²M¹T⁻² / T¹ = L²M¹T⁻³ (Power)
impl<U1, U2> Div<Quantity<U2, 0, 0, 1, 0, 0, 0, 0>> for Quantity<U1, 2, 1, -2, 0, 0, 0, 0>
where
    U1: UnitComposition,
    U2: UnitComposition,
{
    type Output = Quantity<(U1, U2), 2, 1, -3, 0, 0, 0, 0>; // Power

    fn div(self, rhs: Quantity<U2, 0, 0, 1, 0, 0, 0, 0>) -> Self::Output {
        let energy_si = self.value * U1::to_si_factor();
        let time_si = rhs.value * U2::to_si_factor();
        let result_si = energy_si / time_si;
        
        let result_factor = U1::to_si_factor() / U2::to_si_factor();
        let result_value = result_si / result_factor;
        
        Quantity::new(result_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantities::*;
    use crate::Area;

    #[test]
    fn test_distance_divided_by_time_equals_velocity() {
        let distance = Distance::<Meter>::new(100.0);
        let time = Time::<Second>::new(10.0);
        let velocity = distance / time;
        
        // Result should be 10.0 in the composed unit (Meter, Second)
        assert_eq!(velocity.value(), 10.0);
    }

    #[test]
    fn test_velocity_divided_by_time_equals_acceleration() {
        let velocity = Velocity::<(Meter, Second)>::new(20.0);
        let time = Time::<Second>::new(4.0);
        let acceleration = velocity / time;
        
        // Result should be 5.0 m/s²
        assert_eq!(acceleration.value(), 5.0);
    }

    #[test]
    fn test_mass_times_acceleration_equals_force() {
        let mass = Mass::<Kilogram>::new(5.0);
        let acceleration = Acceleration::<(Meter, Second)>::new(2.0);
        let force = mass * acceleration;
        
        // Result should be 10.0 N
        assert_eq!(force.value(), 10.0);
    }

    #[test]
    fn test_distance_times_distance_equals_area() {
        let length = Distance::<Meter>::new(5.0);
        let width = Distance::<Meter>::new(3.0);
        let area = length * width;
        
        // Result should be 15.0 m²
        assert_eq!(area.value(), 15.0);
    }

    #[test]
    fn test_area_times_distance_equals_volume() {
        let area = Area::<(Meter, Meter)>::new(20.0);
        let height = Distance::<Meter>::new(3.0);
        let volume = area * height;
        
        // Result should be 60.0 m³
        assert_eq!(volume.value(), 60.0);
    }

    #[test]
    fn test_mass_times_velocity_equals_momentum() {
        let mass = Mass::<Kilogram>::new(2.0);
        let velocity = Velocity::<(Meter, Second)>::new(10.0);
        let momentum = mass * velocity;
        
        // Result should be 20.0 kg⋅m/s
        assert_eq!(momentum.value(), 20.0);
    }

    #[test]
    fn test_dimensionless_division() {
        let d1 = Distance::<Meter>::new(100.0);
        let d2 = Distance::<Meter>::new(50.0);
        let ratio = d1 / d2;
        
        // Result should be 2.0 (dimensionless)
        assert_eq!(ratio, 2.0);
    }

    #[test]
    fn test_different_units_dimensional_analysis() {
        let distance = Distance::<Kilometer>::new(1.0);  // 1 km = 1000 m
        let time = Time::<Hour>::new(1.0);               // 1 h = 3600 s
        let velocity = distance / time;
        
        // 1 km / 1 h should give the correct ratio
        // The result should be in the compound unit (Kilometer, Hour)
        assert_eq!(velocity.value(), 1.0);
    }
}