use crate::composition::unit::UnitScale;

// Basic quantity type
#[derive(Debug, Clone, Copy)]
pub struct Quantity<Unit, Value> {
    value: Value,
    _unit: std::marker::PhantomData<Unit>,
}

impl<Unit, Value> Quantity<Unit, Value>
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
impl<Unit, Value> Quantity<Unit, Value>
where
    Unit: UnitScale,
    Value: Copy + Into<f64>,
{
    pub fn si(&self) -> f64 {
        self.value.into() * Unit::scale()
    }
}
