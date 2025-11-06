/// Represents the dimensional signature of a physical quantity using SI base units
///
/// Uses array-based storage for efficient const arithmetic operations.
/// Array indices: [L, T, M, I, Θ, J, N]
/// - L: Length (meter)
/// - T: Time (second)
/// - M: Mass (kilogram)
/// - I: Electric current (ampere)
/// - Θ: Thermodynamic temperature (kelvin)
/// - J: Luminous intensity (candela)
/// - N: Amount of substance (mole)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dimension {
    pub data: [i8; 7], // [L, T, M, I, Θ, J, N]
}

impl Dimension {
    /// Creates a new dimension with all exponents set to zero (dimensionless)
    pub const fn new() -> Self {
        Self {
            data: [0, 0, 0, 0, 0, 0, 0],
        }
    }

    /// Creates a dimension with specific exponents: [L, T, M, I, Θ, J, N]
    pub const fn from_array(data: [i8; 7]) -> Self {
        Self { data }
    }

    /// Creates a dimension with individual exponents
    pub const fn from_exponents(l: i8, t: i8, m: i8, i: i8, temp: i8, lum: i8, amount: i8) -> Self {
        Self {
            data: [l, t, m, i, temp, lum, amount],
        }
    }

    /// Creates a dimension for length (L¹)
    pub const fn length() -> Self {
        Self {
            data: [1, 0, 0, 0, 0, 0, 0],
        }
    }

    /// Creates a dimension for time (T¹)
    pub const fn time() -> Self {
        Self {
            data: [0, 1, 0, 0, 0, 0, 0],
        }
    }

    /// Creates a dimension for mass (M¹)
    pub const fn mass() -> Self {
        Self {
            data: [0, 0, 1, 0, 0, 0, 0],
        }
    }

    /// Creates a dimension for electric current (I¹)
    pub const fn current() -> Self {
        Self {
            data: [0, 0, 0, 1, 0, 0, 0],
        }
    }

    /// Creates a dimension for temperature (Θ¹)
    pub const fn temperature() -> Self {
        Self {
            data: [0, 0, 0, 0, 1, 0, 0],
        }
    }

    /// Creates a dimension for luminous intensity (J¹)
    pub const fn luminosity() -> Self {
        Self {
            data: [0, 0, 0, 0, 0, 1, 0],
        }
    }

    /// Creates a dimension for amount of substance (N¹)
    pub const fn amount() -> Self {
        Self {
            data: [0, 0, 0, 0, 0, 0, 1],
        }
    }

    /// Multiplies this dimension by another (adds exponents) - const fn for compile-time
    pub const fn multiply(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] + other.data[0], // L
                self.data[1] + other.data[1], // T
                self.data[2] + other.data[2], // M
                self.data[3] + other.data[3], // I
                self.data[4] + other.data[4], // Θ
                self.data[5] + other.data[5], // J
                self.data[6] + other.data[6], // N
            ],
        }
    }

    /// Divides this dimension by another (subtracts exponents) - const fn for compile-time
    pub const fn divide(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] - other.data[0], // L
                self.data[1] - other.data[1], // T
                self.data[2] - other.data[2], // M
                self.data[3] - other.data[3], // I
                self.data[4] - other.data[4], // Θ
                self.data[5] - other.data[5], // J
                self.data[6] - other.data[6], // N
            ],
        }
    }

    /// Raises this dimension to a power (multiplies all exponents) - const fn for compile-time
    pub const fn power(self, exponent: i8) -> Self {
        Self {
            data: [
                self.data[0] * exponent, // L
                self.data[1] * exponent, // T
                self.data[2] * exponent, // M
                self.data[3] * exponent, // I
                self.data[4] * exponent, // Θ
                self.data[5] * exponent, // J
                self.data[6] * exponent, // N
            ],
        }
    }

    /// Returns the inverse dimension (negates all exponents) - const fn for compile-time
    pub const fn inverse(self) -> Self {
        Self {
            data: [
                -self.data[0], // L
                -self.data[1], // T
                -self.data[2], // M
                -self.data[3], // I
                -self.data[4], // Θ
                -self.data[5], // J
                -self.data[6], // N
            ],
        }
    }

    /// Checks if this dimension is dimensionless (all exponents are zero) - const fn for compile-time
    pub const fn is_dimensionless(self) -> bool {
        self.data[0] == 0
            && self.data[1] == 0
            && self.data[2] == 0
            && self.data[3] == 0
            && self.data[4] == 0
            && self.data[5] == 0
            && self.data[6] == 0
    }

    /// Checks if two dimensions are equal - const fn for compile-time
    pub const fn equals(self, other: Self) -> bool {
        self.data[0] == other.data[0]
            && self.data[1] == other.data[1]
            && self.data[2] == other.data[2]
            && self.data[3] == other.data[3]
            && self.data[4] == other.data[4]
            && self.data[5] == other.data[5]
            && self.data[6] == other.data[6]
    }

    /// Getter methods for individual components
    pub const fn length_exp(&self) -> i8 {
        self.data[0]
    }
    pub const fn time_exp(&self) -> i8 {
        self.data[1]
    }
    pub const fn mass_exp(&self) -> i8 {
        self.data[2]
    }
    pub const fn current_exp(&self) -> i8 {
        self.data[3]
    }
    pub const fn temperature_exp(&self) -> i8 {
        self.data[4]
    }
    pub const fn luminosity_exp(&self) -> i8 {
        self.data[5]
    }
    pub const fn amount_exp(&self) -> i8 {
        self.data[6]
    }
}

// Common derived dimensions as constants
impl Dimension {
    /// Dimensionless (no units)
    pub const DIMENSIONLESS: Self = Self {
        data: [0, 0, 0, 0, 0, 0, 0],
    };

    /// Area dimension (L²)
    pub const AREA: Self = Self {
        data: [2, 0, 0, 0, 0, 0, 0],
    };

    /// Volume dimension (L³)
    pub const VOLUME: Self = Self {
        data: [3, 0, 0, 0, 0, 0, 0],
    };

    /// Velocity/Speed dimension (LT⁻¹)
    pub const VELOCITY: Self = Self {
        data: [1, -1, 0, 0, 0, 0, 0],
    };

    /// Acceleration dimension (LT⁻²)
    pub const ACCELERATION: Self = Self {
        data: [1, -2, 0, 0, 0, 0, 0],
    };

    /// Force dimension (MLT⁻²)
    pub const FORCE: Self = Self {
        data: [1, -2, 1, 0, 0, 0, 0],
    };

    /// Energy dimension (ML²T⁻²)
    pub const ENERGY: Self = Self {
        data: [2, -2, 1, 0, 0, 0, 0],
    };

    /// Power dimension (ML²T⁻³)
    pub const POWER: Self = Self {
        data: [2, -3, 1, 0, 0, 0, 0],
    };

    /// Pressure dimension (ML⁻¹T⁻²)
    pub const PRESSURE: Self = Self {
        data: [-1, -2, 1, 0, 0, 0, 0],
    };

    /// Frequency dimension (T⁻¹)
    pub const FREQUENCY: Self = Self {
        data: [0, -1, 0, 0, 0, 0, 0],
    };

    /// Electric charge dimension (IT)
    pub const CHARGE: Self = Self {
        data: [0, 1, 0, 1, 0, 0, 0],
    };

    /// Electric potential dimension (ML²T⁻³I⁻¹)
    pub const VOLTAGE: Self = Self {
        data: [2, -3, 1, -1, 0, 0, 0],
    };

    /// Resistance dimension (ML²T⁻³I⁻²)
    pub const RESISTANCE: Self = Self {
        data: [2, -3, 1, -2, 0, 0, 0],
    };

    /// Capacitance dimension (M⁻¹L⁻²T⁴I²)
    pub const CAPACITANCE: Self = Self {
        data: [-2, 4, -1, 2, 0, 0, 0],
    };

    /// Magnetic flux dimension (ML²T⁻²I⁻¹)
    pub const MAGNETIC_FLUX: Self = Self {
        data: [2, -2, 1, -1, 0, 0, 0],
    };

    /// Magnetic flux density dimension (MT⁻²I⁻¹)
    pub const MAGNETIC_FLUX_DENSITY: Self = Self {
        data: [0, -2, 1, -1, 0, 0, 0],
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_creation() {
        let length = Dimension::length();
        assert_eq!(length.data, [1, 0, 0, 0, 0, 0, 0]);

        let time = Dimension::time();
        assert_eq!(time.data, [0, 1, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_dimension_multiply() {
        let length = Dimension::length();
        let time = Dimension::time();
        let velocity = length.multiply(time.inverse());

        assert_eq!(velocity.data, [1, -1, 0, 0, 0, 0, 0]);
        assert_eq!(velocity, Dimension::VELOCITY);
    }

    #[test]
    fn test_dimension_divide() {
        let length = Dimension::length();
        let time = Dimension::time();
        let velocity = length.divide(time);

        assert_eq!(velocity, Dimension::VELOCITY);
    }

    #[test]
    fn test_dimension_power() {
        let length = Dimension::length();
        let area = length.power(2);

        assert_eq!(area, Dimension::AREA);
    }

    #[test]
    fn test_is_dimensionless() {
        let dimensionless = Dimension::new();
        assert!(dimensionless.is_dimensionless());

        let length = Dimension::length();
        assert!(!length.is_dimensionless());
    }
}
