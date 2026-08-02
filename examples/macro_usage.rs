// Note: We only import astronomy to avoid conflicts with prelude's Kilometer
use units::quantities::astronomy::*;
use units::value::Value;

fn main() {
    println!("=== Macro System - Easy Unit Definition ===\n");

    // ============================================================================
    // Example 1: Using astronomy module (already defined with macros)
    // ============================================================================

    println!("1. Astronomical Distances:");
    let earth_sun = Value::<Distance, AstronomicalUnit>::new(1.0);
    let earth_sun_m = earth_sun.convert::<Meter>();
    println!("   1 AU = {:.3e} m", earth_sun_m.get());

    let alpha_centauri = Value::<Distance, LightYear>::new(4.37);
    let alpha_centauri_parsec = alpha_centauri.convert::<Parsec>();
    println!(
        "   Alpha Centauri: {} ly = {} pc",
        alpha_centauri.get(),
        alpha_centauri_parsec.get()
    );

    // ============================================================================
    // Example 2: Astronomical Masses
    // ============================================================================

    println!("\n2. Astronomical Masses:");
    let sun_mass = Value::<AstroMass, SolarMass>::new(1.0);
    let sun_mass_earth = sun_mass.convert::<EarthMass>();
    println!("   1 Solar Mass = {} Earth Masses", sun_mass_earth.get());

    // ============================================================================
    // Example 3: Angles
    // ============================================================================

    println!("\n3. Angles:");
    let right_angle = Value::<Angle, Degree>::new(90.0);
    let right_angle_rad = right_angle.convert::<Radian>();
    println!("   90° = {} rad", right_angle_rad.get());

    let parallax = Value::<Angle, Arcsecond>::new(0.1);
    let parallax_rad = parallax.convert::<Radian>();
    println!("   0.1\" = {} rad", parallax_rad.get());

    // ============================================================================
    // Example 4: Velocity (including speed of light!)
    // ============================================================================

    println!("\n4. Velocities:");
    let c = Value::<Velocity, SpeedOfLight>::new(1.0);
    let c_mps = c.convert::<MeterPerSecond>();
    println!("   Speed of light = {} m/s", c_mps.get());

    let escape_velocity = Value::<Velocity, KilometerPerSecond>::new(11.2);
    let escape_mps = escape_velocity.convert::<MeterPerSecond>();
    println!(
        "   Earth escape velocity = {} km/s = {} m/s",
        escape_velocity.get(),
        escape_mps.get()
    );

    // ============================================================================
    // Example 5: Acceleration
    // ============================================================================

    println!("\n5. Acceleration:");
    let g = Value::<Acceleration, StandardGravity>::new(1.0);
    let g_mps2 = g.convert::<MeterPerSecondSquared>();
    println!("   1 g = {} m/s²", g_mps2.get());

    // ============================================================================
    // Example 6: Luminosity
    // ============================================================================

    println!("\n6. Luminosity:");
    let sun_lum = Value::<Luminosity, SolarLuminosity>::new(1.0);
    let sun_watts = sun_lum.convert::<Watt>();
    println!("   Solar Luminosity = {:.3e} W", sun_watts.get());

    // ============================================================================
    // Example 7: Pressure
    // ============================================================================

    println!("\n7. Pressure:");
    let atm = Value::<Pressure, Atmosphere>::new(1.0);
    let pascal = atm.convert::<Pascal>();
    let bar = atm.convert::<Bar>();
    println!("   1 atm = {} Pa = {} bar", pascal.get(), bar.get());

    // ============================================================================
    // Example 8: Type Safety
    // ============================================================================

    println!("\n8. Type Safety:");
    println!("   ✓ Can convert Distance to different length units");
    println!("   ✓ Can convert Angle to different angle units");
    println!("   ✗ Cannot convert Distance to Angle (different dimensions)");

    // This would be a compile error:
    // let invalid = earth_sun.convert::<Degree>();

    println!("\n=== All macro-defined units work perfectly! ===");
}
