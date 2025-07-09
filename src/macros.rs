#[macro_export]
macro_rules! define_quantity {
    ($name:ident, L=$l:expr, M=$m:expr, T=$t:expr, THETA=$theta:expr, I=$i:expr, J=$j:expr, N=$n:expr) => {
        paste::paste! {
            #[doc = concat!("Physical quantity: ", stringify!($name))]
            #[doc = concat!("Dimensions: L=", stringify!($l), ", M=", stringify!($m), ", T=", stringify!($t), ", THETA=", stringify!($theta), ", I=", stringify!($i), ", J=", stringify!($j), ", N=", stringify!($n))]
            pub struct $name<U, V = crate::features::DefaultFloat>
            where
                U: crate::core::Dimension + crate::core::UnitScale,
                V: crate::features::Float,
            {
                quantity: crate::core::Quantity<U, V>,
            }

            // Register this quantity type in the global registry for automatic operation detection
            // This will be used by the arithmetic operations to detect valid combinations

            impl<U, V> $name<U, V>
            where
                U: crate::core::Dimension + crate::core::UnitScale,
                V: crate::features::Float,
            {
                /// Create a new quantity with dimensional validation
                ///
                /// # Panics
                ///
                /// Panics if the unit U does not have the correct dimensions for this quantity type.
                pub fn new(value: V) -> Self {
                    Self::validate_dimensions::<U>();
                    Self { quantity: crate::core::Quantity::new(value) }
                }

                /// Create from a base quantity without additional validation
                pub fn from_quantity(quantity: crate::core::Quantity<U, V>) -> Self {
                    Self { quantity }
                }

                /// Convert into the underlying quantity
                pub fn into_quantity(self) -> crate::core::Quantity<U, V> {
                    self.quantity
                }

                /// Get the numeric value in the current unit
                pub fn value(&self) -> V {
                    self.quantity.value()
                }

                /// Convert to a different unit of the same quantity type
                pub fn to<NewU>(&self) -> $name<NewU, V>
                where
                    NewU: crate::core::Dimension + crate::core::UnitScale,
                    V: std::ops::Mul<crate::features::DefaultFloat, Output = V>
                        + std::ops::Div<crate::features::DefaultFloat, Output = V>
                        + From<crate::features::DefaultFloat>,
                {
                    Self::validate_dimensions::<NewU>();
                    $name { quantity: self.quantity.to() }
                }

                /// Validate that a unit type has the correct dimensions
                fn validate_dimensions<Unit: crate::core::Dimension>() {
                    const EXPECTED: [i8; 7] = [$l, $m, $t, $theta, $i, $j, $n];
                    let actual = [Unit::L, Unit::M, Unit::T, Unit::THETA, Unit::I, Unit::J, Unit::N];

                    if actual != EXPECTED {
                        panic!(
                            "\nDIMENSIONAL MISMATCH for {}\n\
                            Expected: L={}, M={}, T={}, THETA={}, I={}, J={}, N={}\n\
                            Actual:   L={}, M={}, T={}, THETA={}, I={}, J={}, N={}\n\
                            \n\
                            Cannot create {} with incompatible unit dimensions.",
                            stringify!($name),
                            $l, $m, $t, $theta, $i, $j, $n,
                            Unit::L, Unit::M, Unit::T, Unit::THETA, Unit::I, Unit::J, Unit::N,
                            stringify!($name)
                        );
                    }
                }

                /// Get the dimensional specification for this quantity type
                pub fn dimensions() -> [i8; 7] {
                    [$l, $m, $t, $theta, $i, $j, $n]
                }

                /// Get the name of this quantity type
                pub fn quantity_name() -> &'static str {
                    stringify!($name)
                }
            }

            // Implement PhysicalQuantity for automatic operation detection
            impl<U, V> crate::arithmetic::PhysicalQuantity for $name<U, V>
            where
                U: crate::core::Dimension + crate::core::UnitScale,
                V: crate::features::Float,
            {
                type Unit = U;
                type Value = V;

                fn dimensions() -> [i8; 7] {
                    [$l, $m, $t, $theta, $i, $j, $n]
                }

                fn quantity_name() -> &'static str {
                    stringify!($name)
                }

                fn from_base_quantity(quantity: crate::core::Quantity<U, V>) -> Self {
                    Self { quantity }
                }

                fn into_base_quantity(self) -> crate::core::Quantity<U, V> {
                    self.quantity
                }
            }

            // Display traits for better debugging
            impl<U, V> std::fmt::Display for $name<U, V>
            where
                U: crate::core::Dimension + crate::core::UnitScale,
                V: crate::features::Float + std::fmt::Display,
            {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} {}", self.value(), stringify!($name))
                }
            }

            impl<U, V> std::fmt::Debug for $name<U, V>
            where
                U: crate::core::Dimension + crate::core::UnitScale,
                V: crate::features::Float + std::fmt::Debug,
            {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!($name))
                        .field("value", &self.value())
                        .field("dimensions", &[$l, $m, $t, $theta, $i, $j, $n])
                        .finish()
                }
            }
        } // End of paste block
    };
}

#[macro_export]
macro_rules! define_prefix {
    ($name:ident, $factor:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
        pub struct $name;
        impl Prefix for $name {
            const FACTOR: DefaultFloat = $factor;
        }
    };
}

#[macro_export]
macro_rules! define_units {
    // Named dimensions syntax
    (
        dimension: { L = $l:expr, M = $m:expr, T = $t:expr, THETA = $theta:expr, I = $i:expr, J = $j:expr, N = $n:expr },
        base_unit: $base:ident = $base_scale:expr,
        units: {
            $($unit:ident = $scale:expr),* $(,)?
        }
    ) => {
        // Basis-Unit definieren
        pub struct $base;
        impl crate::core::Dimension for $base {
            const L: i8 = $l;
            const M: i8 = $m;
            const T: i8 = $t;
            const THETA: i8 = $theta;
            const I: i8 = $i;
            const J: i8 = $j;
            const N: i8 = $n;
        }
        impl crate::core::UnitScale for $base {
            const SCALE: f64 = $base_scale;
        }

        // Weitere Units definieren
        $(
            pub struct $unit;
            impl crate::core::Dimension for $unit {
                const L: i8 = $l;
                const M: i8 = $m;
                const T: i8 = $t;
                const THETA: i8 = $theta;
                const I: i8 = $i;
                const J: i8 = $j;
                const N: i8 = $n;
            }
            impl crate::core::UnitScale for $unit {
                const SCALE: f64 = $scale;
            }
        )*
    };

    // Short tuple syntax
    (
        dimension: { $l:expr, $m:expr, $t:expr, $theta:expr, $i:expr, $j:expr, $n:expr },
        base_unit: $base:ident = $base_scale:expr,
        units: {
            $($unit:ident = $scale:expr),* $(,)?
        }
    ) => {
        // Basis-Unit definieren
        pub struct $base;
        impl crate::core::Dimension for $base {
            const L: i8 = $l;
            const M: i8 = $m;
            const T: i8 = $t;
            const THETA: i8 = $theta;
            const I: i8 = $i;
            const J: i8 = $j;
            const N: i8 = $n;
        }
        impl crate::core::UnitScale for $base {
            const SCALE: f64 = $base_scale;
        }

        // Weitere Units definieren
        $(
            pub struct $unit;
            impl crate::core::Dimension for $unit {
                const L: i8 = $l;
                const M: i8 = $m;
                const T: i8 = $t;
                const THETA: i8 = $theta;
                const I: i8 = $i;
                const J: i8 = $j;
                const N: i8 = $n;
            }
            impl crate::core::UnitScale for $unit {
                const SCALE: f64 = $scale;
            }
        )*
    };
}
