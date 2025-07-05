//! Test current display functionality

use units::prelude::*;

fn main() {
    println!("🖨️ Current Display Functionality Test");
    println!("====================================\n");

    // Test basic unit display
    println!("1. Basic unit display:");
    let distance = Distance::<Meter>::new(42.5);
    let time = Time::<Second>::new(10.0);
    let mass = Mass::<Kilogram>::new(75.0);
    
    println!("   Distance: {}", distance);
    println!("   Time: {}", time);
    println!("   Mass: {}\n", mass);

    // Test composed units
    println!("2. Composed unit display:");
    let velocity = Velocity::<MeterPerSecond>::new(15.0);
    let force = Force::<Newton>::new(100.0);
    let energy = Energy::<Joule>::new(500.0);
    
    println!("   Velocity: {}", velocity);
    println!("   Force: {}", force);
    println!("   Energy: {}\n", energy);

    // Test astronomical units
    println!("3. Astronomical unit display:");
    let distance_au = Distance::<AstronomicalUnit>::new(5.2);
    let mass_solar = Mass::<SolarMass>::new(2.5);
    let luminosity = Power::<SolarLuminosity>::new(10_000.0);
    
    println!("   Distance: {}", distance_au);
    println!("   Mass: {}", mass_solar);
    println!("   Luminosity: {}\n", luminosity);

    // Test tuple syntax display
    println!("4. Tuple syntax display:");
    let velocity_tuple = Velocity::<(Meter, Second)>::new(25.0);
    let acceleration_tuple = Acceleration::<(Meter, Second, Second)>::new(9.81);
    
    println!("   Velocity (tuple): {}", velocity_tuple);
    println!("   Acceleration (tuple): {}\n", acceleration_tuple);

    // Test large and small numbers
    println!("5. Large and small number display:");
    let planck_length = Distance::<PlanckLength>::new(1.0);
    let light_year = Distance::<LightYear>::new(1.0);
    let electron_mass = Mass::<ElectronMass>::new(1.0);
    let solar_mass = Mass::<SolarMass>::new(1.0);
    
    println!("   Planck length: {}", planck_length);
    println!("   Light year: {}", light_year);
    println!("   Electron mass: {}", electron_mass);
    println!("   Solar mass: {}\n", solar_mass);

    // Test scientific notation values
    println!("6. Scientific notation values:");
    let big_distance = Distance::<Meter>::new(1.5e12);
    let tiny_time = Time::<Second>::new(1.2e-9);
    let huge_energy = Energy::<Joule>::new(4.18e26);
    
    println!("   Big distance: {}", big_distance);
    println!("   Tiny time: {}", tiny_time);
    println!("   Huge energy: {}\n", huge_energy);

    println!("Current display shows: value + unit symbol");
    println!("Areas for improvement:");
    println!("- No automatic scientific notation for extreme values");
    println!("- No precision control");
    println!("- No unit-aware formatting");
    println!("- No alternative display modes");
}