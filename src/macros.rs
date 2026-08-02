macro_rules! define_quantity {
    ($name:ident) => {
        #[doc = concat!("A scalar `", stringify!($name), "` stored in its canonical SI scale.")]
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        pub struct $name(f64);

        impl $name {
            /// Constructs a quantity from a value expressed in `U`.
            pub fn new<U>(value: f64) -> Self
            where
                U: $crate::Unit<Quantity = Self>,
            {
                Self(value * U::SCALE_TO_SI)
            }

            /// Constructs a quantity directly from its canonical SI value.
            pub const fn from_si(value: f64) -> Self {
                Self(value)
            }

            /// Returns the canonical SI value.
            pub const fn si(&self) -> f64 {
                self.0
            }

            /// Returns the value expressed in `U`.
            pub fn to<U>(&self) -> f64
            where
                U: $crate::Unit<Quantity = Self>,
            {
                self.0 / U::SCALE_TO_SI
            }
        }
    };
}

macro_rules! define_unit {
    ($(#[$meta:meta])* $name:ident : $quantity:ty, scale = $scale:expr) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $name;

        impl $crate::Unit for $name {
            type Quantity = $quantity;

            const SCALE_TO_SI: f64 = $scale;
        }
    };
    ($(#[$meta:meta])* $name:ident : $quantity:ty, scale = $scale:expr, prefixable) => {
        $crate::macros::define_unit!(
            $(#[$meta])*
            $name : $quantity,
            scale = $scale
        );

        impl $crate::PrefixableUnit for $name {}
    };
}

macro_rules! define_prefix {
    ($name:ident, factor = $factor:expr) => {
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $name;

        impl $crate::Prefix for $name {
            const FACTOR: f64 = $factor;
        }
    };
}

pub(crate) use define_prefix;
pub(crate) use define_quantity;
pub(crate) use define_unit;
