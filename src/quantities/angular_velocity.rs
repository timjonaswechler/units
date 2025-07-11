#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units};
#[cfg(feature = "f32")]
const RADIANS_PER_DEGREE: f32 = std::f32::consts::PI / 180.0;
#[cfg(feature = "f64")]
const RADIANS_PER_DEGREE: f64 = std::f64::consts::PI / 180.0;
#[cfg(feature = "f128")]
const RADIANS_PER_DEGREE: f128 = std::f128::consts::PI / 180.0;

define_quantity!(
    AngularVelocity,
    L = 0,
    M = 0,
    T = -1,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // 1/Time

// Define AngularVelocity units (angle/time)
define_units! {
    dimension: {L = 0, M = 0, T = -1, THETA = 0, I = 0, J = 0, N = 0},
        base_unit: RadianPerSecond = 1.0,
        units: {

            DegreePerSecond = RADIANS_PER_DEGREE,

    }
}
