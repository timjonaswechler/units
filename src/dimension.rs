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
        Self { data: [0, 0, 0, 0, 0, 0, 0] }
    }
    
    /// Creates a dimension with specific exponents: [L, T, M, I, Θ, J, N]
    pub const fn from_array(data: [i8; 7]) -> Self {
        Self { data }
    }
    
    /// Creates a dimension with individual exponents
    pub const fn from_exponents(l: i8, t: i8, m: i8, i: i8, temp: i8, lum: i8, amount: i8) -> Self {
        Self { data: [l, t, m, i, temp, lum, amount] }
    }
    
    /// Creates a dimension for length (L¹)
    pub const fn length() -> Self {
        Self { data: [1, 0, 0, 0, 0, 0, 0] }
    }
    
    /// Creates a dimension for time (T¹)
    pub const fn time() -> Self {
        Self { data: [0, 1, 0, 0, 0, 0, 0] }
    }
    
    /// Creates a dimension for mass (M¹)
    pub const fn mass() -> Self {
        Self { data: [0, 0, 1, 0, 0, 0, 0] }
    }
    
    /// Creates a dimension for electric current (I¹)
    pub const fn current() -> Self {
        Self { data: [0, 0, 0, 1, 0, 0, 0] }
    }
    
    /// Creates a dimension for temperature (Θ¹)
    pub const fn temperature() -> Self {
        Self { data: [0, 0, 0, 0, 1, 0, 0] }
    }
    
    /// Creates a dimension for luminous intensity (J¹)
    pub const fn luminosity() -> Self {
        Self { data: [0, 0, 0, 0, 0, 1, 0] }
    }
    
    /// Creates a dimension for amount of substance (N¹)
    pub const fn amount() -> Self {
        Self { data: [0, 0, 0, 0, 0, 0, 1] }
    }
    
    /// Multiplies this dimension by another (adds exponents) - const fn for compile-time
    pub const fn multiply(self, other: Self) -> Self {
        Self { data: [
            self.data[0] + other.data[0], // L
            self.data[1] + other.data[1], // T
            self.data[2] + other.data[2], // M
            self.data[3] + other.data[3], // I
            self.data[4] + other.data[4], // Θ
            self.data[5] + other.data[5], // J
            self.data[6] + other.data[6], // N
        ]}
    }
    
    /// Divides this dimension by another (subtracts exponents) - const fn for compile-time
    pub const fn divide(self, other: Self) -> Self {
        Self { data: [
            self.data[0] - other.data[0], // L
            self.data[1] - other.data[1], // T
            self.data[2] - other.data[2], // M
            self.data[3] - other.data[3], // I
            self.data[4] - other.data[4], // Θ
            self.data[5] - other.data[5], // J
            self.data[6] - other.data[6], // N
        ]}
    }
    
    /// Raises this dimension to a power (multiplies all exponents) - const fn for compile-time
    pub const fn power(self, exponent: i8) -> Self {
        Self { data: [
            self.data[0] * exponent, // L
            self.data[1] * exponent, // T
            self.data[2] * exponent, // M
            self.data[3] * exponent, // I
            self.data[4] * exponent, // Θ
            self.data[5] * exponent, // J
            self.data[6] * exponent, // N
        ]}
    }
    
    /// Returns the inverse dimension (negates all exponents) - const fn for compile-time
    pub const fn inverse(self) -> Self {
        Self { data: [
            -self.data[0], // L
            -self.data[1], // T
            -self.data[2], // M
            -self.data[3], // I
            -self.data[4], // Θ
            -self.data[5], // J
            -self.data[6], // N
        ]}
    }
    
    /// Checks if this dimension is dimensionless (all exponents are zero) - const fn for compile-time
    pub const fn is_dimensionless(self) -> bool {
        self.data[0] == 0 && self.data[1] == 0 && self.data[2] == 0 && self.data[3] == 0 
        && self.data[4] == 0 && self.data[5] == 0 && self.data[6] == 0
    }
    
    /// Returns a human-readable string representation for error messages
    pub fn display(&self) -> String {
        format!("[L:{} T:{} M:{} I:{} Θ:{} J:{} N:{}]", 
                self.data[0], self.data[1], self.data[2], 
                self.data[3], self.data[4], self.data[5], self.data[6])
    }
    
    /// Getter methods for individual components
    pub const fn length_exp(&self) -> i8 { self.data[0] }
    pub const fn time_exp(&self) -> i8 { self.data[1] }
    pub const fn mass_exp(&self) -> i8 { self.data[2] }
    pub const fn current_exp(&self) -> i8 { self.data[3] }
    pub const fn temperature_exp(&self) -> i8 { self.data[4] }
    pub const fn luminosity_exp(&self) -> i8 { self.data[5] }
    pub const fn amount_exp(&self) -> i8 { self.data[6] }
}

// Common derived dimensions as constants - updated for new array format
impl Dimension {
    /// Area dimension (L²)
    pub const AREA: Self = Self { data: [2, 0, 0, 0, 0, 0, 0] };
    
    /// Volume dimension (L³)
    pub const VOLUME: Self = Self { data: [3, 0, 0, 0, 0, 0, 0] };
    
    /// Velocity/Speed dimension (LT⁻¹)
    pub const VELOCITY: Self = Self { data: [1, -1, 0, 0, 0, 0, 0] };
    
    /// Acceleration dimension (LT⁻²)
    pub const ACCELERATION: Self = Self { data: [1, -2, 0, 0, 0, 0, 0] };
    
    /// Force dimension (MLT⁻²)
    pub const FORCE: Self = Self { data: [1, -2, 1, 0, 0, 0, 0] };
    
    /// Energy dimension (ML²T⁻²)
    pub const ENERGY: Self = Self { data: [2, -2, 1, 0, 0, 0, 0] };
    
    /// Power dimension (ML²T⁻³)
    pub const POWER: Self = Self { data: [2, -3, 1, 0, 0, 0, 0] };
    
    /// Pressure dimension (ML⁻¹T⁻²)
    pub const PRESSURE: Self = Self { data: [-1, -2, 1, 0, 0, 0, 0] };
    
    /// Frequency dimension (T⁻¹)
    pub const FREQUENCY: Self = Self { data: [0, -1, 0, 0, 0, 0, 0] };
    
    /// Electric charge dimension (IT)
    pub const CHARGE: Self = Self { data: [0, 1, 0, 1, 0, 0, 0] };
    
    /// Electric potential dimension (ML²T⁻³I⁻¹)
    pub const VOLTAGE: Self = Self { data: [2, -3, 1, -1, 0, 0, 0] };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_operations() {
        let length = Dimension::length();
        let time = Dimension::time();
        
        // Velocity = Length / Time
        let velocity = length.divide(time);
        assert_eq!(velocity, Dimension::VELOCITY);
        
        // Acceleration = Velocity / Time
        let acceleration = velocity.divide(time);
        assert_eq!(acceleration, Dimension::ACCELERATION);
        
        // Force = Mass * Acceleration
        let mass = Dimension::mass();
        let force = mass.multiply(acceleration);
        assert_eq!(force, Dimension::FORCE);
    }
    
    #[test]
    fn test_dimension_power() {
        let length = Dimension::length();
        let area = length.power(2);
        let volume = length.power(3);
        
        assert_eq!(area, Dimension::AREA);
        assert_eq!(volume, Dimension::VOLUME);
    }
    
    #[test]
    fn test_dimension_inverse() {
        let velocity = Dimension::VELOCITY;
        let time_per_length = velocity.inverse();
        
        assert_eq!(time_per_length, Dimension::from_array([-1, 1, 0, 0, 0, 0, 0]));
    }
    
    #[test] 
    fn test_const_arithmetic() {
        // Test that const functions work at compile-time
        const FORCE_DIM: Dimension = Dimension::FORCE;
        const AREA_DIM: Dimension = Dimension::AREA;
        const PRESSURE_DIM: Dimension = FORCE_DIM.divide(AREA_DIM);
        
        assert_eq!(PRESSURE_DIM, Dimension::PRESSURE);
    }
    
    #[test]
    fn test_dimension_display() {
        let pressure = Dimension::PRESSURE;
        let display = pressure.display();
        assert_eq!(display, "[L:-1 T:-2 M:1 I:0 Θ:0 J:0 N:0]");
    }
    
    #[test]
    fn test_dimension_getters() {
        let force = Dimension::FORCE;
        assert_eq!(force.length_exp(), 1);
        assert_eq!(force.time_exp(), -2);
        assert_eq!(force.mass_exp(), 1);
        assert_eq!(force.current_exp(), 0);
    }
}