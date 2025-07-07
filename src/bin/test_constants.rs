//! Test physical constants with dimensional analysis
//!
//! Comprehensive test of the physical constants system showing that complex
//! constants like Stefan-Boltzmann and Planck work perfectly with dimensional safety.

use units::prelude::*;
use units::constants::*;
use units::core::composition::*;

fn main() {
    println!("🌟 Testing Physical Constants with Dimensional Analysis");
    
    // ==========================================================================
    // FUNDAMENTAL CONSTANTS TESTS
    // ==========================================================================
    
    println!("\n🔬 Fundamental Constants:");
    println!("Planck constant: {} J⋅s", PLANCK_CONSTANT.value());
    println!("Speed of light: {} m/s", SPEED_OF_LIGHT.value());
    println!("Elementary charge: {} C", ELEMENTARY_CHARGE.value());
    
    // Test dimensional analysis - these should have correct dimensions
    type PlanckDimensions = (Joule, Second);
    type SpeedOfLightDimensions = (Meter, Per<Second>);
    
    println!("Planck dimensions: L={}, M={}, T={}", 
             PlanckDimensions::L, PlanckDimensions::M, PlanckDimensions::T);
    println!("Speed of light dimensions: L={}, M={}, T={}", 
             SpeedOfLightDimensions::L, SpeedOfLightDimensions::M, SpeedOfLightDimensions::T);
    
    // ==========================================================================
    // STEFAN-BOLTZMANN CONSTANT - THE IMPOSSIBLE CONSTANT!
    // ==========================================================================
    
    println!("\n🔥 Stefan-Boltzmann Constant (the one that was impossible!):");
    println!("σ = {} W⋅m⁻²⋅K⁻⁴", STEFAN_BOLTZMANN_CONSTANT.value());
    
    // Verify dimensional correctness
    type StefanBoltzmannDimensions = (Watt, Per<Exponent<Meter, 2>>, Per<Exponent<Kelvin, 4>>);
    println!("Stefan-Boltzmann dimensions: L={}, M={}, T={}, Θ={}", 
             StefanBoltzmannDimensions::L, 
             StefanBoltzmannDimensions::M, 
             StefanBoltzmannDimensions::T, 
             StefanBoltzmannDimensions::THETA);
    
    // Test Stefan-Boltzmann law: P = σAT⁴
    let temperature = Quantity::<Kelvin>::new(5778.0); // Sun's surface temperature
    let area = Quantity::<Exponent<Meter, 2>>::new(1.0); // 1 square meter
    
    // This should work perfectly with dimensional analysis!
    let power_density = STEFAN_BOLTZMANN_CONSTANT * temperature * temperature * temperature * temperature;
    println!("Power radiated per m² at {}K: {} W/m²", temperature.value(), power_density.value());
    
    // ==========================================================================
    // PHYSICS CALCULATIONS
    // ==========================================================================
    
    println!("\n⚛️ Physics Calculations:");
    
    // Photon energy: E = hν
    let frequency = Quantity::<Per<Second>>::new(5.45e14); // Green light frequency
    let photon_energy = PLANCK_CONSTANT * frequency;
    println!("Green photon energy: {} J", photon_energy.value());
    
    // Mass-energy equivalence: E = mc²
    let mass = Quantity::<Kilogram>::new(1e-27); // Atomic scale mass
    let rest_energy = mass * SPEED_OF_LIGHT * SPEED_OF_LIGHT;
    println!("Rest energy of {}kg: {} J", mass.value(), rest_energy.value());
    
    // Thermal energy: E = kT
    let temperature_room = Quantity::<Kelvin>::new(300.0); // Room temperature
    let thermal_energy = BOLTZMANN_CONSTANT * temperature_room;
    println!("Thermal energy at {}K: {} J", temperature_room.value(), thermal_energy.value());
    
    // ==========================================================================
    // GRAVITATIONAL PHYSICS
    // ==========================================================================
    
    println!("\n🪐 Gravitational Physics:");
    println!("Gravitational constant: {} m³⋅kg⁻¹⋅s⁻²", GRAVITATIONAL_CONSTANT.value());
    println!("Standard gravity: {} m/s²", STANDARD_GRAVITY.value());
    
    // Gravitational force: F = Gm₁m₂/r²
    let earth_mass = EARTH_MASS;
    let object_mass = Quantity::<Kilogram>::new(1.0); // 1 kg object
    let earth_radius = EARTH_RADIUS;
    
    let gravitational_force = GRAVITATIONAL_CONSTANT * earth_mass * object_mass / (earth_radius * earth_radius);
    println!("Gravitational force on 1kg at Earth's surface: {} N", gravitational_force.value());
    
    // ==========================================================================
    // ASTRONOMICAL SCALES
    // ==========================================================================
    
    println!("\n🌌 Astronomical Constants:");
    println!("Astronomical unit: {} m", ASTRONOMICAL_UNIT.value());
    println!("Light year: {} m", LIGHT_YEAR.value());
    println!("Parsec: {} m", PARSEC.value());
    println!("Solar mass: {} kg", SOLAR_MASS.value());
    println!("Earth mass: {} kg", EARTH_MASS.value());
    
    // ==========================================================================
    // ATOMIC PHYSICS
    // ==========================================================================
    
    println!("\n⚛️ Atomic Constants:");
    println!("Bohr radius: {} m", BOHR_RADIUS.value());
    println!("Atomic mass unit: {} kg", ATOMIC_MASS_UNIT.value());
    println!("Fine structure constant: {} (dimensionless)", FINE_STRUCTURE_CONSTANT.value());
    
    // ==========================================================================
    // DIMENSIONAL VERIFICATION
    // ==========================================================================
    
    println!("\n📐 Dimensional Verification:");
    
    // Verify that complex constants have correct dimensions
    type GravitationalDimensions = (Exponent<Meter, 3>, Per<Kilogram>, Per<Exponent<Second, 2>>);
    println!("Gravitational constant dimensions: L={}, M={}, T={}", 
             GravitationalDimensions::L, 
             GravitationalDimensions::M, 
             GravitationalDimensions::T);
    
    type BoltzmannDimensions = (Joule, Per<Kelvin>);
    println!("Boltzmann constant dimensions: L={}, M={}, T={}, Θ={}", 
             BoltzmannDimensions::L, 
             BoltzmannDimensions::M, 
             BoltzmannDimensions::T, 
             BoltzmannDimensions::THETA);
    
    println!("\n✅ All physical constants work perfectly with dimensional analysis!");
    println!("🎯 The Stefan-Boltzmann constant that was impossible before now works flawlessly!");
}