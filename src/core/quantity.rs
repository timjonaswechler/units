use crate::core::UnitScale;
use crate::features::{DefaultFloat, Float};
use std::marker::PhantomData;

// Basis-Quantity für alle physikalischen Größen
pub struct Quantity<U, V = DefaultFloat>
where
    U: UnitScale,
    V: Float,
{
    value: V,
    _phantom: PhantomData<U>,
}

impl<U, V> Quantity<U, V>
where
    U: UnitScale,
    V: Float,
{
    pub fn new(value: V) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    pub fn value(&self) -> V {
        self.value
    }

    pub fn to<NewU>(&self) -> Quantity<NewU, V>
    where
        NewU: UnitScale,
        V: std::ops::Mul<DefaultFloat, Output = V>
            + std::ops::Div<DefaultFloat, Output = V>
            + From<DefaultFloat>,
    {
        let scale_factor = DefaultFloat::from(U::scale() / NewU::scale());
        Quantity::new(self.value * scale_factor)
    }
}
