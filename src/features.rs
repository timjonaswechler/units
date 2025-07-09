#![allow(dead_code)]
use std::ops::{Add, Div, Mul, Neg, Sub};

use static_assertions::const_assert;

// Mutually exclusive value type validation
#[cfg(all(feature = "f32", feature = "f64"))]
compile_error!("Cannot enable both f32 and f64 features");

#[cfg(all(feature = "f32", feature = "f128"))]
compile_error!("Cannot enable both f32 and f128 features");

#[cfg(all(feature = "f64", feature = "f128"))]
compile_error!("Cannot enable both f64 and f128 features");

#[cfg(not(any(feature = "f32", feature = "f64", feature = "f128")))]
compile_error!("Must enable one of: f32, f64, f128");

// Precision validation
#[cfg(all(feature = "precision-3", feature = "precision-6"))]
compile_error!("Cannot enable both precision-3 and precision-6 features");

#[cfg(all(feature = "precision-3", feature = "precision-9"))]
compile_error!("Cannot enable both precision-3 and precision-9 features");

#[cfg(all(feature = "precision-3", feature = "precision-12"))]
compile_error!("Cannot enable both precision-3 and precision-12 features");

#[cfg(all(feature = "precision-6", feature = "precision-9"))]
compile_error!("Cannot enable both precision-6 and precision-9 features");

#[cfg(all(feature = "precision-6", feature = "precision-12"))]
compile_error!("Cannot enable both precision-6 and precision-12 features");

#[cfg(all(feature = "precision-9", feature = "precision-12"))]
compile_error!("Cannot enable both precision-9 and precision-12 features");

// Display mode validation
#[cfg(all(feature = "compact", feature = "verbose"))]
compile_error!("Cannot enable both compact and verbose features");

#[cfg(all(feature = "compact", feature = "scientific"))]
compile_error!("Cannot enable both compact and scientific features");

#[cfg(all(feature = "verbose", feature = "scientific"))]
compile_error!("Cannot enable both verbose and scientific features");

// Value type selection
#[cfg(feature = "f32")]
pub type DefaultFloat = f32;

#[cfg(feature = "f64")]
pub type DefaultFloat = f64;

#[cfg(feature = "f128")]
pub type DefaultFloat = f128;

// Precision configuration
#[cfg(feature = "precision-3")]
pub const DEFAULT_PRECISION: usize = 3;

#[cfg(feature = "precision-6")]
pub const DEFAULT_PRECISION: usize = 6;

#[cfg(feature = "precision-9")]
pub const DEFAULT_PRECISION: usize = 9;

#[cfg(feature = "precision-12")]
pub const DEFAULT_PRECISION: usize = 12;

// Default precision if none specified
#[cfg(not(any(
    feature = "precision-3",
    feature = "precision-6",
    feature = "precision-9",
    feature = "precision-12"
)))]
pub const DEFAULT_PRECISION: usize = 6;

// Float trait bounds
pub trait Float:
    Copy + Clone + PartialEq + PartialOrd + std::fmt::Debug + std::fmt::Display
{
    const EPSILON: Self;
    const MAX: Self;
    const MIN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NAN: Self;

    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn log10(self) -> Self;
    fn round(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn is_finite(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_nan(self) -> bool;
}
#[cfg(feature = "f32")]
impl Float for f32 {
    const EPSILON: Self = f32::EPSILON;
    const MAX: Self = f32::MAX;
    const MIN: Self = f32::MIN;
    const INFINITY: Self = f32::INFINITY;
    const NEG_INFINITY: Self = f32::NEG_INFINITY;
    const NAN: Self = f32::NAN;

    fn abs(self) -> Self {
        self.abs()
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn tan(self) -> Self {
        self.tan()
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn log10(self) -> Self {
        self.log10()
    }
    fn round(self) -> Self {
        self.round()
    }
    fn floor(self) -> Self {
        self.floor()
    }
    fn ceil(self) -> Self {
        self.ceil()
    }
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn is_infinite(self) -> bool {
        self.is_infinite()
    }
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

#[cfg(feature = "f64")]
impl Float for f64 {
    const EPSILON: Self = f64::EPSILON;
    const MAX: Self = f64::MAX;
    const MIN: Self = f64::MIN;
    const INFINITY: Self = f64::INFINITY;
    const NEG_INFINITY: Self = f64::NEG_INFINITY;
    const NAN: Self = f64::NAN;

    fn abs(self) -> Self {
        self.abs()
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn tan(self) -> Self {
        self.tan()
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn log10(self) -> Self {
        self.log10()
    }
    fn round(self) -> Self {
        self.round()
    }
    fn floor(self) -> Self {
        self.floor()
    }
    fn ceil(self) -> Self {
        self.ceil()
    }
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn is_infinite(self) -> bool {
        self.is_infinite()
    }
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

// f128 support when available
#[cfg(feature = "f128")]
impl Float for f128 {
    const EPSILON: Self = f128::EPSILON;
    const MAX: Self = f128::MAX;
    const MIN: Self = f128::MIN;
    const INFINITY: Self = f128::INFINITY;
    const NEG_INFINITY: Self = f128::NEG_INFINITY;
    const NAN: Self = f128::NAN;

    fn abs(self) -> Self {
        self.abs()
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn tan(self) -> Self {
        self.tan()
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn log10(self) -> Self {
        self.log10()
    }
    fn round(self) -> Self {
        self.round()
    }
    fn floor(self) -> Self {
        self.floor()
    }
    fn ceil(self) -> Self {
        self.ceil()
    }
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn is_infinite(self) -> bool {
        self.is_infinite()
    }
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

// Compile-time validation that default float implements required traits
const_assert!(std::mem::size_of::<DefaultFloat>() > 0);

// Arithmetic trait implementations
/// Newtype wrapper to allow trait implementations for DefaultFloat
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct DefaultFloatWrapper(pub DefaultFloat);

impl Add for DefaultFloatWrapper {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        DefaultFloatWrapper(self.0 + rhs.0)
    }
}

impl Sub for DefaultFloatWrapper {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        DefaultFloatWrapper(self.0 - rhs.0)
    }
}

impl Mul for DefaultFloatWrapper {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        DefaultFloatWrapper(self.0 * rhs.0)
    }
}

impl Div for DefaultFloatWrapper {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        DefaultFloatWrapper(self.0 / rhs.0)
    }
}

impl Neg for DefaultFloatWrapper {
    type Output = Self;
    fn neg(self) -> Self::Output {
        DefaultFloatWrapper(-self.0)
    }
}

// Conversion helpers
impl From<DefaultFloatWrapper> for DefaultFloat {
    fn from(wrapper: DefaultFloatWrapper) -> Self {
        wrapper.0
    }
}

impl From<f32> for DefaultFloatWrapper {
    fn from(value: f32) -> Self {
        #[cfg(feature = "f32")]
        {
            DefaultFloatWrapper(value)
        }
        #[cfg(feature = "f64")]
        {
            DefaultFloatWrapper(value as f64)
        }
        #[cfg(feature = "f128")]
        {
            DefaultFloatWrapper(value as f128)
        }
    }
}

impl From<f64> for DefaultFloatWrapper {
    fn from(value: f64) -> Self {
        #[cfg(feature = "f32")]
        {
            DefaultFloatWrapper(value as f32)
        }
        #[cfg(feature = "f64")]
        {
            DefaultFloatWrapper(value)
        }
        #[cfg(feature = "f128")]
        {
            DefaultFloatWrapper(value as f128)
        }
    }
}

impl From<i32> for DefaultFloatWrapper {
    fn from(value: i32) -> Self {
        #[cfg(feature = "f32")]
        {
            DefaultFloatWrapper(value as f32)
        }
        #[cfg(feature = "f64")]
        {
            DefaultFloatWrapper(value as f64)
        }
        #[cfg(feature = "f128")]
        {
            DefaultFloatWrapper(value as f128)
        }
    }
}

impl From<i64> for DefaultFloatWrapper {
    fn from(value: i64) -> Self {
        #[cfg(feature = "f32")]
        {
            DefaultFloatWrapper(value as f32)
        }
        #[cfg(feature = "f64")]
        {
            DefaultFloatWrapper(value as f64)
        }
        #[cfg(feature = "f128")]
        {
            DefaultFloatWrapper(value as f128)
        }
    }
}

// Zero and One traits for generic programming
pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

impl Zero for DefaultFloat {
    fn zero() -> Self {
        0.0.into()
    }

    fn is_zero(&self) -> bool {
        self.abs() < Self::EPSILON
    }
}

impl One for DefaultFloat {
    fn one() -> Self {
        1.0.into()
    }

    fn is_one(&self) -> bool {
        (self - Self::one()).abs() < Self::EPSILON
    }
}
