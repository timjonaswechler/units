use crate::core::{Dimension, UnitScale};
use crate::features::DefaultFloat;

// Tuple-Komposition for Dimensionsmultiplikation
// (U1, U2) = U1 * U2
// (U1, U2, U3) = U1 * U2 * U3
// etc.

// Automatic implementation for ALL dimension combinations!
// This is the organic, automatic approach you wanted

// 2-Tuple: (U1, U2) = U1 * U2
impl<U1: Dimension, U2: Dimension> Dimension for (U1, U2) {
    const L: i8 = U1::L + U2::L;
    const M: i8 = U1::M + U2::M;
    const T: i8 = U1::T + U2::T;
    const THETA: i8 = U1::THETA + U2::THETA;
    const I: i8 = U1::I + U2::I;
    const J: i8 = U1::J + U2::J;
    const N: i8 = U1::N + U2::N;
}

// 3-Tuple: (U1, U2, U3) = U1 * U2 * U3
impl<U1: Dimension, U2: Dimension, U3: Dimension> Dimension for (U1, U2, U3) {
    const L: i8 = U1::L + U2::L + U3::L;
    const M: i8 = U1::M + U2::M + U3::M;
    const T: i8 = U1::T + U2::T + U3::T;
    const THETA: i8 = U1::THETA + U2::THETA + U3::THETA;
    const I: i8 = U1::I + U2::I + U3::I;
    const J: i8 = U1::J + U2::J + U3::J;
    const N: i8 = U1::N + U2::N + U3::N;
}

// 4-Tuple: (U1, U2, U3, U4) = U1 * U2 * U3 * U4
impl<U1: Dimension, U2: Dimension, U3: Dimension, U4: Dimension> Dimension for (U1, U2, U3, U4) {
    const L: i8 = U1::L + U2::L + U3::L + U4::L;
    const M: i8 = U1::M + U2::M + U3::M + U4::M;
    const T: i8 = U1::T + U2::T + U3::T + U4::T;
    const THETA: i8 = U1::THETA + U2::THETA + U3::THETA + U4::THETA;
    const I: i8 = U1::I + U2::I + U3::I + U4::I;
    const J: i8 = U1::J + U2::J + U3::J + U4::J;
    const N: i8 = U1::N + U2::N + U3::N + U4::N;
}

// UnitScale implementations for tuples
// Scale factors multiply: (U1, U2) has scale U1::SCALE * U2::SCALE

impl<U1: UnitScale, U2: UnitScale> UnitScale for (U1, U2) {
    const SCALE: DefaultFloat = 1.0; // Will be calculated at runtime
    
    fn scale() -> DefaultFloat {
        U1::scale() * U2::scale()
    }
}

impl<U1: UnitScale, U2: UnitScale, U3: UnitScale> UnitScale for (U1, U2, U3) {
    const SCALE: DefaultFloat = 1.0; // Will be calculated at runtime
    
    fn scale() -> DefaultFloat {
        U1::scale() * U2::scale() * U3::scale()
    }
}

impl<U1: UnitScale, U2: UnitScale, U3: UnitScale, U4: UnitScale> UnitScale for (U1, U2, U3, U4) {
    const SCALE: DefaultFloat = 1.0; // Will be calculated at runtime
    
    fn scale() -> DefaultFloat {
        U1::scale() * U2::scale() * U3::scale() * U4::scale()
    }
}