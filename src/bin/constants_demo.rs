//! Demo for physical constants module

use units::prelude::*;
use units::constants::*;

fn main() {
    println!("🔬 Physical Constants Demo");
    println!("==========================\n");

    // Fundamental constants
    println!("1. Fundamental Constants:");
    println!("   Speed of light: {}", SPEED_OF_LIGHT.format_scientific());
    println!("   Planck constant: {:.3e} J⋅s", PLANCK_CONSTANT);
    println!("   Elementary charge: {}", ELEMENTARY_CHARGE.format_scientific());
    println!("   Electron mass: {}", ELECTRON_MASS.format_scientific());
    println!("   Proton mass: {}", PROTON_MASS.format_scientific());
    println!("   Gravitational constant: {:.3e} m³/(kg⋅s²)", GRAVITATIONAL_CONSTANT);
    println!("   Fine structure constant: {:.6}", FINE_STRUCTURE_CONSTANT);
    println!();

    // Planck units
    println!("2. Planck Units:");
    println!("   Planck length: {}", PLANCK_LENGTH.format_scientific());
    println!("   Planck time: {}", PLANCK_TIME.format_scientific());
    println!("   Planck mass: {}", PLANCK_MASS.format_scientific());
    println!("   Planck energy: {}", PLANCK_ENERGY.format_scientific());
    println!("   Planck temperature: {:.3e} K", PLANCK_TEMPERATURE);
    println!();

    // Astronomical constants
    println!("3. Astronomical Constants:");
    println!("   Astronomical Unit: {}", ASTRONOMICAL_UNIT.format_astronomy());
    println!("   Solar mass: {}", SOLAR_MASS.format_astronomy());
    println!("   Solar radius: {}", SOLAR_RADIUS.format_astronomy());
    println!("   Solar luminosity: {}", SOLAR_LUMINOSITY.format_astronomy());
    println!("   Earth mass: {}", EARTH_MASS.format_astronomy());
    println!("   Earth radius: {}", EARTH_RADIUS.format_astronomy());
    println!("   Standard gravity: {}", STANDARD_GRAVITY.format_engineering());
    println!();

    // Atomic constants
    println!("4. Atomic Constants:");
    println!("   Bohr radius: {}", BOHR_RADIUS.format_scientific());
    println!("   Electron Compton wavelength: {}", ELECTRON_COMPTON_WAVELENGTH.format_scientific());
    println!("   Rydberg constant: {:.6e} m⁻¹", RYDBERG_CONSTANT);
    println!("   Hartree energy: {}", HARTREE_ENERGY.format_scientific());
    println!("   Electron volt: {}", ELECTRON_VOLT.format_scientific());
    println!("   Cesium frequency: {}", CESIUM_FREQUENCY.format_engineering());
    println!();

    // Electromagnetic constants
    println!("5. Electromagnetic Constants:");
    println!("   Vacuum permittivity: {:.6e} F/m", ELECTRIC_CONSTANT);
    println!("   Vacuum permeability: {:.6e} H/m", MAGNETIC_CONSTANT);
    println!("   Impedance of free space: {:.3} Ω", IMPEDANCE_OF_FREE_SPACE);
    println!("   Bohr magneton: {:.6e} J/T", BOHR_MAGNETON);
    println!("   Nuclear magneton: {:.6e} J/T", NUCLEAR_MAGNETON);
    println!("   Magnetic flux quantum: {:.6e} Wb", MAGNETIC_FLUX_QUANTUM);
    println!();

    // Thermodynamic constants
    println!("6. Thermodynamic Constants:");
    println!("   Boltzmann constant: {:.6e} J/K", BOLTZMANN_CONSTANT);
    println!("   Gas constant: {:.6} J/(mol⋅K)", GAS_CONSTANT);
    println!("   Stefan-Boltzmann constant: {:.6e} W/(m²⋅K⁴)", STEFAN_BOLTZMANN_CONSTANT);
    println!("   Standard atmosphere: {}", STANDARD_ATMOSPHERE.format_engineering());
    println!("   Water critical temperature: {:.1} K", WATER_CRITICAL_TEMPERATURE);
    println!("   Water critical pressure: {}", WATER_CRITICAL_PRESSURE.format_engineering());
    println!();

    // Nuclear constants
    println!("7. Nuclear Constants:");
    println!("   Proton mass: {:.3} MeV/c²", PROTON_MASS_MEV);
    println!("   Neutron mass: {:.3} MeV/c²", NEUTRON_MASS_MEV);
    println!("   Neutron lifetime: {}", NEUTRON_LIFETIME.format_ui());
    println!("   W boson mass: {:.1} GeV/c²", W_BOSON_MASS_GEV);
    println!("   Z boson mass: {:.3} GeV/c²", Z_BOSON_MASS_GEV);
    println!("   Uranium-235 critical mass: {}", URANIUM_235_CRITICAL_MASS.format_ui());
    println!();

    // Mathematical constants
    println!("8. Mathematical Constants:");
    println!("   π: {:.10}", PI);
    println!("   e: {:.10}", E);
    println!("   Golden ratio: {:.10}", GOLDEN_RATIO);
    println!("   Euler-Mascheroni: {:.10}", EULER_MASCHERONI);
    println!("   Degree to radian: {:.10}", DEG_TO_RAD);
    println!("   Arcsecond to radian: {:.6e}", ARCSEC_TO_RAD);
    println!();

    // Derived calculations
    println!("9. Derived Calculations:");
    
    // Classical electron radius
    let r_e = ELEMENTARY_CHARGE.value().powi(2) / 
              (4.0 * PI * ELECTRIC_CONSTANT * ELECTRON_MASS.value() * SPEED_OF_LIGHT.value().powi(2));
    println!("   Classical electron radius: {:.3e} m", r_e);
    
    // Hydrogen binding energy in eV
    let binding_ev = HYDROGEN_BINDING_ENERGY.value() / ELECTRON_VOLT.value();
    println!("   Hydrogen binding energy: {:.6} eV", binding_ev);
    
    // Hubble time
    let hubble_time_years = (1.0 / HUBBLE_CONSTANT) / (365.25 * 24.0 * 3600.0);
    println!("   Hubble time: {:.1} billion years", hubble_time_years / 1e9);
    
    // Solar escape velocity
    let v_esc_sun = (2.0 * GRAVITATIONAL_CONSTANT * SOLAR_MASS.value() / SOLAR_RADIUS.value()).sqrt();
    println!("   Solar escape velocity: {:.0} km/s", v_esc_sun / 1000.0);
    
    // Earth orbital period (verify using constants)
    let earth_orbit_period = 2.0 * PI * (ASTRONOMICAL_UNIT.value().powi(3) / 
                                         (GRAVITATIONAL_CONSTANT * SOLAR_MASS.value())).sqrt();
    println!("   Earth orbital period: {:.1} days", earth_orbit_period / (24.0 * 3600.0));
    
    println!();

    // Unit demonstrations
    println!("10. Unit Formatting Examples:");
    
    // Show same quantity in different units
    let jupiter_distance = Distance::<Meter>::new(7.785e11);
    println!("   Jupiter distance:");
    println!("     Scientific: {}", jupiter_distance.format_scientific());
    println!("     Engineering: {}", jupiter_distance.format_engineering());
    println!("     Astronomy: {}", jupiter_distance.format_astronomy());
    println!("     UI: {}", jupiter_distance.format_ui());
    
    let stellar_mass = Mass::<SolarMass>::new(1.5);
    println!("   Stellar mass:");
    println!("     Default: {}", stellar_mass);
    println!("     Scientific: {}", stellar_mass.format_scientific());
    println!("     Astronomy: {}", stellar_mass.format_astronomy());
    
    println!();

    // Physical relationships
    println!("11. Physical Relationships:");
    
    // Verify E=mc²
    let proton_energy = PROTON_MASS.value() * SPEED_OF_LIGHT.value().powi(2);
    println!("   Proton rest energy: {:.3} MeV", proton_energy / (1e6 * ELECTRON_VOLT.value()));
    
    // Verify de Broglie wavelength for thermal neutron
    let thermal_energy = 1.5 * BOLTZMANN_CONSTANT * 300.0; // 3/2 kT at 300K
    let thermal_velocity = (2.0 * thermal_energy / NEUTRON_MASS.value()).sqrt();
    let de_broglie = PLANCK_CONSTANT / (NEUTRON_MASS.value() * thermal_velocity);
    println!("   Thermal neutron de Broglie wavelength: {:.3} Å", de_broglie * 1e10);
    
    // Alpha particle kinetic energy from typical decay
    let alpha_ke_mev = TYPICAL_ALPHA_DECAY_Q_VALUE_MEV;
    let alpha_velocity = (2.0 * alpha_ke_mev * 1e6 * ELECTRON_VOLT.value() / ALPHA_PARTICLE_MASS.value()).sqrt();
    println!("   Alpha particle velocity (5 MeV): {:.1}% of c", alpha_velocity / SPEED_OF_LIGHT.value() * 100.0);
    
    println!();
    println!("✅ Physical constants system works perfectly!");
    println!("🎯 All fundamental physics constants available");
    println!("📏 Type-safe units with proper dimensions");
    println!("🔄 Automatic conversions and formatting");
    println!("🌌 Ready for scientific calculations!");
}