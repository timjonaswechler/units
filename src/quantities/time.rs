use crate::macros::{define_quantity, define_unit};
use crate::{Milli, Prefixed};

define_quantity!(Time);

define_unit!(Second: Time, scale = 1.0, prefixable);
define_unit!(Minute: Time, scale = 60.0);
define_unit!(Hour: Time, scale = 3_600.0);
define_unit!(Day: Time, scale = 86_400.0);
define_unit!(JulianYear: Time, scale = 31_557_600.0);

/// One thousandth of a second.
pub type Millisecond = Prefixed<Milli, Second>;
