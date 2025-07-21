#![allow(non_snake_case)]

use crate::quantities::{Acceleration, Distance, Meter, Second, Time};
use crate::{define_quantity, define_units};

define_quantity!(Velocity); // Length/Time

define_units! {
    base_unit: MeterPerSecond = 1.0,
    units: {

    }
}
