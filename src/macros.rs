#[macro_export]
macro_rules! define_quantity {
    ($name:ident, L=$l:expr, M=$m:expr, T=$t:expr, THETA=$theta:expr, I=$i:expr, J=$j:expr, N=$n:expr) => {
        use crate::core::{Quantity, Dimension, UnitScale};
        use crate::features::{DefaultFloat, Float};
        use core::ops::{Mul, Div};

        #[doc = concat!("Physical quantity: ", stringify!($name))]
        #[doc = concat!("Dimensions: L=", stringify!($l), ", M=", stringify!($m), ", T=", stringify!($t), ", THETA=", stringify!($theta), ", I=", stringify!($i), ", J=", stringify!($j), ", N=", stringify!($n))]
        pub struct $name<U, V = DefaultFloat>
        where
            U: Dimension<$l, $m, $t, $theta, $i, $j, $n> + UnitScale,
            V: Float,
        {
            quantity: Quantity<U, V>,
        }

        impl<U, V> $name<U, V>
        where
            U: Dimension<$l, $m, $t, $theta, $i, $j, $n> + UnitScale,
            V: Float,
        {
            pub fn new(value: V) -> Self {
                Self { quantity: Quantity::new(value) }
            }

            pub fn value(&self) -> V {
                self.quantity.value()
            }

            pub fn to<NewU>(&self) -> $name<NewU, V>
            where
                NewU: Dimension<$l, $m, $t, $theta, $i, $j, $n> + UnitScale,
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
