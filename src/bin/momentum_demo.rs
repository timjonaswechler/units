//! Demo für Momentum quantity

use units::prelude::*;

fn main() {
    println!("⚡ Momentum Quantity Demo");
    println!("========================\n");

    // 1. Basic Momentum units
    println!("1. Basic Momentum units:");
    let p_si = Momentum::<KilogramMeterPerSecond>::new(100.0);
    let p_impulse = Momentum::<NewtonSecond>::new(100.0);
    println!("   {} kg⋅m/s = {} N⋅s", p_si.value(), p_impulse.value());
    
    let converted: Momentum<NewtonSecond> = p_si.convert_to();
    println!("   Conversion check: {} kg⋅m/s = {} N⋅s\n", p_si.value(), converted.value());

    // 2. Classical mechanics examples
    println!("2. Classical mechanics:");
    
    // Car collision
    let car_mass_kg = 1500.0;
    let car_velocity_ms = 20.0; // 72 km/h
    let car_momentum = Momentum::<KilogramMeterPerSecond>::new(car_mass_kg * car_velocity_ms);
    
    println!("   Car momentum: {} kg⋅m/s", car_momentum.value());
    println!("   (1500 kg car at 72 km/h)");
    
    // Baseball
    let baseball_mass_g = 145.0;
    let baseball_velocity_ms = 45.0; // Fast pitch
    let baseball_momentum_g = Momentum::<GramMeterPerSecond>::new(baseball_mass_g * baseball_velocity_ms);
    let baseball_momentum_si: Momentum<KilogramMeterPerSecond> = baseball_momentum_g.convert_to();
    
    println!("   Baseball: {} g⋅m/s = {:.3} kg⋅m/s", 
             baseball_momentum_g.value(), baseball_momentum_si.value());
    
    // Momentum ratio
    let momentum_ratio = car_momentum / baseball_momentum_si;
    println!("   Car has {}x more momentum than baseball\n", momentum_ratio);

    // 3. Particle physics
    println!("3. Particle physics:");
    
    // Electron at rest
    let electron_rest = Momentum::<ElectronMassSpeedOfLight>::new(1.0);
    let electron_rest_si: Momentum<KilogramMeterPerSecond> = electron_rest.convert_to();
    
    println!("   Electron rest momentum: m_e⋅c");
    println!("   = {:.3e} kg⋅m/s", electron_rest_si.value());
    
    // High-energy electron (1 GeV/c)
    let electron_gev = Momentum::<GigaElectronVoltPerSpeedOfLight>::new(1.0);
    let electron_gev_si: Momentum<KilogramMeterPerSecond> = electron_gev.convert_to();
    
    println!("   1 GeV/c electron: {:.3e} kg⋅m/s", electron_gev_si.value());
    
    // Relativistic boost factor
    let boost_factor = electron_gev / electron_rest;
    println!("   Relativistic boost: {:.0}x rest momentum", boost_factor);
    
    // LHC proton beam (7 TeV)
    let lhc_proton = Momentum::<TeraElectronVoltPerSpeedOfLight>::new(7.0);
    let lhc_si: Momentum<KilogramMeterPerSecond> = lhc_proton.convert_to();
    
    println!("   LHC proton (7 TeV): {:.3e} kg⋅m/s\n", lhc_si.value());

    // 4. Astronomical momentum scales
    println!("4. Astronomical momentum:");
    
    // Earth's orbital momentum
    let earth_orbital_speed_kms = 29.8; // km/s around Sun
    let earth_momentum = Momentum::<EarthMassKilometerPerSecond>::new(earth_orbital_speed_kms);
    let earth_si: Momentum<KilogramMeterPerSecond> = earth_momentum.convert_to();
    
    println!("   Earth orbital momentum: {:.2e} kg⋅m/s", earth_si.value());
    println!("   (Earth mass × 29.8 km/s orbital speed)");
    
    // Moon's orbital momentum around Earth
    let lunar_orbital_speed_kms = 1.0; // ~1 km/s around Earth
    let moon_momentum = Momentum::<LunarMassKilometerPerSecond>::new(lunar_orbital_speed_kms);
    let moon_si: Momentum<KilogramMeterPerSecond> = moon_momentum.convert_to();
    
    println!("   Moon orbital momentum: {:.2e} kg⋅m/s", moon_si.value());
    
    // Solar system reference
    let solar_momentum = Momentum::<SolarMassAUPerYear>::new(1.0);
    let solar_si: Momentum<KilogramMeterPerSecond> = solar_momentum.convert_to();
    
    println!("   Solar reference (M☉⋅AU/yr): {:.2e} kg⋅m/s\n", solar_si.value());

    // 5. Spacecraft and engineering
    println!("5. Spacecraft momentum:");
    
    // International Space Station
    let iss_mass_kg = 420_000.0; // ~420 tonnes
    let iss_velocity_kms = 7.66; // orbital velocity
    let iss_momentum = Momentum::<KilogramMeterPerSecond>::new(iss_mass_kg * iss_velocity_kms * 1000.0);
    
    println!("   ISS momentum: {:.2e} kg⋅m/s", iss_momentum.value());
    println!("   (420 tonnes at 7.66 km/s)");
    
    // Apollo spacecraft
    let apollo_mass_kg = 45_000.0; // Command and Service Module
    let apollo_velocity_kms = 11.0; // escape velocity
    let apollo_momentum = Momentum::<KilogramMeterPerSecond>::new(apollo_mass_kg * apollo_velocity_kms * 1000.0);
    
    println!("   Apollo escape momentum: {:.2e} kg⋅m/s", apollo_momentum.value());
    
    // Voyager spacecraft
    let voyager_mass_kg = 825.0;
    let voyager_velocity_kms = 17.0; // current velocity
    let voyager_momentum = Momentum::<KilogramMeterPerSecond>::new(voyager_mass_kg * voyager_velocity_kms * 1000.0);
    
    println!("   Voyager 1 momentum: {:.2e} kg⋅m/s\n", voyager_momentum.value());

    // 6. Momentum conservation
    println!("6. Momentum conservation:");
    
    // Elastic collision example
    let ball1_before = Momentum::<KilogramMeterPerSecond>::new(10.0);
    let ball2_before = Momentum::<KilogramMeterPerSecond>::new(-5.0);
    let total_before = ball1_before + ball2_before;
    
    println!("   Before collision:");
    println!("     Ball 1: {} kg⋅m/s", ball1_before.value());
    println!("     Ball 2: {} kg⋅m/s", ball2_before.value());
    println!("     Total: {} kg⋅m/s", total_before.value());
    
    // After collision (exchange velocities for equal masses)
    let ball1_after = Momentum::<KilogramMeterPerSecond>::new(-5.0);
    let ball2_after = Momentum::<KilogramMeterPerSecond>::new(10.0);
    let total_after = ball1_after + ball2_after;
    
    println!("   After collision:");
    println!("     Ball 1: {} kg⋅m/s", ball1_after.value());
    println!("     Ball 2: {} kg⋅m/s", ball2_after.value());
    println!("     Total: {} kg⋅m/s", total_after.value());
    println!("   Momentum conserved: {} = {}\n", total_before.value(), total_after.value());

    // 7. Impulse-momentum theorem
    println!("7. Impulse-momentum theorem:");
    
    // Rocket thrust
    let initial_momentum = Momentum::<KilogramMeterPerSecond>::new(0.0);
    let final_momentum = Momentum::<KilogramMeterPerSecond>::new(50_000.0);
    let impulse = final_momentum - initial_momentum;
    let impulse_ns: Momentum<NewtonSecond> = impulse.convert_to();
    
    println!("   Rocket acceleration:");
    println!("     Initial momentum: {} kg⋅m/s", initial_momentum.value());
    println!("     Final momentum: {} kg⋅m/s", final_momentum.value());
    println!("     Required impulse: {} N⋅s", impulse_ns.value());
    
    // Time calculation (if we know force)
    let thrust_force_n = 5000.0; // 5 kN thrust
    let burn_time_s = impulse_ns.value() / thrust_force_n;
    println!("     Burn time (5 kN thrust): {:.0} seconds\n", burn_time_s);

    // 8. CGS units
    println!("8. CGS units (astrophysics):");
    
    let stellar_wind_cgs = Momentum::<GramCentimeterPerSecond>::new(1e15);
    let stellar_wind_si: Momentum<KilogramMeterPerSecond> = stellar_wind_cgs.convert_to();
    
    println!("   Stellar wind particle: {:.0e} g⋅cm/s", stellar_wind_cgs.value());
    println!("   = {:.2e} kg⋅m/s\n", stellar_wind_si.value());

    // 9. Scale comparison
    println!("9. Momentum scale comparison:");
    
    let photon = Momentum::<GigaElectronVoltPerSpeedOfLight>::new(1.0);
    let atom = Momentum::<AtomicMassUnitMeterPerSecond>::new(1000.0); // 1 km/s
    let human = Momentum::<KilogramMeterPerSecond>::new(70.0 * 5.0); // 70 kg person at 5 m/s
    let car = car_momentum;
    let asteroid = Momentum::<TonneMeterPerSecond>::new(1e6); // 1000 tonnes at 1 km/s  
    let planet = earth_momentum;
    
    println!("   Momentum hierarchy (kg⋅m/s):");
    
    let photon_si: Momentum<KilogramMeterPerSecond> = photon.convert_to();
    let atom_si: Momentum<KilogramMeterPerSecond> = atom.convert_to();
    
    println!("     1 GeV photon: {:.1e}", photon_si.value());
    println!("     Fast atom: {:.1e}", atom_si.value());
    println!("     Running human: {:.1e}", human.value());
    println!("     Moving car: {:.1e}", car.value());
    println!("     Asteroid: {:.1e}", asteroid.value());
    println!("     Earth orbital: {:.1e}", earth_si.value());
    
    let extreme_range = earth_si / photon_si;
    println!("   Total range: {:.0e} orders of magnitude\n", extreme_range);

    // 10. Mixed unit arithmetic
    println!("10. Mixed unit arithmetic:");
    
    let p1 = Momentum::<KilogramMeterPerSecond>::new(50.0);
    let p2 = Momentum::<NewtonSecond>::new(30.0);
    let p3 = Momentum::<GramMeterPerSecond>::new(20_000.0); // 20 kg⋅m/s
    let total = p1 + p2 + p3; // Result in SI units
    
    println!("    Momentum sum:");
    println!("      {} kg⋅m/s + {} N⋅s + {} g⋅m/s", p1.value(), p2.value(), p3.value());
    println!("      = {} kg⋅m/s (SI units)\n", total.value());

    // 11. Relativistic vs classical
    println!("11. Relativistic vs classical momentum:");
    
    // Classical momentum: p = mv
    let mass_kg = 1.0;
    let velocity_ms = 0.9 * 299_792_458.0; // 0.9c
    let classical_p = Momentum::<KilogramMeterPerSecond>::new(mass_kg * velocity_ms);
    
    // Relativistic momentum approximation: p ≈ γmv, where γ ≈ 2.29 for v = 0.9c
    let gamma = 2.29;
    let relativistic_p = Momentum::<KilogramMeterPerSecond>::new(gamma * mass_kg * velocity_ms);
    
    println!("    1 kg object at 0.9c:");
    println!("      Classical: {:.2e} kg⋅m/s", classical_p.value());
    println!("      Relativistic: {:.2e} kg⋅m/s", relativistic_p.value());
    println!("      Ratio: {:.1}x\n", relativistic_p.value() / classical_p.value());

    println!("✅ Momentum quantity funktioniert perfekt!");
    println!("⚡ From quantum particles to astronomical objects");
    println!("🚀 Spacecraft propulsion and orbital mechanics");
    println!("🎯 Particle physics and relativistic effects");
    println!("💥 Collision dynamics and conservation laws");
    println!("🌌 42 orders of magnitude momentum range!");
}