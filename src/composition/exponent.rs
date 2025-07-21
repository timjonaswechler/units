// Exponent<Unit, N> - Powers of units (Unit^N)
// This is a more advanced feature not used in the basic system

use crate::composition::unit::UnitScale;
use std::marker::PhantomData;

pub struct Exponent<U, const N: i8>(PhantomData<U>);

impl<U: UnitScale, const N: i8> UnitScale for Exponent<U, N> {
    fn scale() -> f64 {
        let base = U::scale();
        if N == 0 {
            return 1.0;
        }
        if N == 1 {
            return base;
        }
        if N == -1 {
            return 1.0 / base;
        }

        // For other powers, calculate at runtime
        let mut result = 1.0;
        let abs_n = N.abs() as u32;
        for _ in 0..abs_n {
            result *= base;
        }
        if N < 0 { 1.0 / result } else { result }
    }
}
