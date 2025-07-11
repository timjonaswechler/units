#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units, features::DefaultFloat};

// Conversion constants to seconds
const SECONDS_PER_MINUTE: DefaultFloat = 60.0;
const SECONDS_PER_HOUR: DefaultFloat = 3600.0;
const SECONDS_PER_DAY: DefaultFloat = 86400.0;
const SECONDS_PER_YEAR: DefaultFloat = 31557600.0; // Julian year

define_quantity!(Time, L = 0, M = 0, T = 1, THETA = 0, I = 0, J = 0, N = 0); // Time

// Define Time units with astronomical focus
define_units! {
    dimension: { L = 0, M = 0, T = 1, THETA = 0, I = 0, J = 0, N = 0 },
    base_unit: Second = 1.0,
    units: {
        Minute = SECONDS_PER_MINUTE,
        Hour = SECONDS_PER_HOUR,
        Day = SECONDS_PER_DAY,
        Year = SECONDS_PER_YEAR,
    }
}
