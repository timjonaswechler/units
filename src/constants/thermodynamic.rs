//! Thermodynamic constants
//!
//! Constants related to thermodynamics and statistical mechanics

use crate::prelude::*;

/// Boltzmann constant: k = 1.380649×10⁻²³ J/K (exact)
pub const BOLTZMANN_CONSTANT: f64 = 1.380649e-23;

/// Gas constant: R = 8.314462618 J/(mol⋅K) (exact)
pub const GAS_CONSTANT: f64 = 8.314462618;

/// Stefan-Boltzmann constant: σ = 5.670374419×10⁻⁸ W/(m²⋅K⁴)
pub const STEFAN_BOLTZMANN_CONSTANT: f64 = 5.670374419e-8;

/// Wien displacement law constant: b = 2.897771955×10⁻³ m⋅K
pub const WIEN_DISPLACEMENT_CONSTANT: f64 = 2.897771955e-3;

/// First radiation constant: c₁ = 2πhc² = 3.741771852×10⁻¹⁶ W⋅m²
pub const FIRST_RADIATION_CONSTANT: f64 = 3.741771852e-16;

/// Second radiation constant: c₂ = hc/k = 1.438776877×10⁻² m⋅K
pub const SECOND_RADIATION_CONSTANT: f64 = 1.438776877e-2;

/// Loschmidt constant at STP: n₀ = 2.686780111×10²⁵ m⁻³
pub const LOSCHMIDT_CONSTANT_STP: f64 = 2.686780111e25;

/// Standard atmospheric pressure: 1 atm = 101325 Pa (exact)
pub const STANDARD_ATMOSPHERE: Pressure<Pascal> = Pressure::new(101_325.0);

/// Standard temperature: T₀ = 273.15 K (exact)
pub const STANDARD_TEMPERATURE: f64 = 273.15;

/// Standard pressure: P₀ = 100000 Pa = 1 bar (exact)
pub const STANDARD_PRESSURE: Pressure<Pascal> = Pressure::new(100_000.0);

/// Molar volume of ideal gas at STP: Vm = 22.41396954×10⁻³ m³/mol
pub const MOLAR_VOLUME_STP: f64 = 22.41396954e-3;

/// Molar volume of ideal gas at SATP: Vm = 24.46954×10⁻³ m³/mol
pub const MOLAR_VOLUME_SATP: f64 = 24.46954e-3;

/// Specific gas constant for dry air: R_air = 287.052874 J/(kg⋅K)
pub const SPECIFIC_GAS_CONSTANT_AIR: f64 = 287.052874;

/// Heat capacity ratio for air: γ_air = 1.4 (diatomic gas)
pub const HEAT_CAPACITY_RATIO_AIR: f64 = 1.4;

/// Heat capacity ratio for monatomic gas: γ = 5/3 ≈ 1.6667
pub const HEAT_CAPACITY_RATIO_MONATOMIC: f64 = 5.0 / 3.0;

/// Heat capacity ratio for diatomic gas: γ = 7/5 = 1.4
pub const HEAT_CAPACITY_RATIO_DIATOMIC: f64 = 1.4;

/// Ice point temperature: T_ice = 273.15 K (exact)
pub const ICE_POINT: f64 = 273.15;

/// Steam point temperature: T_steam = 373.15 K
pub const STEAM_POINT: f64 = 373.15;

/// Triple point of water: T_tp = 273.16 K (exact)
pub const WATER_TRIPLE_POINT: f64 = 273.16;

/// Critical temperature of water: T_c = 647.096 K
pub const WATER_CRITICAL_TEMPERATURE: f64 = 647.096;

/// Critical pressure of water: P_c = 22.064×10⁶ Pa
pub const WATER_CRITICAL_PRESSURE: Pressure<Pascal> = Pressure::new(22.064e6);

/// Critical density of water: ρ_c = 322.0 kg/m³
pub const WATER_CRITICAL_DENSITY: Density<KilogramPerCubicMeter> = Density::new(322.0);

/// Density of water at 4°C: ρ_w = 999.972 kg/m³
pub const WATER_DENSITY_4C: Density<KilogramPerCubicMeter> = Density::new(999.972);

/// Thermal conductivity of vacuum (zero): k_vac = 0 W/(m⋅K)
pub const THERMAL_CONDUCTIVITY_VACUUM: f64 = 0.0;

/// Thermal conductivity of air at 20°C: k_air ≈ 0.02587 W/(m⋅K)
pub const THERMAL_CONDUCTIVITY_AIR_20C: f64 = 0.02587;

/// Thermal conductivity of water at 20°C: k_water ≈ 0.598 W/(m⋅K)
pub const THERMAL_CONDUCTIVITY_WATER_20C: f64 = 0.598;

/// Thermal conductivity of copper: k_Cu ≈ 401 W/(m⋅K)
pub const THERMAL_CONDUCTIVITY_COPPER: f64 = 401.0;

/// Dynamic viscosity of air at 20°C: μ_air ≈ 1.825×10⁻⁵ Pa⋅s
pub const DYNAMIC_VISCOSITY_AIR_20C: f64 = 1.825e-5;

/// Dynamic viscosity of water at 20°C: μ_water ≈ 1.002×10⁻³ Pa⋅s
pub const DYNAMIC_VISCOSITY_WATER_20C: f64 = 1.002e-3;

/// Kinematic viscosity of air at 20°C: ν_air ≈ 1.516×10⁻⁵ m²/s
pub const KINEMATIC_VISCOSITY_AIR_20C: f64 = 1.516e-5;

/// Kinematic viscosity of water at 20°C: ν_water ≈ 1.004×10⁻⁶ m²/s
pub const KINEMATIC_VISCOSITY_WATER_20C: f64 = 1.004e-6;

/// Surface tension of water at 20°C: γ_water ≈ 0.0728 N/m
pub const SURFACE_TENSION_WATER_20C: f64 = 0.0728;

/// Specific heat capacity of air at constant pressure: cp_air = 1005 J/(kg⋅K)
pub const SPECIFIC_HEAT_AIR_CP: f64 = 1005.0;

/// Specific heat capacity of air at constant volume: cv_air = 718 J/(kg⋅K)
pub const SPECIFIC_HEAT_AIR_CV: f64 = 718.0;

/// Specific heat capacity of water: cp_water = 4182 J/(kg⋅K)
pub const SPECIFIC_HEAT_WATER: f64 = 4182.0;

/// Latent heat of vaporization of water: L_v = 2.257×10⁶ J/kg
pub const LATENT_HEAT_VAPORIZATION_WATER: f64 = 2.257e6;

/// Latent heat of fusion of water: L_f = 334×10³ J/kg
pub const LATENT_HEAT_FUSION_WATER: f64 = 334e3;

/// Enthalpy of formation of water vapor: ΔH_f = -241.83×10³ J/mol
pub const ENTHALPY_FORMATION_WATER_VAPOR: f64 = -241.83e3;

/// Enthalpy of formation of liquid water: ΔH_f = -285.83×10³ J/mol
pub const ENTHALPY_FORMATION_WATER_LIQUID: f64 = -285.83e3;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::fundamental::*;

    #[test]
    fn test_gas_constant_from_boltzmann() {
        // R = NAk
        let calculated = AVOGADRO_CONSTANT * BOLTZMANN_CONSTANT;
        assert!((GAS_CONSTANT - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_stefan_boltzmann_constant() {
        // σ = (2π⁵k⁴)/(15h³c²)
        let k = BOLTZMANN_CONSTANT;
        let h = PLANCK_CONSTANT;
        let c = SPEED_OF_LIGHT.value();
        let pi = std::f64::consts::PI;
        
        let calculated = (2.0 * pi.powi(5) * k.powi(4)) / (15.0 * h.powi(3) * c.powi(2));
        assert!((STEFAN_BOLTZMANN_CONSTANT - calculated).abs() / calculated < 1e-6);
    }

    #[test]
    fn test_wien_displacement_constant() {
        // b = hc/(4.965kB) ≈ 2.898×10⁻³ m⋅K
        let h = PLANCK_CONSTANT;
        let c = SPEED_OF_LIGHT.value();
        let k = BOLTZMANN_CONSTANT;
        
        let calculated = h * c / (4.965 * k);
        assert!((WIEN_DISPLACEMENT_CONSTANT - calculated).abs() / calculated < 1e-3);
    }

    #[test]
    fn test_first_radiation_constant() {
        // c₁ = 2πhc²
        let h = PLANCK_CONSTANT;
        let c = SPEED_OF_LIGHT.value();
        let pi = std::f64::consts::PI;
        
        let calculated = 2.0 * pi * h * c.powi(2);
        assert!((FIRST_RADIATION_CONSTANT - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_second_radiation_constant() {
        // c₂ = hc/k
        let h = PLANCK_CONSTANT;
        let c = SPEED_OF_LIGHT.value();
        let k = BOLTZMANN_CONSTANT;
        
        let calculated = h * c / k;
        assert!((SECOND_RADIATION_CONSTANT - calculated).abs() / calculated < 1e-10);
    }

    #[test]
    fn test_standard_atmosphere() {
        assert_eq!(STANDARD_ATMOSPHERE.value(), 101_325.0);
    }

    #[test]
    fn test_standard_temperature() {
        assert_eq!(STANDARD_TEMPERATURE, 273.15);
    }

    #[test]
    fn test_ideal_gas_law_stp() {
        // PV = nRT, so V/n = RT/P
        let r = GAS_CONSTANT;
        let t = STANDARD_TEMPERATURE;
        let p = STANDARD_ATMOSPHERE.value();
        
        let calculated_molar_volume = r * t / p;
        assert!((MOLAR_VOLUME_STP - calculated_molar_volume).abs() / calculated_molar_volume < 1e-6);
    }

    #[test]
    fn test_heat_capacity_ratios() {
        // For monatomic: γ = 5/3
        assert!((HEAT_CAPACITY_RATIO_MONATOMIC - 5.0/3.0).abs() < 1e-10);
        
        // For diatomic: γ = 7/5
        assert!((HEAT_CAPACITY_RATIO_DIATOMIC - 7.0/5.0).abs() < 1e-10);
    }

    #[test]
    fn test_air_heat_capacity_ratio() {
        // γ = cp/cv
        let gamma = SPECIFIC_HEAT_AIR_CP / SPECIFIC_HEAT_AIR_CV;
        assert!((gamma - HEAT_CAPACITY_RATIO_AIR).abs() < 0.01);
    }

    #[test]
    fn test_water_phase_transition_temperatures() {
        // Ice point should be less than steam point
        assert!(ICE_POINT < STEAM_POINT);
        
        // Triple point should be very close to ice point
        assert!((WATER_TRIPLE_POINT - ICE_POINT).abs() < 0.1);
        
        // Critical temperature should be much higher
        assert!(WATER_CRITICAL_TEMPERATURE > 600.0);
    }

    #[test]
    fn test_thermal_conductivity_ordering() {
        // Vacuum < Air < Water < Copper
        assert!(THERMAL_CONDUCTIVITY_VACUUM < THERMAL_CONDUCTIVITY_AIR_20C);
        assert!(THERMAL_CONDUCTIVITY_AIR_20C < THERMAL_CONDUCTIVITY_WATER_20C);
        assert!(THERMAL_CONDUCTIVITY_WATER_20C < THERMAL_CONDUCTIVITY_COPPER);
    }

    #[test]
    fn test_viscosity_values() {
        // Air should be less viscous than water
        assert!(DYNAMIC_VISCOSITY_AIR_20C < DYNAMIC_VISCOSITY_WATER_20C);
        assert!(KINEMATIC_VISCOSITY_AIR_20C > KINEMATIC_VISCOSITY_WATER_20C); // Due to density
    }

    #[test]
    fn test_water_specific_properties() {
        // Water should have high specific heat
        assert!(SPECIFIC_HEAT_WATER > 4000.0);
        
        // Latent heats should be positive and reasonable
        assert!(LATENT_HEAT_VAPORIZATION_WATER > 2e6);
        assert!(LATENT_HEAT_FUSION_WATER > 3e5);
        assert!(LATENT_HEAT_VAPORIZATION_WATER > LATENT_HEAT_FUSION_WATER);
    }
}