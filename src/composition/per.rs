use crate::core::{Dimension, UnitScale};
use crate::features::DefaultFloat;
use std::marker::PhantomData;

// Per<U> - Invertiert alle Dimensionen (1/U)
pub struct Per<U>(PhantomData<U>);

// Automatic implementation for ALL dimension combinations!
// This is the organic, automatic approach you wanted
impl<U: Dimension> Dimension for Per<U> {
    const L: i8 = -U::L;
    const M: i8 = -U::M;
    const T: i8 = -U::T;
    const THETA: i8 = -U::THETA;
    const I: i8 = -U::I;
    const J: i8 = -U::J;
    const N: i8 = -U::N;
}

// Per<U> hat die gleiche Skalierung wie U (1/U = 1/U)
impl<U: UnitScale> UnitScale for Per<U> {
    const SCALE: DefaultFloat = U::SCALE;
}
