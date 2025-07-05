//! Demo für dimensionslose Division

use units::prelude::*;

fn main() {
    println!("🔢 Dimensionslose Division Demo");
    println!("================================\n");

    // 1. Distance ÷ Distance = dimensionslos
    println!("1. Distance ÷ Distance:");
    let distance1 = Distance::<Meter>::new(100.0);
    let distance2 = Distance::<Meter>::new(50.0);
    let ratio1 = distance1 / distance2;
    println!("   {} m ÷ {} m = {} (dimensionslos)", distance1.value(), distance2.value(), ratio1);
    println!("   Typ: Distance ÷ Distance → f64\n");

    // 2. Verschiedene Distance-Einheiten
    println!("2. Mixed Distance Units:");
    let meters = Distance::<Meter>::new(1000.0);
    let kilometers = Distance::<Kilometer>::new(1.0);
    let ratio2 = meters / kilometers;
    println!("   {} m ÷ {} km = {} (dimensionslos)", meters.value(), kilometers.value(), ratio2);
    println!("   (1000 m ÷ 1000 m = 1.0)");
    println!("   Typ: Distance<Meter> ÷ Distance<Kilometer> → f64\n");

    // 3. Time ÷ Time = dimensionslos
    println!("3. Time ÷ Time:");
    let time1 = Time::<Hour>::new(2.0);
    let time2 = Time::<Minute>::new(30.0);
    let ratio3 = time1 / time2;
    println!("   {} h ÷ {} min = {} (dimensionslos)", time1.value(), time2.value(), ratio3);
    println!("   (7200 s ÷ 1800 s = 4.0)");
    println!("   Typ: Time<Hour> ÷ Time<Minute> → f64\n");

    // 4. Mass ÷ Mass = dimensionslos
    println!("4. Mass ÷ Mass:");
    let mass1 = Mass::<Kilogram>::new(5.0);
    let mass2 = Mass::<Gram>::new(1000.0);
    let ratio4 = mass1 / mass2;
    println!("   {} kg ÷ {} g = {} (dimensionslos)", mass1.value(), mass2.value(), ratio4);
    println!("   (5000 g ÷ 1000 g = 5.0)");
    println!("   Typ: Mass<Kilogram> ÷ Mass<Gram> → f64\n");

    // 5. Velocity ÷ Velocity = dimensionslos
    println!("5. Velocity ÷ Velocity:");
    let velocity1 = Velocity::<(Meter, Second)>::new(10.0);
    let velocity2 = Velocity::<KilometerPerHour>::new(36.0);
    let ratio5 = velocity1 / velocity2;
    println!("   {} m/s ÷ {} km/h = {} (dimensionslos)", velocity1.value(), velocity2.value(), ratio5);
    println!("   (10 m/s ÷ 10 m/s = 1.0)");
    println!("   Typ: Velocity<Tuple> ÷ Velocity<KmH> → f64\n");

    // 6. Force ÷ Force = dimensionslos
    println!("6. Force ÷ Force:");
    let force1 = Force::<Newton>::new(100.0);
    let force2 = Force::<Dyne>::new(1_000_000.0);
    let ratio6 = force1 / force2;
    println!("   {} N ÷ {} dyn = {} (dimensionslos)", force1.value(), force2.value(), ratio6);
    println!("   (100 N ÷ 10 N = 10.0)");
    println!("   Typ: Force<Newton> ÷ Force<Dyne> → f64\n");

    // 7. Praktisches Beispiel: Verhältnisse berechnen
    println!("7. Praktische Anwendung - Verhältnisse:");
    let earth_radius = Distance::<Kilometer>::new(6371.0);
    let moon_radius = Distance::<Kilometer>::new(1737.0);
    let size_ratio = earth_radius / moon_radius;
    println!("   Erde Radius: {} km", earth_radius.value());
    println!("   Mond Radius: {} km", moon_radius.value());
    println!("   Größenverhältnis: {:.2} (Erde ist {:.2}x größer)", size_ratio, size_ratio);
    println!("   Typ: Distance ÷ Distance → f64\n");

    // 8. Energieverhältnisse
    println!("8. Energieverhältnisse:");
    let energy_joule = Energy::<Joule>::new(1000.0);
    let energy_cal = Energy::<Calorie>::new(239.0);
    let energy_ratio = energy_joule / energy_cal;
    println!("   {} J ÷ {} cal = {:.3} (dimensionslos)", energy_joule.value(), energy_cal.value(), energy_ratio);
    println!("   Typ: Energy<Joule> ÷ Energy<Calorie> → f64\n");

    // 9. Zeigen was mit dem Ergebnis möglich ist
    println!("9. Verwendung der dimensionslosen Ergebnisse:");
    println!("   let verhältnis = distance1 / distance2;  // = {}", ratio1);
    println!("   let prozent = verhältnis * 100.0;        // = {}%", ratio1 * 100.0);
    println!("   let ist_größer = verhältnis > 1.0;       // = {}", ratio1 > 1.0);
    
    println!("\n✅ Dimensionslose Division funktioniert perfekt!");
    println!("🎯 Alle Verhältnisse werden korrekt als f64 zurückgegeben!");
    println!("📊 Ideal für Vergleiche, Prozentangaben und Verhältnisrechnungen!");
}