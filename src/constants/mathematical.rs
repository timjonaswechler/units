//! Mathematical constants
//!
//! Mathematical constants commonly used in physics

/// π: The ratio of a circle's circumference to its diameter
pub const PI: f64 = std::f64::consts::PI;

/// τ = 2π: The ratio of a circle's circumference to its radius
pub const TAU: f64 = 2.0 * std::f64::consts::PI;

/// e: Euler's number, base of natural logarithm
pub const E: f64 = std::f64::consts::E;

/// φ: Golden ratio = (1 + √5)/2
pub const GOLDEN_RATIO: f64 = 1.618033988749895;

/// √2: Square root of 2
pub const SQRT_2: f64 = std::f64::consts::SQRT_2;

/// √3: Square root of 3
pub const SQRT_3: f64 = 1.7320508075688772;

/// √π: Square root of π
pub const SQRT_PI: f64 = 1.7724538509055159;

/// 1/√(2π): Normalization constant for Gaussian
pub const INV_SQRT_2PI: f64 = 0.39894228040143267;

/// ln(2): Natural logarithm of 2
pub const LN_2: f64 = std::f64::consts::LN_2;

/// ln(10): Natural logarithm of 10
pub const LN_10: f64 = std::f64::consts::LN_10;

/// log₂(e): Base-2 logarithm of e
pub const LOG2_E: f64 = std::f64::consts::LOG2_E;

/// log₁₀(e): Base-10 logarithm of e
pub const LOG10_E: f64 = std::f64::consts::LOG10_E;

/// γ: Euler-Mascheroni constant
pub const EULER_MASCHERONI: f64 = 0.5772156649015329;

/// Catalan's constant: β(2) = ∑((-1)ⁿ/(2n+1)²)
pub const CATALAN: f64 = 0.9159655941772190;

/// Apéry's constant: ζ(3) = ∑(1/n³)
pub const APERY: f64 = 1.2020569031595942;

/// Feigenbaum's first constant: δ
pub const FEIGENBAUM_DELTA: f64 = 4.669201609102990;

/// Feigenbaum's second constant: α
pub const FEIGENBAUM_ALPHA: f64 = 2.502907875095892;

/// Conway's constant: λ
pub const CONWAY: f64 = 1.303577269034296;

/// Khinchin's constant: K₀
pub const KHINCHIN: f64 = 2.685452001065306;

/// Glaisher-Kinkelin constant: A
pub const GLAISHER_KINKELIN: f64 = 1.282427129100623;

/// Lévy's constant: γ
pub const LEVY: f64 = 3.275822918721811;

/// Ramanujan-Soldner constant: μ
pub const RAMANUJAN_SOLDNER: f64 = 1.451369234883381;

/// Erdős-Borwein constant: E₁
pub const ERDOS_BORWEIN: f64 = 1.606695152415291;

/// Omega constant: Ω (solution to xe^x = 1)
pub const OMEGA: f64 = 0.5671432904097839;

/// Plastic number: ρ (real root of x³ = x + 1)
pub const PLASTIC: f64 = 1.324717957244746;

/// Backhouse's constant
pub const BACKHOUSE: f64 = 1.456074948582690;

/// Porter's constant
pub const PORTER: f64 = 1.467078079433975;

/// Lieb's square ice constant
pub const LIEB_SQUARE_ICE: f64 = 1.539600717839002;

/// Niven's constant: C
pub const NIVEN: f64 = 1.705211140105367;

/// Brun's constant for twin primes: B₂
pub const BRUN_TWIN_PRIMES: f64 = 1.902160583104;

/// Landau-Ramanujan constant: K
pub const LANDAU_RAMANUJAN: f64 = 0.764223653589220;

/// Gauss's constant: G = 1/AGM(1,√2)
pub const GAUSS: f64 = 0.834626841674073;

/// Lemniscate constant: ϖ = 2G
pub const LEMNISCATE: f64 = 2.0 * 0.834626841674073;

/// Second Hermite constant: γ₂
pub const HERMITE_2: f64 = 1.154700538379252;

/// Liouville's constant: L
pub const LIOUVILLE: f64 = 0.110001000000000;

/// Champernowne's constant: C₁₀
pub const CHAMPERNOWNE_10: f64 = 0.123456789101112;

/// Universal parabolic constant: P
pub const UNIVERSAL_PARABOLIC: f64 = 2.295587149392638;

/// Cahen's constant: C
pub const CAHEN: f64 = 0.643410546288338;

/// Meissel-Mertens constant: M
pub const MEISSEL_MERTENS: f64 = 0.261497212847643;

/// Twin prime constant: C₂
pub const TWIN_PRIME: f64 = 0.660161815846869;

/// Feller-Tornier constant: C_FT
pub const FELLER_TORNIER: f64 = 0.661317251701895;

/// Laplace limit: L
pub const LAPLACE_LIMIT: f64 = 0.662743419349181;

/// Alladi-Grinstead constant: A_G
pub const ALLADI_GRINSTEAD: f64 = 0.809394020540235;

/// Grothendieck constant: K_G
pub const GROTHENDIECK: f64 = 1.782591164532393;

/// Degrees to radians conversion factor: π/180
pub const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

/// Radians to degrees conversion factor: 180/π
pub const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;

/// Arcseconds to radians: π/(180×3600)
pub const ARCSEC_TO_RAD: f64 = std::f64::consts::PI / (180.0 * 3600.0);

/// Radians to arcseconds: (180×3600)/π
pub const RAD_TO_ARCSEC: f64 = (180.0 * 3600.0) / std::f64::consts::PI;

/// Arcminutes to radians: π/(180×60)
pub const ARCMIN_TO_RAD: f64 = std::f64::consts::PI / (180.0 * 60.0);

/// Radians to arcminutes: (180×60)/π
pub const RAD_TO_ARCMIN: f64 = (180.0 * 60.0) / std::f64::consts::PI;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_constants() {
        assert_eq!(PI, std::f64::consts::PI);
        assert_eq!(E, std::f64::consts::E);
        assert_eq!(TAU, 2.0 * PI);
    }

    #[test]
    fn test_golden_ratio() {
        // φ = (1 + √5)/2
        let calculated = (1.0 + 5.0_f64.sqrt()) / 2.0;
        assert!((GOLDEN_RATIO - calculated).abs() < 1e-15);
        
        // φ² = φ + 1
        let phi_squared = GOLDEN_RATIO * GOLDEN_RATIO;
        assert!((phi_squared - GOLDEN_RATIO - 1.0).abs() < 1e-14);
    }

    #[test]
    fn test_sqrt_constants() {
        assert_eq!(SQRT_2, std::f64::consts::SQRT_2);
        assert!((SQRT_3 - 3.0_f64.sqrt()).abs() < 1e-15);
        assert!((SQRT_PI - PI.sqrt()).abs() < 1e-15);
    }

    #[test]
    fn test_logarithm_constants() {
        assert_eq!(LN_2, std::f64::consts::LN_2);
        assert_eq!(LN_10, std::f64::consts::LN_10);
        assert_eq!(LOG2_E, std::f64::consts::LOG2_E);
        assert_eq!(LOG10_E, std::f64::consts::LOG10_E);
    }

    #[test]
    fn test_conversion_factors() {
        // Test degree/radian conversions
        assert!((DEG_TO_RAD - PI / 180.0).abs() < 1e-15);
        assert!((RAD_TO_DEG - 180.0 / PI).abs() < 1e-15);
        
        // Test that conversions are inverses
        assert!((DEG_TO_RAD * RAD_TO_DEG - 1.0).abs() < 1e-14);
        
        // Test arcsecond conversions
        assert!((ARCSEC_TO_RAD - PI / (180.0 * 3600.0)).abs() < 1e-15);
        assert!((RAD_TO_ARCSEC - (180.0 * 3600.0) / PI).abs() < 1e-15);
        assert!((ARCSEC_TO_RAD * RAD_TO_ARCSEC - 1.0).abs() < 1e-14);
    }

    #[test]
    fn test_euler_mascheroni() {
        // Euler-Mascheroni constant should be approximately 0.5772
        assert!(EULER_MASCHERONI > 0.577);
        assert!(EULER_MASCHERONI < 0.578);
    }

    #[test]
    fn test_special_constants() {
        // Test some well-known values
        assert!(CATALAN > 0.91 && CATALAN < 0.92);
        assert!(APERY > 1.20 && APERY < 1.21);
        assert!(FEIGENBAUM_DELTA > 4.66 && FEIGENBAUM_DELTA < 4.67);
    }

    #[test]
    fn test_gauss_constant() {
        // Gauss constant is related to the lemniscate constant
        assert!((LEMNISCATE - 2.0 * GAUSS).abs() < 1e-14);
    }

    #[test]
    fn test_omega_constant() {
        // Omega is the solution to x*e^x = 1
        let x = OMEGA;
        let result = x * x.exp();
        assert!((result - 1.0).abs() < 1e-14);
    }

    #[test]
    fn test_plastic_number() {
        // Plastic number is the real root of x³ = x + 1
        let x = PLASTIC;
        let result = x.powi(3) - x - 1.0;
        assert!(result.abs() < 1e-14);
    }

    #[test]
    fn test_normalization_constant() {
        // 1/√(2π) should be approximately 0.3989
        let calculated = 1.0 / (2.0 * PI).sqrt();
        assert!((INV_SQRT_2PI - calculated).abs() < 1e-15);
    }

    #[test]
    fn test_angle_unit_relationships() {
        // 1 degree = 60 arcminutes = 3600 arcseconds
        assert!((DEG_TO_RAD - 60.0 * ARCMIN_TO_RAD).abs() < 1e-15);
        assert!((DEG_TO_RAD - 3600.0 * ARCSEC_TO_RAD).abs() < 1e-15);
        assert!((ARCMIN_TO_RAD - 60.0 * ARCSEC_TO_RAD).abs() < 1e-15);
    }

    #[test]
    fn test_constant_ordering() {
        // Some basic ordering tests
        assert!(E > 2.7 && E < 2.8);
        assert!(PI > 3.1 && PI < 3.2);
        assert!(GOLDEN_RATIO > 1.6 && GOLDEN_RATIO < 1.7);
        assert!(SQRT_2 > 1.4 && SQRT_2 < 1.5);
        assert!(SQRT_3 > 1.7 && SQRT_3 < 1.8);
    }
}