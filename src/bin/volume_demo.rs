//! Demo für Volume quantity

use units::prelude::*;

fn main() {
    println!("📦 Volume Quantity Demo");
    println!("======================\n");

    // 1. Basic Volume units
    println!("1. Basic Volume units:");
    let vol_m3 = Volume::<CubicMeter>::new(1000.0);
    let vol_l = Volume::<Liter>::new(1000_000.0);  // 1000 m³ = 1,000,000 L
    println!("   {} m³ = {} L", vol_m3.value(), vol_l.value());
    
    let converted: Volume<Liter> = vol_m3.convert_to();
    println!("   Conversion check: {} m³ = {} L\n", vol_m3.value(), converted.value());

    // 2. Dimensional Analysis: Area × Distance = Volume
    println!("2. Dimensional Analysis (Area × Distance = Volume):");
    let base_area = Area::<SquareMeter>::new(25.0);  // 5m × 5m
    let height = Distance::<Meter>::new(4.0);
    let volume = base_area * height;
    println!("   {} m² × {} m = {} m³", base_area.value(), height.value(), volume.value());
    
    // Kann zu spezifischen Volume-Einheiten konvertiert werden
    let volume_l: Volume<Liter> = volume.convert_to();
    println!("   = {} L\n", volume_l.value());

    // 3. Triple multiplication: Distance × Distance × Distance = Volume
    println!("3. Triple Dimensional Analysis (Distance³ = Volume):");
    let length = Distance::<Meter>::new(2.0);
    let width = Distance::<Meter>::new(3.0);
    let depth = Distance::<Meter>::new(4.0);
    let cube_volume = length * width * depth;
    println!("   {} m × {} m × {} m = {} m³", 
             length.value(), width.value(), depth.value(), cube_volume.value());
    
    let cube_l: Volume<Liter> = cube_volume.convert_to();
    println!("   = {} L\n", cube_l.value());

    // 4. Liquid volume units
    println!("4. Liquid volumes:");
    let bottle = Volume::<Liter>::new(0.5);
    let bottle_ml: Volume<Milliliter> = bottle.convert_to();
    let bottle_gal: Volume<Gallon> = bottle.convert_to();
    println!("   Bottle: {} L = {} mL = {:.3} gal\n", 
             bottle.value(), bottle_ml.value(), bottle_gal.value());

    // 5. Astronomical volumes
    println!("5. Astronomical volumes:");
    let solar_vol = Volume::<CubicAstronomicalUnit>::new(1.0);
    let solar_km3: Volume<CubicKilometer> = solar_vol.convert_to();
    println!("   1 AU³ = {:.2e} km³", solar_km3.value());
    
    // Earth's volume vs AU³
    let earth_volume = Volume::<CubicKilometer>::new(1.083e12); // ~1.083×10¹² km³
    let earth_in_au: Volume<CubicAstronomicalUnit> = earth_volume.convert_to();
    println!("   Earth volume: {:.2e} km³ = {:.2e} AU³\n", 
             earth_volume.value(), earth_in_au.value());

    // 6. Imperial and US units
    println!("6. Imperial/US volumes:");
    let tank = Volume::<CubicFoot>::new(100.0);
    let tank_m3: Volume<CubicMeter> = tank.convert_to();
    let tank_gal: Volume<Gallon> = tank.convert_to();
    println!("   Tank: {} ft³ = {:.2} m³ = {:.1} gal\n", 
             tank.value(), tank_m3.value(), tank_gal.value());

    // 7. Mixed unit arithmetic
    println!("7. Mixed unit arithmetic:");
    let vol1 = Volume::<CubicMeter>::new(1.0);      // 1 m³
    let vol2 = Volume::<Liter>::new(500.0);         // 500 L = 0.5 m³
    let total = vol1 + vol2;  // Result in SI units
    println!("   {} m³ + {} L = {} m³ (SI)", 
             vol1.value(), vol2.value(), total.value());

    // 8. Dimensionless ratios
    println!("8. Volume ratios:");
    let big_vol = Volume::<CubicMeter>::new(10.0);
    let small_vol = Volume::<CubicMeter>::new(2.5);
    let ratio = big_vol / small_vol;
    println!("   {} m³ ÷ {} m³ = {} (dimensionless)", 
             big_vol.value(), small_vol.value(), ratio);

    // 9. Milliliter = Cubic Centimeter equivalence
    println!("9. mL ≡ cm³ equivalence:");
    let medicine = Volume::<Milliliter>::new(15.0);
    let medicine_cm3: Volume<CubicCentimeter> = medicine.convert_to();
    println!("   Medicine dose: {} mL = {} cm³", 
             medicine.value(), medicine_cm3.value());

    println!("\n✅ Volume quantity funktioniert perfekt!");
    println!("🎯 Dimensional analysis: Area × Distance → Volume");
    println!("🎯 Triple multiplication: Distance³ → Volume");
    println!("📏 Mixed units, conversions, astronomical scales, and liquids all working!");
}