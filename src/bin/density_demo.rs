//! Demo für Density quantity

use units::prelude::*;

fn main() {
    println!("🪨 Density Quantity Demo");
    println!("========================\n");

    // 1. Basic Density units
    println!("1. Basic Density units:");
    let d_si = Density::<KilogramPerCubicMeter>::new(1000.0);
    let d_cgs = Density::<GramPerCubicCentimeter>::new(1.0);
    println!("   {} kg/m³ = {} g/cm³", d_si.value(), d_cgs.value());
    
    let converted: Density<GramPerCubicCentimeter> = d_si.convert_to();
    println!("   Conversion check: {} kg/m³ = {} g/cm³\n", d_si.value(), converted.value());

    // 2. Water density reference
    println!("2. Water density reference:");
    let water = Density::<WaterDensity>::new(1.0);
    let water_si: Density<KilogramPerCubicMeter> = water.convert_to();
    let water_cgs: Density<GramPerCubicCentimeter> = water.convert_to();
    
    println!("   Water density: 1 ρ_H₂O");
    println!("   = {} kg/m³", water_si.value());
    println!("   = {} g/cm³\n", water_cgs.value());

    // 3. Stellar densities
    println!("3. Stellar object densities:");
    
    // Solar density
    let sun = Density::<SolarDensity>::new(1.0);
    let sun_cgs: Density<GramPerCubicCentimeter> = sun.convert_to();
    println!("   Sun (mean): {} ρ☉ = {:.3} g/cm³", sun.value(), sun_cgs.value());
    
    // Earth density (for comparison)
    let earth = Density::<EarthDensity>::new(1.0);
    let earth_cgs: Density<GramPerCubicCentimeter> = earth.convert_to();
    println!("   Earth (mean): {} ρ_⊕ = {:.1} g/cm³", earth.value(), earth_cgs.value());
    
    // Density comparison
    let earth_sun_ratio = earth / sun;
    println!("   Earth is {:.1}x denser than Sun\n", earth_sun_ratio);

    // 4. Extreme stellar object densities
    println!("4. Extreme stellar densities:");
    
    // White dwarf
    let white_dwarf = Density::<WhiteDwarfDensity>::new(1.0);
    let wd_cgs: Density<GramPerCubicCentimeter> = white_dwarf.convert_to();
    println!("   White dwarf: 1 ρ_WD = {:.0e} g/cm³", wd_cgs.value());
    
    // Neutron star
    let neutron_star = Density::<NeutronStarDensity>::new(1.0);
    let ns_cgs: Density<GramPerCubicCentimeter> = neutron_star.convert_to();
    println!("   Neutron star: 1 ρ_NS = {:.1e} g/cm³", ns_cgs.value());
    
    // Nuclear density
    let nuclear = Density::<NuclearDensity>::new(1.0);
    let nuc_cgs: Density<GramPerCubicCentimeter> = nuclear.convert_to();
    println!("   Nuclear density: 1 ρ_nuc = {:.1e} g/cm³", nuc_cgs.value());
    
    // Density ratios
    let wd_water_ratio = white_dwarf / water;
    let ns_nuclear_ratio = neutron_star / nuclear;
    println!("   White dwarf vs water: {:.0e}x denser", wd_water_ratio);
    println!("   Neutron star vs nuclear: {:.1}x denser\n", ns_nuclear_ratio);

    // 5. Interstellar and molecular cloud densities
    println!("5. Interstellar medium densities:");
    
    // Interstellar medium
    let ism = Density::<InterstellarDensity>::new(1.0);
    let ism_cgs: Density<GramPerCubicCentimeter> = ism.convert_to();
    let ism_si: Density<KilogramPerCubicMeter> = ism.convert_to();
    println!("   Interstellar medium: 1 ρ_ISM");
    println!("     = {:.0e} g/cm³", ism_cgs.value());
    println!("     = {:.0e} kg/m³", ism_si.value());
    
    // Molecular cloud
    let mol_cloud = Density::<MolecularCloudDensity>::new(1.0);
    let mc_cgs: Density<GramPerCubicCentimeter> = mol_cloud.convert_to();
    println!("   Molecular cloud: 1 ρ_MC = {:.0e} g/cm³", mc_cgs.value());
    
    // Density enhancement
    let mc_ism_ratio = mol_cloud / ism;
    println!("   Molecular cloud is {}x denser than ISM\n", mc_ism_ratio);

    // 6. Atmospheric densities
    println!("6. Atmospheric densities:");
    
    // Air at sea level
    let air = Density::<AirDensity>::new(1.0);
    let air_si: Density<KilogramPerCubicMeter> = air.convert_to();
    let air_cgs: Density<GramPerCubicCentimeter> = air.convert_to();
    println!("   Air (sea level): {} ρ_air = {} kg/m³ = {:.6} g/cm³", 
             air.value(), air_si.value(), air_cgs.value());
    
    // Compare to water
    let water_air_ratio = water / air;
    println!("   Water is {:.0}x denser than air\n", water_air_ratio);

    // 7. Material density examples
    println!("7. Common material densities:");
    
    // Various materials in g/cm³
    let aluminum = Density::<GramPerCubicCentimeter>::new(2.7);
    let iron = Density::<GramPerCubicCentimeter>::new(7.87);
    let lead = Density::<GramPerCubicCentimeter>::new(11.34);
    let gold = Density::<GramPerCubicCentimeter>::new(19.32);
    
    println!("   Aluminum: {} g/cm³", aluminum.value());
    println!("   Iron: {} g/cm³", iron.value());
    println!("   Lead: {} g/cm³", lead.value());
    println!("   Gold: {} g/cm³", gold.value());
    
    // Metal ratios
    let gold_aluminum_ratio = gold / aluminum;
    println!("   Gold is {:.1}x denser than aluminum\n", gold_aluminum_ratio);

    // 8. Scale comparison across the universe
    println!("8. Density scale across the universe:");
    
    // Create density scale
    let vacuum = ism;                           // ISM density (~10⁻²¹ g/cm³)
    let gas_giant = Density::<GramPerCubicCentimeter>::new(1.3);  // Jupiter-like
    let rocky_planet = earth;                   // Earth density
    let stellar_core = Density::<GramPerCubicCentimeter>::new(100.0);  // Solar core
    let white_dwarf_core = white_dwarf;         // White dwarf
    let neutron_star_core = neutron_star;       // Neutron star
    
    println!("   Density hierarchy (g/cm³):");
    println!("     ISM vacuum: {:.0e}", ism_cgs.value());
    println!("     Gas giant: {}", gas_giant.value());
    println!("     Rocky planet: {:.1}", earth_cgs.value());
    println!("     Stellar core: {}", stellar_core.value());
    println!("     White dwarf: {:.0e}", wd_cgs.value());
    println!("     Neutron star: {:.0e}", ns_cgs.value());
    
    let extreme_range = neutron_star_core / vacuum;
    println!("   Total range: {:.0e} orders of magnitude\n", extreme_range);

    // 9. Mixed unit arithmetic
    println!("9. Mixed unit arithmetic:");
    let d1 = Density::<KilogramPerCubicMeter>::new(500.0);        // 500 kg/m³
    let d2 = Density::<GramPerCubicCentimeter>::new(0.5);         // 0.5 g/cm³ = 500 kg/m³
    let total = d1 + d2;  // Result in SI units (kg/m³)
    
    let total_cgs: Density<GramPerCubicCentimeter> = total.convert_to();
    println!("   Density sum: {} kg/m³ + {} g/cm³ = {} kg/m³", 
             d1.value(), d2.value(), total.value());
    println!("   = {} g/cm³\n", total_cgs.value());

    // 10. Astrophysical applications
    println!("10. Astrophysical density applications:");
    
    // Star formation density threshold
    let jeans_density = Density::<GramPerCubicCentimeter>::new(1e-19);  // Jeans instability
    let jeans_mc_ratio = mol_cloud / jeans_density;
    println!("    Molecular cloud vs Jeans density: {:.0}x", jeans_mc_ratio);
    
    // Stellar evolution density jump
    let main_sequence = Density::<GramPerCubicCentimeter>::new(1.4);      // Solar center
    let red_giant_core = Density::<GramPerCubicCentimeter>::new(1000.0);  // He flash
    let evolution_jump = red_giant_core / main_sequence;
    println!("    Red giant core evolution: {:.0}x density increase", evolution_jump);
    
    // Planetary differentiation
    let silicate_mantle = Density::<GramPerCubicCentimeter>::new(3.3);    // Olivine
    let iron_core = Density::<GramPerCubicCentimeter>::new(7.9);          // Iron
    let differentiation = iron_core / silicate_mantle;
    println!("    Core/mantle differentiation: {:.1}x density contrast\n", differentiation);

    println!("✅ Density quantity funktioniert perfekt!");
    println!("🌌 From interstellar vacuum to neutron star cores");
    println!("🪐 Planetary and stellar structure modeling");
    println!("⭐ Stellar evolution density tracking");
    println!("📊 CGS units for astrophysics papers");
    println!("🔬 35 orders of magnitude density range!");
}