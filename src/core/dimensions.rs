// Dimensional trait with const generic parameters for automatic validation
pub trait Dimension<
    const L: i8,     // Length (m)
    const M: i8,     // Mass (kg)
    const T: i8,     // Time (s)
    const THETA: i8, // Temperature (K)
    const I: i8,     // Current (A)
    const J: i8,     // Luminous Intensity (cd)
    const N: i8,     // Amount of Substance (mol)
>
{
    // Dimensions are now encoded in the type, accessible via const generics
    const L_DIM: i8 = L;
    const M_DIM: i8 = M;
    const T_DIM: i8 = T;
    const THETA_DIM: i8 = THETA;
    const I_DIM: i8 = I;
    const J_DIM: i8 = J;
    const N_DIM: i8 = N;
}
