use crate::scalar::macros::{define_quantity, define_unit};

define_quantity!(Temperature);

define_unit!(Kelvin: Temperature, scale = 1.0, prefixable);
