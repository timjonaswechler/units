use crate::scalar::macros::{define_quantity, define_unit};
use crate::scalar::{Kilo, Prefixed};

define_quantity!(Length);

define_unit!(Meter: Length, scale = 1.0, prefixable);
define_unit!(AstronomicalUnit: Length, scale = 149_597_870_700.0);
define_unit!(LightYear: Length, scale = 9_460_730_472_580_800.0);
define_unit!(Parsec: Length, scale = 3.085_677_581_491_367e16);

/// One thousand meters.
pub type Kilometer = Prefixed<Kilo, Meter>;
