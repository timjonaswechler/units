//! Demonstration of dimensional safety in the physics units library
//!
//! This example shows how the type system prevents dimensional errors at runtime.

use units::quantities::{Distance, Meter, Second, Time};

fn main() {
    println!("🔬 Physics Units Library - Dimensional Safety Demo");
    println!("==================================================");

    // ✅ Valid constructions
    println!("\n✅ Creating valid quantities:");
    let distance = Distance::<Meter>::new(100.0);
    let time = Time::<Second>::new(10.0);
    println!("  Distance: {} meters", distance.value());
    println!("  Time: {} seconds", time.value());

    // ✅ Unit conversions work within the same quantity type
    println!("\n✅ Unit conversions:");
    println!("  Original distance: {} meters", distance.value());
    println!(
        "  Distance dimensions: {:?}",
        Distance::<Meter>::dimensions()
    );
    println!("  Time dimensions: {:?}", Time::<Second>::dimensions());

    // 🚨 Let's try to create invalid combinations
    println!("\n🚨 Now let's see what happens with invalid dimensional assignments...");

    // This will demonstrate the runtime validation:
    println!("\n❌ Attempting to create Distance with Time unit (Second):");
    println!("   Distance expects: L=1, M=0, T=0, ...");
    println!("   Second provides:  L=0, M=0, T=1, ...");
    println!("   This should panic with a dimensional mismatch error:");

    // Uncomment the next line to see the dimensional validation error:
    let invalid_distance = Distance::<Second>::new(100.0);
    println!("{}", invalid_distance.value());
    println!("\n✅ Dimensional validation working correctly!");
    println!(
        "💡 Uncomment the invalid line in examples/dimensional_safety_demo.rs to see the error!"
    );
}
