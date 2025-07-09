use crate::core::UnitScale;
use crate::features::{DefaultFloat, Float};
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

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
    pub const fn new(value: V) -> Self {
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

// Arithmetic operations for Quantity
impl<U, V> Mul<Quantity<U, V>> for Quantity<U, V>
where
    U: UnitScale,
    V: Float + Mul<Output = V>,
{
    type Output = Quantity<(U, U), V>;

    fn mul(self, rhs: Quantity<U, V>) -> Self::Output {
        Quantity::new(self.value * rhs.value)
    }
}

impl<U, V> Div<Quantity<U, V>> for Quantity<U, V>
where
    U: UnitScale,
    V: Float + Div<Output = V>,
{
    type Output = Quantity<crate::composition::Per<U>, V>;

    fn div(self, rhs: Quantity<U, V>) -> Self::Output {
        Quantity::new(self.value / rhs.value)
    }
}

impl<U, V> Add<Quantity<U, V>> for Quantity<U, V>
where
    U: UnitScale,
    V: Float + Add<Output = V>,
{
    type Output = Quantity<U, V>;

    fn add(self, rhs: Quantity<U, V>) -> Self::Output {
        Quantity::new(self.value + rhs.value)
    }
}

impl<U, V> Sub<Quantity<U, V>> for Quantity<U, V>
where
    U: UnitScale,
    V: Float + Sub<Output = V>,
{
    type Output = Quantity<U, V>;

    fn sub(self, rhs: Quantity<U, V>) -> Self::Output {
        Quantity::new(self.value - rhs.value)
    }
}

impl<U, V> Neg for Quantity<U, V>
where
    U: UnitScale,
    V: Float + Neg<Output = V>,
{
    type Output = Quantity<U, V>;

    fn neg(self) -> Self::Output {
        Quantity::new(-self.value)
    }
}

// Scalar operations
impl<U, V> Mul<V> for Quantity<U, V>
where
    U: UnitScale,
    V: Float + Mul<Output = V>,
{
    type Output = Quantity<U, V>;

    fn mul(self, rhs: V) -> Self::Output {
        Quantity::new(self.value * rhs)
    }
}

impl<U, V> Div<V> for Quantity<U, V>
where
    U: UnitScale,
    V: Float + Div<Output = V>,
{
    type Output = Quantity<U, V>;

    fn div(self, rhs: V) -> Self::Output {
        Quantity::new(self.value / rhs)
    }
}

// Implement conversion traits for Quantity
impl<U, V> crate::arithmetic::IntoQuantity<U, V> for Quantity<U, V>
where
    U: crate::core::Dimension + UnitScale,
    V: Float,
{
    fn into_quantity(self) -> Quantity<U, V> {
        self
    }
}

impl<U, V> crate::arithmetic::FromQuantity<U, V> for Quantity<U, V>
where
    U: crate::core::Dimension + UnitScale,
    V: Float,
{
    fn from_quantity(quantity: Quantity<U, V>) -> Self {
        quantity
    }
}
