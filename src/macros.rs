#[macro_export]
macro_rules! define_quantity {
    ($name:ident, L=$l:expr, M=$m:expr, T=$t:expr, THETA=$theta:expr, I=$i:expr, J=$j:expr, N=$n:expr) => {
        // Helper trait for dimensional validation would go here
        // For now we use runtime validation in the constructor

        #[doc = concat!("Physical quantity: ", stringify!($name))]
        #[doc = concat!("Dimensions: L=", stringify!($l), ", M=", stringify!($m), ", T=", stringify!($t), ", THETA=", stringify!($theta), ", I=", stringify!($i), ", J=", stringify!($j), ", N=", stringify!($n))]
        pub struct $name<U, V = crate::features::DefaultFloat>
        where
            U: crate::core::Dimension +crate::core:: UnitScale,
            V: crate::features::Float,
        {
            quantity: crate::core::Quantity<U, V>,
        }

        impl<U, V> $name<U, V>
        where
            U: crate::core::Dimension + crate::core::UnitScale,
            V: crate::features::Float,
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

                Self { quantity: crate::core::Quantity::new(value) }
            }

            pub fn value(&self) -> V {
                self.quantity.value()
            }

            pub fn to<NewU>(&self) -> $name<NewU, V>
            where
                NewU: crate::core::Dimension + crate::core::UnitScale,
                V: std::ops::Mul<crate::features::DefaultFloat, Output = V> + std::ops::Div<crate::features::DefaultFloat, Output = V> + From<crate::features::DefaultFloat>,
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

#[macro_export]
macro_rules! define_units {
    (
        dimension $dim_name:ident {
            base_unit: $base:ident = $base_scale:expr,
            units: {
                $($unit:ident = $scale:expr),* $(,)?
            }
        }
    ) => {
        // Basis-Unit definieren
        pub struct $base;
        impl crate::core::Dimension for $base {
            // Dimensionen werden automatisch aus dem Kontext abgeleitet
        }
        impl crate::core::UnitScale for $base {
            const SCALE: f64 = $base_scale;
        }

        // Weitere Units definieren
        $(
            pub struct $unit;
            impl crate::core::Dimension for $unit {
                // Gleiche Dimensionen wie Basis-Unit
            }
            impl crate::core::UnitScale for $unit {
                const SCALE: f64 = $scale;
            }
        )*
    };
}
