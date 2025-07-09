use crate::core::Dimension;
use std::marker::PhantomData;
// Per<U> - Invertiert alle Dimensionen (1/U)
// pub struct Per<U>(PhantomData<U>);
// impl<U: Dimension> Dimension for Per<U> {
//     const L: i8 = -U::L;
//     const M: i8 = -U::M;
//     const T: i8 = -U::T;
//     // ... alle anderen Dimensionen
// }
pub struct Per<U>(PhantomData<U>);
impl<U: Dimension> Dimension for Per<U> {
    const L_DIM: i8 = -U::L_DIM; // Inverse length
    const M_DIM: i8 = -U::M_DIM; // Inverse mass
    const T_DIM: i8 = -U::T_DIM; // Inverse time
    const THETA_DIM: i8 = -U::THETA_DIM; // Inverse temperature
    const I_DIM: i8 = -U::I_DIM; // Inverse current
    const J_DIM: i8 = -U::J_DIM; // Inverse luminous intensity
    const N_DIM: i8 = -U::N_DIM; // Inverse amount of substance
}
