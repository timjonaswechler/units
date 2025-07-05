//! Demo für Luminosity quantity

use units::prelude::*;

fn main() {
    println!("🌟 Luminosity Quantity Demo");
    println!("===========================\n");

    // 1. Basic Luminosity units
    println!("1. Basic Luminosity units:");
    let power_w = Luminosity::<Watt>::new(1000.0);
    let power_kw = Luminosity::<Kilowatt>::new(1.0);
    println!("   {} W = {} kW", power_w.value(), power_kw.value());
    
    let converted: Luminosity<Kilowatt> = power_w.convert_to();
    println!("   Conversion check: {} W = {} kW\n", power_w.value(), converted.value());

    // 2. Solar luminosity reference
    println!("2. Solar luminosity reference:");
    let sun = Luminosity::<SolarLuminosity>::new(1.0);
    let sun_watts: Luminosity<Watt> = sun.convert_to();
    let sun_gw: Luminosity<Gigawatt> = sun.convert_to();
    
    println!("   Sun luminosity: 1 L☉");
    println!("   = {:.3e} W", sun_watts.value());
    println!("   = {:.1e} GW\n", sun_gw.value());

    // 3. Stellar classification by luminosity
    println!("3. Stellar classification:");
    
    // M-dwarf (red dwarf)
    let red_dwarf = Luminosity::<SolarLuminosity>::new(0.0001);
    println!("   M-dwarf (red dwarf): {} L☉", red_dwarf.value());
    
    // K-dwarf 
    let k_dwarf = Luminosity::<SolarLuminosity>::new(0.1);
    println!("   K-dwarf: {} L☉", k_dwarf.value());
    
    // G-dwarf (Sun-like)
    let g_dwarf = Luminosity::<SolarLuminosity>::new(1.0);
    println!("   G-dwarf (Sun): {} L☉", g_dwarf.value());
    
    // F-star
    let f_star = Luminosity::<SolarLuminosity>::new(3.0);
    println!("   F-star: {} L☉", f_star.value());
    
    // A-star
    let a_star = Luminosity::<SolarLuminosity>::new(25.0);
    println!("   A-star (like Sirius): {} L☉", a_star.value());
    
    // B-star
    let b_star = Luminosity::<SolarLuminosity>::new(10_000.0);
    println!("   B-star (blue giant): {} L☉", b_star.value());
    
    println!();

    // 4. Extreme stellar objects
    println!("4. Extreme stellar objects:");
    
    // White dwarf
    let white_dwarf = Luminosity::<SolarLuminosity>::new(0.001);
    let wd_watts: Luminosity<Watt> = white_dwarf.convert_to();
    println!("   White dwarf: {} L☉ = {:.2e} W", white_dwarf.value(), wd_watts.value());
    
    // Red supergiant (Betelgeuse)
    let betelgeuse = Luminosity::<SolarLuminosity>::new(100_000.0);
    let betelgeuse_tw: Luminosity<TeraWatt> = betelgeuse.convert_to();
    println!("   Red supergiant (Betelgeuse): {} L☉ = {:.1e} TW", 
             betelgeuse.value(), betelgeuse_tw.value());
    
    // Hypergiant
    let hypergiant = Luminosity::<SolarLuminosity>::new(1_000_000.0);
    println!("   Hypergiant: {} L☉", hypergiant.value());
    
    println!();

    // 5. Variable stars
    println!("5. Variable star brightness:");
    
    // Cepheid variable (δ Cephei)
    let cepheid_min = Luminosity::<SolarLuminosity>::new(2000.0);
    let cepheid_max = Luminosity::<SolarLuminosity>::new(20_000.0);
    let variation_ratio = cepheid_max / cepheid_min;
    
    println!("   Cepheid variable:");
    println!("     Minimum: {} L☉", cepheid_min.value());
    println!("     Maximum: {} L☉", cepheid_max.value());
    println!("     Variation: {}x brighter at maximum\n", variation_ratio);

    // 6. CGS units for astrophysics
    println!("6. CGS units (astrophysics papers):");
    let stellar_lum_cgs = Luminosity::<ErgPerSecond>::new(1e33);
    let stellar_lum_w: Luminosity<Watt> = stellar_lum_cgs.convert_to();
    let stellar_lum_solar: Luminosity<SolarLuminosity> = stellar_lum_w.convert_to();
    
    println!("   Typical star: {:.1e} erg/s", stellar_lum_cgs.value());
    println!("   = {:.1e} W", stellar_lum_w.value());
    println!("   = {:.2} L☉\n", stellar_lum_solar.value());

    // 7. Stellar evolution stages
    println!("7. Stellar evolution:");
    
    // Main sequence lifetime luminosity
    let ms_start = Luminosity::<SolarLuminosity>::new(1.0);
    let ms_end = Luminosity::<SolarLuminosity>::new(2.0);  // Sun brightens over time
    
    println!("   Main sequence evolution:");
    println!("     Early MS: {} L☉", ms_start.value());
    println!("     Late MS: {} L☉", ms_end.value());
    
    // Red giant phase
    let red_giant = Luminosity::<SolarLuminosity>::new(1000.0);
    println!("     Red giant: {} L☉", red_giant.value());
    
    // Final white dwarf
    let final_wd = Luminosity::<SolarLuminosity>::new(0.0001);
    println!("     Final white dwarf: {} L☉\n", final_wd.value());

    // 8. Mixed unit arithmetic
    println!("8. Mixed unit arithmetic:");
    let binary_primary = Luminosity::<SolarLuminosity>::new(2.5);
    let binary_secondary = Luminosity::<Watt>::new(1e26);  // 0.26 L☉
    let total_system = binary_primary + binary_secondary;  // Result in SI (W)
    
    let total_solar: Luminosity<SolarLuminosity> = total_system.convert_to();
    println!("   Binary system total:");
    println!("     Primary: {} L☉", binary_primary.value());
    println!("     Secondary: {:.1e} W", binary_secondary.value());
    println!("     Total: {:.2} L☉\n", total_solar.value());

    // 9. Distance and apparent brightness relationships
    println!("9. Brightness comparisons:");
    
    // Compare stars of different luminosities
    let proxima = Luminosity::<SolarLuminosity>::new(0.0017);  // Proxima Centauri
    let sirius = Luminosity::<SolarLuminosity>::new(25.0);     // Sirius A
    let rigel = Luminosity::<SolarLuminosity>::new(120_000.0); // Rigel
    
    println!("   Intrinsic luminosities:");
    println!("     Proxima Centauri: {} L☉", proxima.value());
    println!("     Sirius A: {} L☉", sirius.value());
    println!("     Rigel: {} L☉", rigel.value());
    
    let sirius_vs_proxima = sirius / proxima;
    let rigel_vs_sun = rigel / g_dwarf;
    
    println!("   Luminosity ratios:");
    println!("     Sirius/Proxima: {:.0}x", sirius_vs_proxima);
    println!("     Rigel/Sun: {:.0}x\n", rigel_vs_sun);

    println!("✅ Luminosity quantity funktioniert perfekt!");
    println!("🌟 Stellar classification from M-dwarfs to hypergiants");
    println!("🌟 Variable star brightness variations");
    println!("📊 CGS units for astrophysics papers");
    println!("🔄 Stellar evolution luminosity changes");
    println!("⭐ Binary system luminosity combinations!");
}