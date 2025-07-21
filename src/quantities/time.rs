#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units, features::DefaultFloat};

// Conversion constants to seconds
const SECONDS_PER_MINUTE: DefaultFloat = 60.0;
const SECONDS_PER_HOUR: DefaultFloat = 3600.0;
const SECONDS_PER_DAY: DefaultFloat = 86400.0;
const SECONDS_PER_YEAR: DefaultFloat = 31557600.0; // Julian year

define_quantity!(Time); // Time

// Define Time units with astronomical focus
define_units! {
    base_unit: Second = 1.0,
    units: {
        Minute = SECONDS_PER_MINUTE,
        Hour = SECONDS_PER_HOUR,
        Day = SECONDS_PER_DAY,
        Year = SECONDS_PER_YEAR,
    }
}
