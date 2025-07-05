//! Demo des neuen Triple Unit Systems

use units::prelude::*;

fn main() {
    println!("🎉 Triple Unit System Demo");
    println!("==========================\n");

    // 1. ALIAS-BASIERTE EINHEITEN (elegant)
    println!("1. Alias-basierte Einheiten:");
    let velocity1 = Velocity::<MeterPerSecond>::new(10.0);
    let velocity2 = Velocity::<KilometerPerHour>::new(36.0);
    println!("   {} = {}", velocity1, velocity2.convert_to::<MeterPerSecond>());
    
    // 2. TUPEL-SYNTAX (maximum flexibility)  
    println!("\n2. Tupel-Syntax:");
    let velocity3 = Velocity::<(Meter, Second)>::new(15.0);
    let velocity4 = Velocity::<(Kilometer, Hour)>::new(54.0);
    println!("   {} m/s", velocity3.value());
    println!("   {} km/h = {} m/s", velocity4.value(), velocity4.convert_to::<(Meter, Second)>().value());

    // 3. PREFIX-SYSTEM (original flexibility)
    println!("\n3. Prefix-System:");
    let distance1 = Distance::<Meter>::new(1000.0);
    let distance2 = Distance::<Prefixed<Kilo, Meter>>::new(1.0);
    let distance3 = Distance::<Kilometer>::new(1.0); // Alias für Prefixed<Kilo, Meter>
    println!("   {} = {} = {}", distance1, distance2, distance3);

    // 4. ARITHMETIK (mixed units)
    println!("\n4. Gemischte Arithmetik:");
    let d1 = Distance::<Meter>::new(1500.0);
    let d2 = Distance::<Kilometer>::new(0.5);
    let total = d1 + d2.convert_to::<Meter>();
    println!("   {} + {} = {} m", d1.value(), d2.value(), total.value());

    // 5. SKALARE OPERATIONEN
    println!("\n5. Skalare Operationen:");
    let distance = Distance::<Meter>::new(100.0);
    let doubled = distance * 2.0;
    let half = distance / 2.0;
    println!("   {} * 2 = {} m", distance.value(), doubled.value());
    println!("   {} / 2 = {} m", distance.value(), half.value());

    // 6. ASTRONOMICAL UNITS
    println!("\n6. Astronomische Einheiten:");
    let earth_distance = Distance::<AstronomicalUnit>::new(1.0);
    let mars_distance = Distance::<AstronomicalUnit>::new(1.52);
    let earth_km: Distance<Kilometer> = earth_distance.convert_to();
    println!("   Erde: {} = {:.0} km", earth_distance.value(), earth_km.value());
    println!("   Mars: {} AU", mars_distance.value());

    println!("\n✅ Alle drei Syntax-Ansätze funktionieren perfekt!");
}