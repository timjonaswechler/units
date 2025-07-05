//! Demo für dimensionale Analyse Arithmetik

use units::prelude::*;

fn main() {
    println!("🧮 Dimensionale Analyse Demo");
    println!("============================\n");

    // 1. Distance × Distance = Area
    println!("1. Distance × Distance = Area:");
    let length = Distance::<Meter>::new(5.0);
    let width = Distance::<Meter>::new(3.0);
    let area = length * width;
    println!("   {} m × {} m = {} m²", length.value(), width.value(), area.value());
    println!("   Typ: Distance × Distance → Area\n");

    // 2. Area × Distance = Volume
    println!("2. Area × Distance = Volume:");
    let height = Distance::<Meter>::new(2.0);
    let volume = area * height;
    println!("   {} m² × {} m = {} m³", area.value(), height.value(), volume.value());
    println!("   Typ: Area × Distance → Volume\n");

    // 3. Distance ÷ Time = Velocity
    println!("3. Distance ÷ Time = Velocity:");
    let distance = Distance::<Meter>::new(100.0);
    let time = Time::<Second>::new(10.0);
    let velocity = distance / time;
    println!("   {} m ÷ {} s = {} m/s", distance.value(), time.value(), velocity.value());
    println!("   Typ: Distance ÷ Time → Velocity\n");

    // 4. Velocity ÷ Time = Acceleration
    println!("4. Velocity ÷ Time = Acceleration:");
    let velocity2 = Velocity::<(Meter, Second)>::new(20.0);
    let time2 = Time::<Second>::new(4.0);
    let acceleration = velocity2 / time2;
    println!("   {} m/s ÷ {} s = {} m/s²", velocity2.value(), time2.value(), acceleration.value());
    println!("   Typ: Velocity ÷ Time → Acceleration\n");

    // 5. Mass × Acceleration = Force
    println!("5. Mass × Acceleration = Force:");
    let mass = Mass::<Kilogram>::new(5.0);
    let acceleration2 = Acceleration::<(Meter, Second)>::new(2.0);
    let force = mass * acceleration2;
    println!("   {} kg × {} m/s² = {} N", mass.value(), acceleration2.value(), force.value());
    println!("   Typ: Mass × Acceleration → Force\n");

    // 6. Force × Distance = Energy (Work)
    println!("6. Force × Distance = Energy (Work):");
    let force2 = Force::<Newton>::new(10.0);
    let distance2 = Distance::<Meter>::new(5.0);
    let energy = force2 * distance2;
    println!("   {} N × {} m = {} J", force2.value(), distance2.value(), energy.value());
    println!("   Typ: Force × Distance → Energy\n");

    // 7. Mass × Velocity = Momentum
    println!("7. Mass × Velocity = Momentum:");
    let mass2 = Mass::<Kilogram>::new(2.0);
    let velocity3 = Velocity::<(Meter, Second)>::new(10.0);
    let momentum = mass2 * velocity3;
    println!("   {} kg × {} m/s = {} kg⋅m/s", mass2.value(), velocity3.value(), momentum.value());
    println!("   Typ: Mass × Velocity → Momentum\n");

    // 8. Energy ÷ Time = Power
    println!("8. Energy ÷ Time = Power:");
    let energy2 = Energy::<Joule>::new(1000.0);
    let time3 = Time::<Second>::new(10.0);
    let power = energy2 / time3;
    println!("   {} J ÷ {} s = {} W", energy2.value(), time3.value(), power.value());
    println!("   Typ: Energy ÷ Time → Power\n");

    // 9. Mixed Units with Dimensional Analysis
    println!("9. Mixed Units mit Dimensionsanalyse:");
    let distance_km = Distance::<Kilometer>::new(1.0);  // 1 km
    let time_hour = Time::<Hour>::new(1.0);            // 1 h
    let speed = distance_km / time_hour;               // km/h → m/s in SI
    println!("   {} km ÷ {} h = {} m/s (SI)", 1.0, 1.0, speed.value());
    println!("   (1 km/h = {} m/s)", speed.value());
    println!("   Typ: Distance<Kilometer> ÷ Time<Hour> → Velocity<SI>\n");

    // 10. Dimensionless Division
    println!("10. Dimensionslose Division:");
    let d1 = Distance::<Meter>::new(100.0);
    let d2 = Distance::<Meter>::new(50.0);
    let ratio = d1 / d2;
    println!("    {} m ÷ {} m = {} (dimensionslos)", d1.value(), d2.value(), ratio);
    println!("    Typ: Distance ÷ Distance → f64\n");

    println!("✅ Alle dimensionalen Transformationen funktionieren!");
    println!("🎯 Das System erkennt automatisch die korrekte Zieldimension!");
}