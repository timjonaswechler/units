use crate::scalar::macros::{define_quantity, define_unit};
use crate::scalar::{Kilo, Prefixed};

define_quantity!(Mass);

// The canonical SI mass scale is the kilogram. Gram is used as the
// prefixable unit so Kilogram composes naturally as Kilo + Gram.
define_unit!(Gram: Mass, scale = 1e-3, prefixable);

define_unit!(
    /// Fixed solar-mass conversion based on the IAU 2015 nominal solar mass
    /// parameter divided by the CODATA 2022 central value
    /// `G = 6.67430e-11 m³ kg⁻¹ s⁻²`.
    ///
    /// This is a versioned conversion convention with uncertainty inherited
    /// from `G`, not an exact measurement of the Sun's mass.
    SolarMass: Mass,
    scale = 1.988_409_870_698_051e30
);
define_unit!(
    /// Fixed Earth-mass conversion based on the IAU 2015 nominal terrestrial
    /// mass parameter divided by the CODATA 2022 central value
    /// `G = 6.67430e-11 m³ kg⁻¹ s⁻²`.
    ///
    /// This is a versioned conversion convention with uncertainty inherited
    /// from `G`, not an exact measurement of Earth's mass.
    EarthMass: Mass,
    scale = 5.972_167_867_791_379e24
);

/// Kilogram, the canonical SI mass scale.
pub type Kilogram = Prefixed<Kilo, Gram>;
