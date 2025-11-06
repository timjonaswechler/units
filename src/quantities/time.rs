use crate::dimension::Dimension;
use crate::quantity::Quantity;
use crate::unit::Unit;

/// Physical quantity: Time
///
/// SI Base Unit: Second (s)
#[derive(Debug, Clone, Copy)]
pub struct Time;

impl Quantity for Time {
    const DIMENSION: Dimension = Dimension::time();
    const NAME: &'static str = "Time";
}

// ============================================================================
// SI Base Unit
// ============================================================================

/// Second - SI base unit for time
#[derive(Debug, Clone, Copy)]
pub struct Second;

impl Unit for Second {
    type BaseQuantity = Time;
    const SYMBOL: &'static str = "s";
    const TO_SI: f64 = 1.0;
    const OFFSET: f64 = 0.0;
}

// ============================================================================
// Common Time Units
// ============================================================================

/// Minute (1 min = 60 s)
#[derive(Debug, Clone, Copy)]
pub struct Minute;

impl Unit for Minute {
    type BaseQuantity = Time;
    const SYMBOL: &'static str = "min";
    const TO_SI: f64 = 60.0;
    const OFFSET: f64 = 0.0;
}

/// Hour (1 h = 3600 s)
#[derive(Debug, Clone, Copy)]
pub struct Hour;

impl Unit for Hour {
    type BaseQuantity = Time;
    const SYMBOL: &'static str = "h";
    const TO_SI: f64 = 3600.0;
    const OFFSET: f64 = 0.0;
}

/// Day (1 d = 86400 s)
#[derive(Debug, Clone, Copy)]
pub struct Day;

impl Unit for Day {
    type BaseQuantity = Time;
    const SYMBOL: &'static str = "d";
    const TO_SI: f64 = 86400.0;
    const OFFSET: f64 = 0.0;
}

/// Week (1 week = 604800 s)
#[derive(Debug, Clone, Copy)]
pub struct Week;

impl Unit for Week {
    type BaseQuantity = Time;
    const SYMBOL: &'static str = "week";
    const TO_SI: f64 = 604800.0;
    const OFFSET: f64 = 0.0;
}

/// Year (1 year = 31557600 s, based on Julian year)
#[derive(Debug, Clone, Copy)]
pub struct Year;

impl Unit for Year {
    type BaseQuantity = Time;
    const SYMBOL: &'static str = "yr";
    const TO_SI: f64 = 31557600.0;
    const OFFSET: f64 = 0.0;
}

// ============================================================================
// Small Time Units
// ============================================================================

/// Millisecond (1 ms = 0.001 s)
#[derive(Debug, Clone, Copy)]
pub struct Millisecond;

impl Unit for Millisecond {
    type BaseQuantity = Time;
    const SYMBOL: &'static str = "ms";
    const TO_SI: f64 = 0.001;
    const OFFSET: f64 = 0.0;
}

/// Microsecond (1 μs = 1e-6 s)
#[derive(Debug, Clone, Copy)]
pub struct Microsecond;

impl Unit for Microsecond {
    type BaseQuantity = Time;
    const SYMBOL: &'static str = "μs";
    const TO_SI: f64 = 1e-6;
    const OFFSET: f64 = 0.0;
}

/// Nanosecond (1 ns = 1e-9 s)
#[derive(Debug, Clone, Copy)]
pub struct Nanosecond;

impl Unit for Nanosecond {
    type BaseQuantity = Time;
    const SYMBOL: &'static str = "ns";
    const TO_SI: f64 = 1e-9;
    const OFFSET: f64 = 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_time_dimension() {
        assert_eq!(Time::dimension(), Dimension::time());
    }

    #[test]
    fn test_second_conversion() {
        let s = Value::<Time, Second>::new(1.0);
        assert_eq!(s.get_si(), 1.0);
    }

    #[test]
    fn test_minute_conversion() {
        let min = Value::<Time, Minute>::new(1.0);
        assert_eq!(min.get_si(), 60.0);

        let s = min.convert::<Second>();
        assert_eq!(s.get(), 60.0);
    }

    #[test]
    fn test_hour_conversion() {
        let h = Value::<Time, Hour>::new(1.0);
        let s = h.convert::<Second>();
        assert_eq!(s.get(), 3600.0);
    }

    #[test]
    fn test_time_addition() {
        let s1 = Value::<Time, Second>::new(30.0);
        let min1 = Value::<Time, Minute>::new(1.0);
        let result = s1 + min1;
        assert_eq!(result.get(), 90.0);
    }
}
