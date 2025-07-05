//! Scalar arithmetic operations

use crate::core::{Quantity, UnitComposition};
use std::ops::{Mul, Div, MulAssign, DivAssign};

// Multiply quantity by scalar (right side)
impl<U, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    Mul<f64> for Quantity<U, L, M, T, K, I, J, N>
where
    U: UnitComposition,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.value * rhs)
    }
}

// Multiply scalar by quantity (left side)
impl<U, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    Mul<Quantity<U, L, M, T, K, I, J, N>> for f64
where
    U: UnitComposition,
{
    type Output = Quantity<U, L, M, T, K, I, J, N>;

    #[inline]
    fn mul(self, rhs: Quantity<U, L, M, T, K, I, J, N>) -> Self::Output {
        Quantity::new(self * rhs.value)
    }
}

// Divide quantity by scalar
impl<U, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    Div<f64> for Quantity<U, L, M, T, K, I, J, N>
where
    U: UnitComposition,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.value / rhs)
    }
}

// MulAssign with scalar
impl<U, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    MulAssign<f64> for Quantity<U, L, M, T, K, I, J, N>
where
    U: UnitComposition,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}

// DivAssign with scalar
impl<U, const L: i8, const M: i8, const T: i8, const K: i8, const I: i8, const J: i8, const N: i8> 
    DivAssign<f64> for Quantity<U, L, M, T, K, I, J, N>
where
    U: UnitComposition,
{
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.value /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantities::*;

    #[test]
    fn test_scalar_multiplication_right() {
        let d = Distance::<Meter>::new(10.0);
        let result = d * 2.0;
        
        assert_eq!(result.value(), 20.0);
    }

    #[test]
    fn test_scalar_multiplication_left() {
        let d = Distance::<Meter>::new(10.0);
        let result = 3.0 * d;
        
        assert_eq!(result.value(), 30.0);
    }

    #[test]
    fn test_scalar_division() {
        let d = Distance::<Meter>::new(20.0);
        let result = d / 4.0;
        
        assert_eq!(result.value(), 5.0);
    }

    #[test]
    fn test_scalar_mul_assign() {
        let mut d = Distance::<Meter>::new(10.0);
        d *= 2.5;
        
        assert_eq!(d.value(), 25.0);
    }

    #[test]
    fn test_scalar_div_assign() {
        let mut d = Distance::<Meter>::new(20.0);
        d /= 2.0;
        
        assert_eq!(d.value(), 10.0);
    }
}