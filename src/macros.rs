#[macro_export]
macro_rules! define_quantity {
    ($name:ident, L=$l:expr, M=$m:expr, T=$t:expr, THETA=$theta:expr, I=$i:expr, J=$j:expr, N=$n:expr) => {
        use crate::core::{Quantity, Dimension, UnitScale};
        use crate::features::{DefaultFloat, Float};
        use core::ops::{Mul, Div};

        // Helper trait for dimensional validation would go here
        // For now we use runtime validation in the constructor

        #[doc = concat!("Physical quantity: ", stringify!($name))]
        #[doc = concat!("Dimensions: L=", stringify!($l), ", M=", stringify!($m), ", T=", stringify!($t), ", THETA=", stringify!($theta), ", I=", stringify!($i), ", J=", stringify!($j), ", N=", stringify!($n))]
        pub struct $name<U, V = DefaultFloat>
        where
            U: Dimension + UnitScale,
            V: Float,
        {
            quantity: Quantity<U, V>,
        }

        impl<U, V> $name<U, V>
        where
            U: Dimension + UnitScale,
            V: Float,
        {
            pub fn new(value: V) -> Self {
                // Runtime validation for now - can be improved later
                assert!(U::L == $l, "Wrong L dimension: expected {}, got {}", $l, U::L);
                assert!(U::M == $m, "Wrong M dimension: expected {}, got {}", $m, U::M);
                assert!(U::T == $t, "Wrong T dimension: expected {}, got {}", $t, U::T);
                assert!(U::THETA == $theta, "Wrong THETA dimension: expected {}, got {}", $theta, U::THETA);
                assert!(U::I == $i, "Wrong I dimension: expected {}, got {}", $i, U::I);
                assert!(U::J == $j, "Wrong J dimension: expected {}, got {}", $j, U::J);
                assert!(U::N == $n, "Wrong N dimension: expected {}, got {}", $n, U::N);

                Self { quantity: Quantity::new(value) }
            }

            pub fn value(&self) -> V {
                self.quantity.value()
            }

            pub fn to<NewU>(&self) -> $name<NewU, V>
            where
                NewU: Dimension + UnitScale,
                V: Mul<DefaultFloat, Output = V> + Div<DefaultFloat, Output = V> + From<DefaultFloat>,
            {
                $name { quantity: self.quantity.to() }
            }
        }
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
