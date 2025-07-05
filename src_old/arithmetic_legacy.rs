//! Dynamic dimensional arithmetic for the unit system.
//!
//! This module provides a macro-based system for generating arithmetic operations
//! with automatic dimensional analysis, eliminating the need for hard-coded
//! operation implementations.

use crate::core::*;
use std::ops::{Mul, Div};

/// Macro to generate multiplication implementations for dimensional arithmetic.
///
/// This macro automatically generates the correct result dimensions by adding
/// the dimensional exponents of the operands.
macro_rules! impl_dimensional_mul {
    (
        ($l1:expr, $m1:expr, $t1:expr, $k1:expr, $i1:expr, $j1:expr, $n1:expr),
        ($l2:expr, $m2:expr, $t2:expr, $k2:expr, $i2:expr, $j2:expr, $n2:expr)
        => ($lr:expr, $mr:expr, $tr:expr, $kr:expr, $ir:expr, $jr:expr, $nr:expr)
    ) => {
        impl<Unit1, Unit2>
            Mul<Quantity<Unit2, $l2, $m2, $t2, $k2, $i2, $j2, $n2>>
            for Quantity<Unit1, $l1, $m1, $t1, $k1, $i1, $j1, $n1>
        where
            Self: ToSI,
            Quantity<Unit2, $l2, $m2, $t2, $k2, $i2, $j2, $n2>: ToSI,
        {
            type Output = Quantity<Unit1, $lr, $mr, $tr, $kr, $ir, $jr, $nr>;
            
            fn mul(self, rhs: Quantity<Unit2, $l2, $m2, $t2, $k2, $i2, $j2, $n2>) -> Self::Output {
                let result_si = self.to_si() * rhs.to_si();
                Quantity::<Unit1, $lr, $mr, $tr, $kr, $ir, $jr, $nr>::new(result_si)
            }
        }
    };
}

/// Macro to generate division implementations for dimensional arithmetic.
///
/// This macro automatically generates the correct result dimensions by subtracting
/// the dimensional exponents of the divisor from the dividend.
macro_rules! impl_dimensional_div {
    (
        ($l1:expr, $m1:expr, $t1:expr, $k1:expr, $i1:expr, $j1:expr, $n1:expr),
        ($l2:expr, $m2:expr, $t2:expr, $k2:expr, $i2:expr, $j2:expr, $n2:expr)
        => ($lr:expr, $mr:expr, $tr:expr, $kr:expr, $ir:expr, $jr:expr, $nr:expr)
    ) => {
        impl<Unit1, Unit2>
            Div<Quantity<Unit2, $l2, $m2, $t2, $k2, $i2, $j2, $n2>>
            for Quantity<Unit1, $l1, $m1, $t1, $k1, $i1, $j1, $n1>
        where
            Self: ToSI,
            Quantity<Unit2, $l2, $m2, $t2, $k2, $i2, $j2, $n2>: ToSI,
        {
            type Output = Quantity<Unit1, $lr, $mr, $tr, $kr, $ir, $jr, $nr>;
            
            fn div(self, rhs: Quantity<Unit2, $l2, $m2, $t2, $k2, $i2, $j2, $n2>) -> Self::Output {
                let result_si = self.to_si() / rhs.to_si();
                Quantity::<Unit1, $lr, $mr, $tr, $kr, $ir, $jr, $nr>::new(result_si)
            }
        }
    };
}

// ================================================================================================
// MULTIPLICATION IMPLEMENTATIONS
// ================================================================================================

// Distance × Distance = Area
impl_dimensional_mul!(
    (1, 0, 0, 0, 0, 0, 0), // Distance
    (1, 0, 0, 0, 0, 0, 0)  // Distance
    => (2, 0, 0, 0, 0, 0, 0) // Area
);

// Area × Distance = Volume
impl_dimensional_mul!(
    (2, 0, 0, 0, 0, 0, 0), // Area
    (1, 0, 0, 0, 0, 0, 0)  // Distance
    => (3, 0, 0, 0, 0, 0, 0) // Volume
);

// Distance × Area = Volume (commutative)
impl_dimensional_mul!(
    (1, 0, 0, 0, 0, 0, 0), // Distance
    (2, 0, 0, 0, 0, 0, 0)  // Area
    => (3, 0, 0, 0, 0, 0, 0) // Volume
);

// Mass × Acceleration = Force
impl_dimensional_mul!(
    (0, 1, 0, 0, 0, 0, 0), // Mass
    (1, 0, -2, 0, 0, 0, 0) // Acceleration
    => (1, 1, -2, 0, 0, 0, 0) // Force
);

// Acceleration × Mass = Force (commutative)
impl_dimensional_mul!(
    (1, 0, -2, 0, 0, 0, 0), // Acceleration
    (0, 1, 0, 0, 0, 0, 0)   // Mass
    => (1, 1, -2, 0, 0, 0, 0) // Force
);

// Force × Distance = Energy
impl_dimensional_mul!(
    (1, 1, -2, 0, 0, 0, 0), // Force
    (1, 0, 0, 0, 0, 0, 0)   // Distance
    => (2, 1, -2, 0, 0, 0, 0) // Energy
);

// Distance × Force = Energy (commutative)
impl_dimensional_mul!(
    (1, 0, 0, 0, 0, 0, 0),  // Distance
    (1, 1, -2, 0, 0, 0, 0)  // Force
    => (2, 1, -2, 0, 0, 0, 0) // Energy
);


// Mass × Velocity = Momentum
impl_dimensional_mul!(
    (0, 1, 0, 0, 0, 0, 0), // Mass
    (1, 0, -1, 0, 0, 0, 0) // Velocity
    => (1, 1, -1, 0, 0, 0, 0) // Momentum
);

// Velocity × Mass = Momentum (commutative)
impl_dimensional_mul!(
    (1, 0, -1, 0, 0, 0, 0), // Velocity
    (0, 1, 0, 0, 0, 0, 0)   // Mass
    => (1, 1, -1, 0, 0, 0, 0) // Momentum
);

// ================================================================================================
// DIVISION IMPLEMENTATIONS
// ================================================================================================

// Distance ÷ Time = Velocity
impl_dimensional_div!(
    (1, 0, 0, 0, 0, 0, 0), // Distance
    (0, 0, 1, 0, 0, 0, 0)  // Time
    => (1, 0, -1, 0, 0, 0, 0) // Velocity
);

// Velocity ÷ Time = Acceleration
impl_dimensional_div!(
    (1, 0, -1, 0, 0, 0, 0), // Velocity
    (0, 0, 1, 0, 0, 0, 0)   // Time
    => (1, 0, -2, 0, 0, 0, 0) // Acceleration
);

// Distance ÷ Distance = Dimensionless
impl_dimensional_div!(
    (1, 0, 0, 0, 0, 0, 0), // Distance
    (1, 0, 0, 0, 0, 0, 0)  // Distance
    => (0, 0, 0, 0, 0, 0, 0) // Dimensionless
);

// Area ÷ Distance = Distance
impl_dimensional_div!(
    (2, 0, 0, 0, 0, 0, 0), // Area
    (1, 0, 0, 0, 0, 0, 0)  // Distance
    => (1, 0, 0, 0, 0, 0, 0) // Distance
);

// Volume ÷ Area = Distance
impl_dimensional_div!(
    (3, 0, 0, 0, 0, 0, 0), // Volume
    (2, 0, 0, 0, 0, 0, 0)  // Area
    => (1, 0, 0, 0, 0, 0, 0) // Distance
);

// Volume ÷ Distance = Area
impl_dimensional_div!(
    (3, 0, 0, 0, 0, 0, 0), // Volume
    (1, 0, 0, 0, 0, 0, 0)  // Distance
    => (2, 0, 0, 0, 0, 0, 0) // Area
);

// Mass ÷ Volume = Density
impl_dimensional_div!(
    (0, 1, 0, 0, 0, 0, 0), // Mass
    (3, 0, 0, 0, 0, 0, 0)  // Volume
    => (-3, 1, 0, 0, 0, 0, 0) // Density
);

// Force ÷ Area = Pressure
impl_dimensional_div!(
    (1, 1, -2, 0, 0, 0, 0), // Force
    (2, 0, 0, 0, 0, 0, 0)   // Area
    => (-1, 1, -2, 0, 0, 0, 0) // Pressure
);

// Energy ÷ Time = Power
impl_dimensional_div!(
    (2, 1, -2, 0, 0, 0, 0), // Energy
    (0, 0, 1, 0, 0, 0, 0)   // Time
    => (2, 1, -3, 0, 0, 0, 0) // Power
);

// Energy ÷ Distance = Force
impl_dimensional_div!(
    (2, 1, -2, 0, 0, 0, 0), // Energy
    (1, 0, 0, 0, 0, 0, 0)   // Distance
    => (1, 1, -2, 0, 0, 0, 0) // Force
);


// Volume ÷ Volume = Dimensionless
impl_dimensional_div!(
    (3, 0, 0, 0, 0, 0, 0), // Volume
    (3, 0, 0, 0, 0, 0, 0)  // Volume
    => (0, 0, 0, 0, 0, 0, 0) // Dimensionless
);

// Area ÷ Area = Dimensionless
impl_dimensional_div!(
    (2, 0, 0, 0, 0, 0, 0), // Area
    (2, 0, 0, 0, 0, 0, 0)  // Area
    => (0, 0, 0, 0, 0, 0, 0) // Dimensionless
);

// Mass ÷ Mass = Dimensionless
impl_dimensional_div!(
    (0, 1, 0, 0, 0, 0, 0), // Mass
    (0, 1, 0, 0, 0, 0, 0)  // Mass
    => (0, 0, 0, 0, 0, 0, 0) // Dimensionless
);

// Time ÷ Time = Dimensionless
impl_dimensional_div!(
    (0, 0, 1, 0, 0, 0, 0), // Time
    (0, 0, 1, 0, 0, 0, 0)  // Time
    => (0, 0, 0, 0, 0, 0, 0) // Dimensionless
);

// TODO: Add more combinations as needed
// This system is extensible - just add more impl_dimensional_mul! and impl_dimensional_div! calls