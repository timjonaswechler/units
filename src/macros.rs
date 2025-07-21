#[macro_export]
macro_rules! define_quantity {
    ($name:ident) => {
        pub struct $name<Unit, Value = f64> {
            value: Value,
            _unit: std::marker::PhantomData<Unit>,
        }

        impl<Unit, Value> $name<Unit, Value>
        where
            Value: Copy,
        {
            pub fn new(value: Value) -> Self {
                Self {
                    value,
                    _unit: std::marker::PhantomData,
                }
            }

            pub fn value(&self) -> Value {
                self.value
            }
        }

        // SI conversion for units that implement UnitScale
        impl<Unit, Value> $name<Unit, Value>
        where
            Unit: $crate::composition::UnitScale,
            Value: Copy + Into<f64>,
        {
            pub fn si(&self) -> f64 {
                self.value.into() * Unit::scale()
            }
            
            /// Returns SI value rounded to specified decimal places
            pub fn si_rounded(&self, decimal_places: u32) -> f64 {
                let factor = 10f64.powi(decimal_places as i32);
                (self.si() * factor).round() / factor
            }
            
            /// Returns SI value truncated to specified decimal places
            pub fn si_truncated(&self, decimal_places: u32) -> f64 {
                let factor = 10f64.powi(decimal_places as i32);
                (self.si() * factor).trunc() / factor
            }
        }
        
        // Precision methods for Value when it's f64/f32
        impl<Unit> $name<Unit, f64>
        where
            Unit: $crate::composition::UnitScale,
        {
            /// Returns value rounded to specified decimal places
            pub fn value_rounded(&self, decimal_places: u32) -> f64 {
                let factor = 10f64.powi(decimal_places as i32);
                (self.value * factor).round() / factor
            }
            
            /// Returns value truncated to specified decimal places
            pub fn value_truncated(&self, decimal_places: u32) -> f64 {
                let factor = 10f64.powi(decimal_places as i32);
                (self.value * factor).trunc() / factor
            }
        }
        
        impl<Unit> $name<Unit, f32>
        where
            Unit: $crate::composition::UnitScale,
        {
            /// Returns value rounded to specified decimal places
            pub fn value_rounded(&self, decimal_places: u32) -> f32 {
                let factor = 10f32.powi(decimal_places as i32);
                (self.value * factor).round() / factor
            }
            
            /// Returns value truncated to specified decimal places  
            pub fn value_truncated(&self, decimal_places: u32) -> f32 {
                let factor = 10f32.powi(decimal_places as i32);
                (self.value * factor).trunc() / factor
            }
        }
    };
}

#[macro_export]
macro_rules! define_prefix {
    ($name:ident, $factor:expr) => {
        pub struct $name;
        impl $crate::composition::PrefixScale for $name {
            fn factor() -> f64 {
                $factor
            }
        }
    };
}

#[macro_export]
macro_rules! define_units {
    (
        base_unit: $base:ident = $base_scale:expr,
        units: {
            $($unit:ident = $scale:expr),* $(,)?
        }
    ) => {
        // Basis-Unit definieren
        pub struct $base;
        impl $crate::composition::UnitScale for $base {
            fn scale() -> f64 { $base_scale }
        }

        // Weitere Units definieren
        $(
            pub struct $unit;
            impl $crate::composition::UnitScale for $unit {
                fn scale() -> f64 { $scale }
            }
        )*
    };
}

// Neues Makro für Composite Units (komplexe Unit-Kombinationen)
#[macro_export]
macro_rules! define_composite_units {
    (
        quantity: $quantity:ident,
        base_composite: $base_type:ty = $base_scale:expr,
        units: {
            $($unit:ident : $unit_type:ty = $scale:expr),* $(,)?
        }
    ) => {
        // Type Aliases für die Units
        pub type $quantity<U> = $quantity<U>;

        // Basis-Unit definieren (als leere Struct für Composite)
        pub struct BaseComposite;
        impl crate::composition::UnitScale for BaseComposite {
            const SCALE: crate::features::DefaultFloat = $base_scale;
        }

        // Type Alias für die Basis-Kombination
        pub type Base = $base_type;

        // Weitere Composite Units definieren
        $(
            pub struct $unit;
            impl crate::composition::UnitScale for $unit {
                const SCALE: crate::features::DefaultFloat = $scale;
            }
            pub type $unit = $unit_type;
        )*
    };
}
