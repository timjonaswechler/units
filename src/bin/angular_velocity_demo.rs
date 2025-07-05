//! Demo für Angular Velocity quantity

use units::prelude::*;

fn main() {
    println!("🌀 Angular Velocity Quantity Demo");
    println!("=================================\n");

    // 1. Basic Angular Velocity units
    println!("1. Basic Angular Velocity units:");
    let omega_rad = AngularVelocity::<RadianPerSecond>::new(1.0);
    let omega_deg = AngularVelocity::<DegreePerSecond>::new(57.2958);  // ~180/π
    println!("   {} rad/s ≈ {} deg/s", omega_rad.value(), omega_deg.value());
    
    let converted: AngularVelocity<DegreePerSecond> = omega_rad.convert_to();
    println!("   Conversion check: {} rad/s = {:.1} deg/s\n", omega_rad.value(), converted.value());

    // 2. Rotational units
    println!("2. Rotational frequency units:");
    let rotation_rpm = AngularVelocity::<RotationsPerMinute>::new(60.0);
    let rotation_rad: AngularVelocity<RadianPerSecond> = rotation_rpm.convert_to();
    let rotation_rps: AngularVelocity<RotationsPerSecond> = rotation_rpm.convert_to();
    
    println!("   60 rpm = {:.3} rad/s = {} rps", rotation_rad.value(), rotation_rps.value());
    println!("   (60 rpm = 1 revolution per second = 2π rad/s)\n");

    // 3. Earth's rotation and orbital motion
    println!("3. Earth's rotational dynamics:");
    
    // Earth's daily rotation
    let earth_rotation = AngularVelocity::<EarthRotationRate>::new(1.0);
    let earth_deg_s: AngularVelocity<DegreePerSecond> = earth_rotation.convert_to();
    let earth_deg_h: AngularVelocity<DegreePerHour> = earth_rotation.convert_to();
    
    println!("   Earth rotation: 1 Ω_⊕ = {:.6} deg/s = {:.1} deg/h", 
             earth_deg_s.value(), earth_deg_h.value());
    
    // Earth's orbital motion
    let earth_orbit = AngularVelocity::<EarthOrbitalRate>::new(1.0);
    let earth_orbit_deg: AngularVelocity<DegreePerSecond> = earth_orbit.convert_to();
    
    println!("   Earth orbital motion: 1 ω_⊕ = {:.8} deg/s", earth_orbit_deg.value());
    
    // Comparison
    let rotation_vs_orbit = earth_rotation / earth_orbit;
    println!("   Earth rotates {:.0}x faster than it orbits\n", rotation_vs_orbit);

    // 4. Solar rotation
    println!("4. Solar rotation:");
    let solar_rotation = AngularVelocity::<SolarRotationRate>::new(1.0);
    let solar_rpm: AngularVelocity<RotationsPerMinute> = solar_rotation.convert_to();
    let solar_deg_day: AngularVelocity<DegreePerHour> = solar_rotation.convert_to();
    
    println!("   Solar rotation: 1 Ω_☉");
    println!("   = {:.2e} rpm", solar_rpm.value());
    println!("   = {:.4} deg/h", solar_deg_day.value());
    
    // Solar rotation period (approximate)
    let solar_period_days = 360.0 / (solar_deg_day.value() * 24.0);
    println!("   Solar rotation period: ~{:.0} days\n", solar_period_days);

    // 5. Pulsar rotation (extreme case)
    println!("5. Pulsar rotation (extreme rotation):");
    let pulsar = AngularVelocity::<PulsarRotationRate>::new(1.0);
    let pulsar_rpm: AngularVelocity<RotationsPerMinute> = pulsar.convert_to();
    let pulsar_rps: AngularVelocity<RotationsPerSecond> = pulsar.convert_to();
    
    println!("   Typical pulsar: 1 Ω_PSR = {} rad/s", pulsar.value());
    println!("   = {:.0} rpm", pulsar_rpm.value());
    println!("   = {:.0} rps", pulsar_rps.value());
    
    // Comparison with Earth and Sun
    let pulsar_vs_earth = pulsar / earth_rotation;
    let pulsar_vs_sun = pulsar / solar_rotation;
    
    println!("   Pulsar vs Earth rotation: {:.0e}x faster", pulsar_vs_earth);
    println!("   Pulsar vs Solar rotation: {:.0e}x faster\n", pulsar_vs_sun);

    // 6. Engineering applications
    println!("6. Engineering rotation rates:");
    
    // Car engine at highway speed
    let car_engine = AngularVelocity::<RPM>::new(3000.0);
    let engine_rad: AngularVelocity<RadianPerSecond> = car_engine.convert_to();
    println!("   Car engine: {} RPM = {:.0} rad/s", car_engine.value(), engine_rad.value());
    
    // Jet engine
    let jet_engine = AngularVelocity::<RPM>::new(50_000.0);
    let jet_rad: AngularVelocity<RadianPerSecond> = jet_engine.convert_to();
    println!("   Jet engine: {} RPM = {:.0} rad/s", jet_engine.value(), jet_rad.value());
    
    // Hard disk drive
    let hdd = AngularVelocity::<RPM>::new(7200.0);
    let hdd_rad: AngularVelocity<RadianPerSecond> = hdd.convert_to();
    println!("   Hard disk: {} RPM = {:.0} rad/s\n", hdd.value(), hdd_rad.value());

    // 7. Astronomical proper motion
    println!("7. Astronomical proper motion:");
    
    // Barnard's star (high proper motion)
    let barnards_star = AngularVelocity::<MilliarcsecondsPerYear>::new(10_000.0);  // ~10 arcsec/yr
    let barnards_rad: AngularVelocity<RadianPerSecond> = barnards_star.convert_to();
    
    println!("   Barnard's star: {} mas/yr", barnards_star.value());
    println!("   = {:.2e} rad/s", barnards_rad.value());
    
    // Ultra-precise measurement with Gaia
    let gaia_precision = AngularVelocity::<MicroarcsecondsPerYear>::new(10.0);
    let gaia_mas: AngularVelocity<MilliarcsecondsPerYear> = gaia_precision.convert_to();
    
    println!("   Gaia precision: {} μas/yr = {} mas/yr\n", gaia_precision.value(), gaia_mas.value());

    // 8. Planetary rotation comparison
    println!("8. Planetary rotation rates:");
    
    // Jupiter (fast rotator)
    let jupiter_rotation = AngularVelocity::<DegreePerHour>::new(36.0);  // ~10 hour period
    let jupiter_earth_ratio = jupiter_rotation / earth_deg_h;
    println!("   Jupiter: {} deg/h ({:.1}x faster than Earth)", 
             jupiter_rotation.value(), jupiter_earth_ratio);
    
    // Venus (retrograde slow rotation)
    let venus_rotation = AngularVelocity::<DegreePerHour>::new(-0.006);  // ~243 Earth days
    let venus_earth_ratio = venus_rotation.value() / earth_deg_h.value();
    println!("   Venus: {:.3} deg/h ({:.0}x slower than Earth, retrograde)", 
             venus_rotation.value(), venus_earth_ratio.abs());
    
    // Mars (similar to Earth)
    let mars_rotation = AngularVelocity::<DegreePerHour>::new(14.6);  // ~24.6 hour period
    let mars_earth_ratio = mars_rotation / earth_deg_h;
    println!("   Mars: {} deg/h ({:.2}x Earth rate)\n", mars_rotation.value(), mars_earth_ratio);

    // 9. Mixed unit arithmetic
    println!("9. Mixed unit arithmetic:");
    let omega1 = AngularVelocity::<RadianPerSecond>::new(1.0);
    let omega2 = AngularVelocity::<DegreePerSecond>::new(57.3);  // ~1 rad/s in deg/s
    let total = omega1 + omega2;  // Result in SI units (rad/s)
    
    let total_deg: AngularVelocity<DegreePerSecond> = total.convert_to();
    println!("   Angular velocity sum: {} rad/s + {} deg/s = {} rad/s", 
             omega1.value(), omega2.value(), total.value());
    println!("   = {:.1} deg/s\n", total_deg.value());

    // 10. Rotational dynamics applications
    println!("10. Rotational dynamics scale:");
    
    // Create angular velocity hierarchy
    let molecular_vibration = AngularVelocity::<RadianPerSecond>::new(1e12);    // THz molecular motion
    let turbine_blade = jet_engine;                                             // Jet engine
    let planetary_rotation = earth_rotation;                                    // Earth
    let stellar_rotation = solar_rotation;                                     // Sun
    let galactic_rotation = AngularVelocity::<RadianPerSecond>::new(1e-15);   // Galaxy rotation
    
    println!("   Angular velocity hierarchy (rad/s):");
    println!("     Molecular vibration: {:.0e}", molecular_vibration.value());
    println!("     Jet engine: {:.0e}", jet_rad.value());
    println!("     Earth rotation: {:.1e}", earth_rotation.value());
    println!("     Solar rotation: {:.1e}", solar_rotation.value());
    println!("     Galactic rotation: {:.0e}", galactic_rotation.value());
    
    let extreme_range = molecular_vibration / galactic_rotation;
    println!("   Total range: {:.0e} orders of magnitude\n", extreme_range);

    // 11. Period calculations
    println!("11. Rotation periods:");
    
    // Calculate periods from angular velocities
    let earth_period_s = 2.0 * std::f64::consts::PI / earth_rotation.value();
    let pulsar_period_ms = (2.0 * std::f64::consts::PI / pulsar.value()) * 1000.0;
    let solar_period_days = (2.0 * std::f64::consts::PI / solar_rotation.value()) / (24.0 * 3600.0);
    
    println!("   Earth rotation period: {:.0} hours", earth_period_s / 3600.0);
    println!("   Pulsar rotation period: {:.1} ms", pulsar_period_ms);
    println!("   Solar rotation period: {:.0} days\n", solar_period_days);

    println!("✅ Angular Velocity quantity funktioniert perfekt!");
    println!("🌍 From Earth rotation to galactic dynamics");
    println!("🌟 Stellar rotation and pulsar timing");
    println!("⭐ Astronomical proper motion measurements");
    println!("🔧 Engineering applications from engines to disks");
    println!("🌌 27 orders of magnitude angular velocity range!");
}