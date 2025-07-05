//! Demo für Area quantity

use units::prelude::*;

fn main() {
    println!("📐 Area Quantity Demo");
    println!("====================\n");

    // 1. Basic Area units
    println!("1. Basic Area units:");
    let area_m2 = Area::<SquareMeter>::new(100.0);
    let area_km2 = Area::<SquareKilometer>::new(0.0001);  // 100 m² = 0.0001 km²
    println!("   {} m² = {} km²", area_m2.value(), area_km2.value());
    
    let converted: Area<SquareKilometer> = area_m2.convert_to();
    println!("   Conversion check: {} m² = {} km²\n", area_m2.value(), converted.value());

    // 2. Dimensional Analysis: Distance × Distance = Area
    println!("2. Dimensional Analysis (Distance × Distance = Area):");
    let length = Distance::<Meter>::new(10.0);
    let width = Distance::<Meter>::new(5.0);
    let area = length * width;
    println!("   {} m × {} m = {} m²", length.value(), width.value(), area.value());
    
    // Kann zu spezifischen Area-Einheiten konvertiert werden
    let area_cm2: Area<SquareCentimeter> = area.convert_to();
    println!("   = {} cm²\n", area_cm2.value());

    // 3. Agricultural units
    println!("3. Agricultural/Land units:");
    let field_hectare = Area::<Hectare>::new(2.5);
    let field_m2: Area<SquareMeter> = field_hectare.convert_to();
    let field_acre: Area<Acre> = field_hectare.convert_to();
    println!("   Field: {} ha = {} m² = {:.2} acres\n", 
             field_hectare.value(), field_m2.value(), field_acre.value());

    // 4. Astronomical areas
    println!("4. Astronomical areas:");
    let solar_area = Area::<SquareAstronomicalUnit>::new(1.0);
    let solar_km2: Area<SquareKilometer> = solar_area.convert_to();
    println!("   1 AU² = {:.2e} km²", solar_km2.value());
    
    // Erde Oberfläche vs AU²
    let earth_surface = Area::<SquareKilometer>::new(510_100_000.0);
    let earth_in_au: Area<SquareAstronomicalUnit> = earth_surface.convert_to();
    println!("   Earth surface: {} km² = {:.2e} AU²\n", 
             earth_surface.value(), earth_in_au.value());

    // 5. Nuclear physics cross-sections
    println!("5. Nuclear cross-sections:");
    let cross_section = Area::<Barn>::new(1.0);
    let cross_m2: Area<SquareMeter> = cross_section.convert_to();
    println!("   1 barn = {:.2e} m²", cross_m2.value());
    
    let milli_barn = Area::<MilliBarn>::new(1000.0);
    let milli_barn_barns: Area<Barn> = milli_barn.convert_to();
    println!("   1000 mb = {} b\n", milli_barn_barns.value());

    // 6. Mixed unit arithmetic
    println!("6. Mixed unit arithmetic:");
    let area1 = Area::<SquareMeter>::new(1000.0);  // 1000 m²
    let area2 = Area::<Hectare>::new(0.5);         // 0.5 ha = 5000 m²
    let total = area1 + area2;  // Result in SI units
    println!("   {} m² + {} ha = {} m² (SI)", 
             area1.value(), area2.value(), total.value());

    // 7. Dimensionless ratios
    println!("7. Area ratios:");
    let big_area = Area::<SquareKilometer>::new(10.0);
    let small_area = Area::<SquareKilometer>::new(2.0);
    let ratio = big_area / small_area;
    println!("   {} km² ÷ {} km² = {} (dimensionless)", 
             big_area.value(), small_area.value(), ratio);

    println!("\n✅ Area quantity funktioniert perfekt!");
    println!("🎯 Dimensional analysis: Distance × Distance → Area");
    println!("📏 Mixed units, conversions, and astronomical scales all working!");
}