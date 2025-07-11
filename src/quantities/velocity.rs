#![allow(non_snake_case)]

use crate::quantities::{Distance, Meter, Second, Time};
use crate::{define_quantity, define_units};

define_quantity!(
    Velocity,
    L = 1,
    M = 0,
    T = -1,
    THETA = 0,
    I = 0,
    J = 0,
    N = 0
); // Length/Time

define_units! {
    dimension: { L = 1, M = 0, T = -1, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: MeterPerSecond = 1.0,
    units: {

    }
}
