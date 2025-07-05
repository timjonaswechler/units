//! Demo für Angle quantity

use units::prelude::*;

fn main() {
    println!("📐 Angle Quantity Demo");
    println!("=====================\n");

    // 1. Basic Angle units
    println!("1. Basic Angle units:");
    let angle_rad = Angle::<Radian>::new(std::f64::consts::PI);
    let angle_deg = Angle::<Degree>::new(180.0);
    println!("   {} rad = {} deg", angle_rad.value(), angle_deg.value());
    
    let converted: Angle<Degree> = angle_rad.convert_to();
    println!("   Conversion check: {} rad = {:.1} deg\n", angle_rad.value(), converted.value());

    // 2. Orbital mechanics angles
    println!("2. Orbital mechanics angles:");
    let earth_inclination = Angle::<Degree>::new(0.0);  // Earth's orbital inclination
    let pluto_inclination = Angle::<Degree>::new(17.16); // Pluto's steep inclination
    println!("   Earth inclination: {} deg", earth_inclination.value());
    println!("   Pluto inclination: {} deg", pluto_inclination.value());
    
    let pluto_rad: Angle<Radian> = pluto_inclination.convert_to();
    println!("   Pluto in radians: {:.4} rad\n", pluto_rad.value());

    // 3. Stellar parallax measurements (milliarcseconds)
    println!("3. Stellar parallax measurements:");
    let proxima_parallax = Angle::<Milliarcsecond>::new(768.5); // Proxima Centauri
    let sirius_parallax = Angle::<Milliarcsecond>::new(379.21); // Sirius
    println!("   Proxima Centauri parallax: {} mas", proxima_parallax.value());
    println!("   Sirius parallax: {} mas", sirius_parallax.value());
    
    let proxima_arcsec: Angle<Arcsecond> = proxima_parallax.convert_to();
    let sirius_deg: Angle<Degree> = sirius_parallax.convert_to();
    println!("   Proxima: {:.4} arcsec", proxima_arcsec.value());
    println!("   Sirius: {:.2e} deg\n", sirius_deg.value());

    // 4. High precision measurements (microarcseconds)
    println!("4. Ultra-precise astrometry:");
    let stellar_diameter = Angle::<Microarcsecond>::new(500.0); // Angular diameter
    let stellar_mas: Angle<Milliarcsecond> = stellar_diameter.convert_to();
    println!("   Stellar angular diameter: {} μas = {:.3} mas\n", 
             stellar_diameter.value(), stellar_mas.value());

    // 5. Full rotation angles
    println!("5. Rotational angles:");
    let full_rotation = Angle::<Revolution>::new(1.0);
    let full_deg: Angle<Degree> = full_rotation.convert_to();
    let full_rad: Angle<Radian> = full_rotation.convert_to();
    println!("   1 revolution = {} deg = {:.3} rad", full_deg.value(), full_rad.value());
    
    let quarter_turn = Angle::<Degree>::new(90.0);
    let quarter_grad: Angle<Gradian> = quarter_turn.convert_to();
    println!("   90 deg = {} grad\n", quarter_grad.value());

    // 6. Precise degree subdivisions
    println!("6. Degree subdivisions:");
    let one_degree = Angle::<Degree>::new(1.0);
    let arcminutes: Angle<Arcminute> = one_degree.convert_to();
    let arcseconds: Angle<Arcsecond> = one_degree.convert_to();
    println!("   1 deg = {} arcmin = {} arcsec", arcminutes.value(), arcseconds.value());
    
    let one_arcmin = Angle::<Arcminute>::new(1.0);
    let arcmin_to_arcsec: Angle<Arcsecond> = one_arcmin.convert_to();
    println!("   1 arcmin = {} arcsec\n", arcmin_to_arcsec.value());

    // 7. Mixed unit arithmetic
    println!("7. Mixed unit arithmetic:");
    let angle1 = Angle::<Degree>::new(45.0);        // 45 degrees
    let angle2 = Angle::<Radian>::new(std::f64::consts::PI / 4.0); // π/4 radians = 45 degrees
    let sum = angle1 + angle2;  // Result in SI units (radians)
    println!("   {} deg + {} rad = {:.4} rad (SI)", 
             angle1.value(), angle2.value(), sum.value());
    
    let sum_deg: Angle<Degree> = sum.convert_to();
    println!("   = {} deg\n", sum_deg.value());

    // 8. Dimensionless ratios
    println!("8. Angular ratios:");
    let big_angle = Angle::<Degree>::new(180.0);
    let small_angle = Angle::<Degree>::new(60.0);
    let ratio = big_angle / small_angle;
    println!("   {} deg ÷ {} deg = {} (dimensionless)", 
             big_angle.value(), small_angle.value(), ratio);

    // 9. Realistic astronomy calculations
    println!("9. Astronomy calculations:");
    let moon_angular_diameter = Angle::<Arcminute>::new(31.0); // ~31 arcmin
    let sun_angular_diameter = Angle::<Arcminute>::new(32.0);  // ~32 arcmin
    println!("   Moon angular diameter: {} arcmin", moon_angular_diameter.value());
    println!("   Sun angular diameter: {} arcmin", sun_angular_diameter.value());
    
    let size_ratio = sun_angular_diameter / moon_angular_diameter;
    println!("   Sun/Moon size ratio: {:.2}\n", size_ratio);

    println!("✅ Angle quantity funktioniert perfekt!");
    println!("🎯 Parallax measurements from μas to degrees");
    println!("🎯 Orbital mechanics angles and rotations");
    println!("📏 Mixed units, precise conversions, and astronomical applications!");
}