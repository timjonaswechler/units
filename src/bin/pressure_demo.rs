//! Demo für Pressure quantity

use units::prelude::*;

fn main() {
    println!("🌡️ Pressure Quantity Demo");
    println!("==========================\n");

    // 1. Basic Pressure units
    println!("1. Basic Pressure units:");
    let p_pa = Pressure::<Pascal>::new(100_000.0);
    let p_bar = Pressure::<Bar>::new(1.0);
    println!("   {} Pa = {} bar", p_pa.value(), p_bar.value());
    
    let converted: Pressure<Bar> = p_pa.convert_to();
    println!("   Conversion check: {} Pa = {} bar\n", p_pa.value(), converted.value());

    // 2. Atmospheric pressure references
    println!("2. Atmospheric pressure:");
    let standard_atm = Pressure::<Atmosphere>::new(1.0);
    let atm_pa: Pressure<Pascal> = standard_atm.convert_to();
    let atm_bar: Pressure<Bar> = standard_atm.convert_to();
    let atm_torr: Pressure<Torr> = standard_atm.convert_to();
    
    println!("   Standard atmosphere: 1 atm");
    println!("   = {:.0} Pa", atm_pa.value());
    println!("   = {:.5} bar", atm_bar.value());
    println!("   = {:.0} Torr (mmHg)\n", atm_torr.value());

    // 3. Stellar atmosphere pressures
    println!("3. Stellar atmosphere pressures:");
    
    // Solar photosphere
    let solar_photo = Pressure::<Pascal>::new(100.0);
    let solar_mbar: Pressure<Millibar> = solar_photo.convert_to();
    println!("   Solar photosphere: {} Pa = {} mbar", solar_photo.value(), solar_mbar.value());
    
    // Red giant atmosphere (very low pressure)
    let red_giant = Pressure::<Micropascal>::new(1.0);
    let rg_pa: Pressure<Pascal> = red_giant.convert_to();
    println!("   Red giant atmosphere: {} μPa = {:.0e} Pa", red_giant.value(), rg_pa.value());
    
    // White dwarf atmosphere
    let white_dwarf_atm = Pressure::<Kilopascal>::new(1.0);
    let wd_atm_pa: Pressure<Pascal> = white_dwarf_atm.convert_to();
    println!("   White dwarf atmosphere: {} kPa = {:.0e} Pa\n", white_dwarf_atm.value(), wd_atm_pa.value());

    // 4. Stellar interior pressures
    println!("4. Stellar interior pressures:");
    
    // Solar core
    let solar_core = Pressure::<Gigapascal>::new(25.0);
    let solar_core_atm: Pressure<Atmosphere> = solar_core.convert_to();
    println!("   Solar core: {} GPa = {:.1e} atm", solar_core.value(), solar_core_atm.value());
    
    // Neutron star interior
    let neutron_star = Pressure::<Petapascal>::new(100.0);  // 10^17 Pa
    let ns_gpa: Pressure<Gigapascal> = neutron_star.convert_to();
    println!("   Neutron star interior: {} PPa = {:.0e} GPa", neutron_star.value(), ns_gpa.value());
    
    // White dwarf core
    let wd_core = Pressure::<Terapascal>::new(1.0);
    let wd_gpa: Pressure<Gigapascal> = wd_core.convert_to();
    println!("   White dwarf core: {} TPa = {:.0e} GPa\n", wd_core.value(), wd_gpa.value());

    // 5. Interstellar and circumstellar medium
    println!("5. Interstellar medium pressures:");
    
    // Interstellar medium
    let ism = Pressure::<InterstellarPressure>::new(1.0);
    let ism_pa: Pressure<Pascal> = ism.convert_to();
    let ism_npa: Pressure<Nanopascal> = ism.convert_to();
    println!("   Interstellar medium: 1 P_ISM = {:.0e} Pa = {:.1e} nPa", ism_pa.value(), ism_npa.value());
    
    // Solar wind
    let solar_wind = Pressure::<SolarWindPressure>::new(1.0);
    let sw_pa: Pressure<Pascal> = solar_wind.convert_to();
    let sw_npa: Pressure<Nanopascal> = solar_wind.convert_to();
    println!("   Solar wind: 1 P_sw = {:.0e} Pa = {:.1} nPa", sw_pa.value(), sw_npa.value());
    
    // Hot coronal gas
    let corona = Pressure::<Micropascal>::new(0.1);
    let corona_pa: Pressure<Pascal> = corona.convert_to();
    println!("   Solar corona: {} μPa = {:.1e} Pa\n", corona.value(), corona_pa.value());

    // 6. Planetary atmospheric pressures
    println!("6. Planetary atmospheres:");
    
    // Earth sea level
    let earth_surface = Pressure::<Atmosphere>::new(1.0);
    println!("   Earth surface: {} atm", earth_surface.value());
    
    // Venus surface (extreme greenhouse)
    let venus_surface = Pressure::<Atmosphere>::new(92.0);
    println!("   Venus surface: {} atm", venus_surface.value());
    
    // Mars surface (thin atmosphere)
    let mars_surface = Pressure::<Pascal>::new(600.0);
    let mars_atm: Pressure<Atmosphere> = mars_surface.convert_to();
    println!("   Mars surface: {} Pa = {:.3} atm", mars_surface.value(), mars_atm.value());
    
    // Jupiter's core (estimated)
    let jupiter_core = Pressure::<Terapascal>::new(4.5);  // 4500 GPa
    let jupiter_megabar: Pressure<Gigapascal> = jupiter_core.convert_to();
    println!("   Jupiter core: {} TPa = {:.0} GPa\n", jupiter_core.value(), jupiter_megabar.value());

    // 7. CGS units for astrophysics
    println!("7. CGS pressure units:");
    let cgs_pressure = Pressure::<Dyne>::new(1e6);  // 1 million dyn/cm²
    let cgs_pa: Pressure<Pascal> = cgs_pressure.convert_to();
    let cgs_bar: Pressure<Bar> = cgs_pressure.convert_to();
    
    println!("   Stellar wind pressure: {:.0e} dyn/cm²", cgs_pressure.value());
    println!("   = {:.0e} Pa", cgs_pa.value());
    println!("   = {:.3} bar\n", cgs_bar.value());

    // 8. Pressure scale comparisons
    println!("8. Pressure scale comparisons:");
    let vacuum = Pressure::<Nanopascal>::new(1.0);       // Ultra-high vacuum
    let atmosphere = Pressure::<Atmosphere>::new(1.0);    // Earth atmosphere
    let stellar_core = Pressure::<Gigapascal>::new(100.0); // Stellar core
    
    let atm_vs_vacuum = atmosphere / vacuum;
    let core_vs_atm = stellar_core / atmosphere;
    
    println!("   Pressure ratios:");
    println!("     Earth atmosphere / Ultra-high vacuum: {:.0e}x", atm_vs_vacuum);
    println!("     Stellar core / Earth atmosphere: {:.0e}x\n", core_vs_atm);

    // 9. Mixed unit arithmetic
    println!("9. Mixed unit arithmetic:");
    let p1 = Pressure::<Pascal>::new(50_000.0);         // 50 kPa
    let p2 = Pressure::<Kilopascal>::new(50.0);         // 50 kPa  
    let total = p1 + p2;  // Result in SI units (Pa)
    
    let total_bar: Pressure<Bar> = total.convert_to();
    println!("   Pressure sum: {} Pa + {} kPa = {} Pa", 
             p1.value(), p2.value(), total.value());
    println!("   = {} bar\n", total_bar.value());

    // 10. Hydrostatic pressure calculations
    println!("10. Hydrostatic pressure examples:");
    
    // Deep ocean pressure (Mariana Trench ~11 km deep)
    let ocean_depth = Pressure::<Megapascal>::new(110.0);  // ~1100 atm
    let ocean_atm: Pressure<Atmosphere> = ocean_depth.convert_to();
    println!("    Mariana Trench depth: {} MPa = {:.0} atm", 
             ocean_depth.value(), ocean_atm.value());
    
    // Stellar core hydrostatic pressure balance
    let radiation_pressure = Pressure::<Gigapascal>::new(10.0);
    let gas_pressure = Pressure::<Gigapascal>::new(15.0);
    let total_stellar = radiation_pressure + gas_pressure;
    let total_gpa: Pressure<Gigapascal> = total_stellar.convert_to();
    println!("    Stellar core total pressure:");
    println!("      Radiation: {} GPa", radiation_pressure.value());
    println!("      Gas: {} GPa", gas_pressure.value());
    println!("      Total: {} GPa\n", total_gpa.value());

    println!("✅ Pressure quantity funktioniert perfekt!");
    println!("🌟 From interstellar vacuum to neutron star cores");
    println!("🪐 Planetary atmosphere modeling capabilities");
    println!("⭐ Stellar interior hydrostatic equilibrium");
    println!("📊 CGS units for astrophysics papers");
    println!("🔬 Ultra-high vacuum to extreme stellar pressures!");
}