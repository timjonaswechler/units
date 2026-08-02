// This example demonstrates compile-time type safety

use units::prelude::*;

fn main() {
    println!("=== Type Safety Demonstration ===\n");

    // These work fine:
    let distance = Value::<Length, Meter>::new(100.0);
    let time = Value::<Time, Second>::new(10.0);
    let mass = Value::<Mass, Kilogram>::new(5.0);

    // ✓ Adding same quantities
    let _sum1 = distance + Value::<Length, Meter>::new(50.0);
    println!("✓ Can add Length + Length");

    let _sum2 = time + Value::<Time, Second>::new(5.0);
    println!("✓ Can add Time + Time");

    let _sum3 = mass + Value::<Mass, Kilogram>::new(2.0);
    println!("✓ Can add Mass + Mass");

    // ✓ Converting between units of same quantity
    let _km = distance.convert::<Kilometer>();
    println!("✓ Can convert Meter to Kilometer");

    let _minutes = time.convert::<Minute>();
    println!("✓ Can convert Second to Minute");

    // ✓ Scalar operations
    let _doubled = distance * 2.0;
    println!("✓ Can multiply Length by scalar");

    let _halved = time / 2.0;
    println!("✓ Can divide Time by scalar");

    // ✓ Ratio of same quantities
    let _ratio = distance / Value::<Length, Meter>::new(50.0);
    println!("✓ Can divide Length by Length (gives dimensionless ratio)");

    println!("\n=== Type Safety Verification ===");
    println!("The following operations would cause COMPILE ERRORS:\n");
    println!("  ✗ distance + time       // Cannot add Length and Time");
    println!("  ✗ distance + mass       // Cannot add Length and Mass");
    println!("  ✗ time + mass           // Cannot add Time and Mass");
    println!("  ✗ distance.convert::<Second>()  // Cannot convert Length to Time");

    println!("\n✓ All type-safe operations completed successfully!");
    println!("✓ The type system prevents invalid operations at compile time!");

    // Uncomment these to see the compile errors:

    // let invalid1 = distance + time;
    // ERROR: no implementation for `Value<Length, Meter> + Value<Time, Second>`

    // let invalid2 = distance + mass;
    // ERROR: no implementation for `Value<Length, Meter> + Value<Mass, Kilogram>`

    // let invalid3 = distance.convert::<Second>();
    // ERROR: the trait bound `Second: Unit<BaseQuantity = Length>` is not satisfied
}
