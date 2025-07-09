use crate::core::{Dimension, UnitScale};
use crate::features::DefaultFloat;
use std::marker::PhantomData;

// Exponent<U, N> - Potenziert alle Dimensionen (U^N)
pub struct Exponent<U, const N: i8>(PhantomData<U>);

// Automatic implementation for ALL dimension combinations!
// This is the organic, automatic approach you wanted
impl<U: Dimension, const N: i8> Dimension for Exponent<U, N> {
    const L: i8 = U::L * N;
    const M: i8 = U::M * N;
    const T: i8 = U::T * N;
    const THETA: i8 = U::THETA * N;
    const I: i8 = U::I * N;
    const J: i8 = U::J * N;
    const N: i8 = U::N * N;
}

// Exponent<U, N> hat die Skalierung U^N
impl<U: UnitScale, const N: i8> UnitScale for Exponent<U, N> {
    const SCALE: DefaultFloat = {
        // Simplified approach - we'll calculate this at runtime if needed
        // For now, default to 1.0 and handle in runtime functions
        1.0
    };
    
    fn scale() -> DefaultFloat {
        // Runtime power calculation
        let base = U::scale();
        if N == 0 { return 1.0; }
        if N == 1 { return base; }
        if N == -1 { return 1.0 / base; }
        
        // For other powers, calculate at runtime
        let mut result = 1.0;
        let abs_n = N.abs() as u32;
        for _ in 0..abs_n {
            result *= base;
        }
        if N < 0 { 1.0 / result } else { result }
    }
}