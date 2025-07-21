#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units};
#[cfg(feature = "f32")]
const RADIANS_PER_DEGREE: f32 = std::f32::consts::PI / 180.0;
#[cfg(feature = "f64")]
const RADIANS_PER_DEGREE: f64 = std::f64::consts::PI / 180.0;
#[cfg(feature = "f128")]
const RADIANS_PER_DEGREE: f128 = std::f128::consts::PI / 180.0;

define_quantity!(Angle); // Dimensionless

// Define Angle units (dimensionless but physically important)
define_units! {
    base_unit: Radian = 1.0,
    units: {
        Degree = RADIANS_PER_DEGREE,
        Arcminute = RADIANS_PER_DEGREE / 60.0,
        Arcsecond = RADIANS_PER_DEGREE / 3600.0,
        Milliarcsecond = RADIANS_PER_DEGREE / 3_600_000.0,
    }

}
