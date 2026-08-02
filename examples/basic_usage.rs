use units::prelude::*;

fn main() {
    println!("=== Units Library - Basic Usage Example ===\n");

    // Create values with different units
    println!("1. Creating values:");
    let distance_m = Value::<Length, Meter>::new(1000.0);
    let distance_km = Value::<Length, Kilometer>::new(1.0);
    println!("  Distance in meters: {}", distance_m);
    println!("  Distance in kilometers: {}", distance_km);

    // Check equality (compares SI values)
    println!("\n2. Comparing values:");
    if distance_m == distance_km {
        println!("  1000 m == 1 km ✓");
    }

    // Unit conversion
    println!("\n3. Unit conversion:");
    let converted = distance_m.convert::<Kilometer>();
    println!("  1000 m = {}", converted);

    // Addition (type-safe!)
    println!("\n4. Addition (type-safe!):");
    let sum = distance_m + distance_km;
    println!("  1000 m + 1 km = {}", sum);

    // Addition with different units
    let cm_100 = Value::<Length, Centimeter>::new(100.0);
    let total = distance_m + cm_100;
    println!("  1000 m + 100 cm = {}", total);

    // Subtraction
    println!("\n5. Subtraction:");
    let diff = distance_m - Value::<Length, Meter>::new(500.0);
    println!("  1000 m - 500 m = {}", diff);

    // Scalar multiplication
    println!("\n6. Scalar multiplication:");
    let doubled = distance_m * 2.0;
    println!("  1000 m × 2 = {}", doubled);

    // Scalar division
    println!("\n7. Scalar division:");
    let halved = distance_m / 2.0;
    println!("  1000 m ÷ 2 = {}", halved);

    // Ratio (dividing same quantities)
    println!("\n8. Ratio (dimensionless):");
    let ratio = distance_m / Value::<Length, Meter>::new(500.0);
    println!("  1000 m ÷ 500 m = {} (dimensionless)", ratio);

    // Time examples
    println!("\n9. Time examples:");
    let time_s = Value::<Time, Second>::new(60.0);
    let time_min = Value::<Time, Minute>::new(1.0);
    println!("  60 seconds = {}", time_s.convert::<Minute>());
    println!("  1 minute = {}", time_min.convert::<Second>());

    let time_sum = time_s + time_min;
    println!("  60 s + 1 min = {}", time_sum);

    // Mass examples
    println!("\n10. Mass examples:");
    let mass_kg = Value::<Mass, Kilogram>::new(1.5);
    let mass_g = Value::<Mass, Gram>::new(500.0);
    println!("  1.5 kg = {}", mass_kg.convert::<Gram>());
    println!(
        "  Total mass: {} + {} = {}",
        mass_kg,
        mass_g,
        mass_kg + mass_g
    );

    // Type safety demonstration
    println!("\n11. Type safety:");
    println!("  ✓ Can add Length + Length");
    println!("  ✓ Can add Time + Time");
    println!("  ✗ Cannot add Length + Time (compile error!)");
    println!("  ✗ Cannot add Length + Mass (compile error!)");

    // Uncomment the following lines to see compile-time errors:
    // let invalid = distance_m + time_s;  // ERROR: mismatched types!
    // let invalid = distance_m + mass_kg; // ERROR: mismatched types!

    println!("\n=== All examples completed successfully! ===");
}
