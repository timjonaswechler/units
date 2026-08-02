use units::prelude::*;

fn main() {
    println!("=== Temperature Handling - The Celsius Problem Solution ===\n");

    // ============================================================================
    // Problem from idea.md:
    // 10°C + 20°C should NOT equal 30°C when done naively with SI conversion
    // (10+273.15) + (20+273.15) = 566.3 K ≠ (30+273.15) = 303.15 K
    // ============================================================================

    println!("1. THE PROBLEM:");
    println!("   If we naively add absolute temperatures in SI units:");
    println!("   (10°C in K) + (20°C in K) = (10+273.15) + (20+273.15) = 566.3 K");
    println!("   But 30°C = 303.15 K");
    println!("   This is WRONG!\n");

    println!("2. OUR SOLUTION:");
    println!("   Separate types: AbsoluteTemperature and TemperatureDifference\n");

    // ============================================================================
    // Absolute Temperatures - Cannot be added!
    // ============================================================================

    println!("3. Absolute Temperatures:");
    let temp1 = Value::<AbsoluteTemperature, Celsius>::new(10.0);
    let temp2 = Value::<AbsoluteTemperature, Celsius>::new(20.0);
    println!("   temp1 = {}", temp1);
    println!("   temp2 = {}", temp2);

    // This would be a COMPILE ERROR:
    // let invalid = temp1 + temp2;  // ❌ Cannot add absolute temperatures!
    println!("   temp1 + temp2 = COMPILE ERROR! ✓ (This is good!)\n");

    // ============================================================================
    // What we CAN do with absolute temperatures
    // ============================================================================

    println!("4. What we CAN do:");

    // Subtraction gives a difference
    let diff = temp2 - temp1;
    println!("   temp2 - temp1 = {} (TemperatureDifference)", diff);

    // Add a difference to an absolute temperature
    let result = temp1 + diff;
    println!("   temp1 + diff = {}", result);
    println!("   (This equals temp2 ✓)\n");

    // ============================================================================
    // Temperature Differences - CAN be added!
    // ============================================================================

    println!("5. Temperature Differences:");
    let change1 = Value::<TemperatureDifference, CelsiusDelta>::new(10.0);
    let change2 = Value::<TemperatureDifference, CelsiusDelta>::new(20.0);
    println!("   change1 = {} change", change1);
    println!("   change2 = {} change", change2);

    let total_change = change1 + change2;
    println!("   change1 + change2 = {} change", total_change);
    println!("   (This is 30°C change, which is correct! ✓)\n");

    // Apply the total change to a starting temperature
    let start = Value::<AbsoluteTemperature, Celsius>::new(0.0);
    let final_temp = start + total_change;
    println!("   0°C + 30°C change = {}", final_temp);

    // ============================================================================
    // Unit Conversions
    // ============================================================================

    println!("\n6. Unit Conversions:");

    let celsius = Value::<AbsoluteTemperature, Celsius>::new(20.0);
    let kelvin = celsius.convert::<Kelvin>();
    let fahrenheit = celsius.convert::<Fahrenheit>();

    println!("   20°C = {} = {}", kelvin, fahrenheit);

    let freezing = Value::<AbsoluteTemperature, Celsius>::new(0.0);
    let boiling = Value::<AbsoluteTemperature, Celsius>::new(100.0);
    println!(
        "   Water freezing: {} = {}",
        freezing,
        freezing.convert::<Kelvin>()
    );
    println!(
        "   Water boiling:  {} = {}",
        boiling,
        boiling.convert::<Kelvin>()
    );

    // Fahrenheit conversions
    let f32 = Value::<AbsoluteTemperature, Fahrenheit>::new(32.0);
    let f212 = Value::<AbsoluteTemperature, Fahrenheit>::new(212.0);
    println!("   32°F = {}", f32.convert::<Celsius>());
    println!("   212°F = {}", f212.convert::<Celsius>());

    // ============================================================================
    // Temperature Differences in Different Units
    // ============================================================================

    println!("\n7. Temperature Differences:");

    let delta_c = Value::<TemperatureDifference, CelsiusDelta>::new(10.0);
    let delta_k = delta_c.convert::<KelvinDelta>();
    let delta_f = delta_c.convert::<FahrenheitDelta>();

    println!("   A change of 10°C = {} = {}", delta_k, delta_f);
    println!("   (Note: A 10°C change = 10 K change, but 18°F change)");

    // ============================================================================
    // Real-world Example
    // ============================================================================

    println!("\n8. Real-world Example:");
    println!("   Morning temperature: 15°C");
    println!("   It warms up by: 8°C");
    println!("   Afternoon temperature: ?");

    let morning = Value::<AbsoluteTemperature, Celsius>::new(15.0);
    let warming = Value::<TemperatureDifference, CelsiusDelta>::new(8.0);
    let afternoon = morning + warming;

    println!("   Answer: {}", afternoon);

    // ============================================================================
    // Type Safety Demonstration
    // ============================================================================

    println!("\n9. Type Safety Summary:");
    println!("   ✓ AbsoluteTemperature - AbsoluteTemperature = TemperatureDifference");
    println!("   ✓ AbsoluteTemperature + TemperatureDifference = AbsoluteTemperature");
    println!("   ✓ AbsoluteTemperature - TemperatureDifference = AbsoluteTemperature");
    println!("   ✓ TemperatureDifference + TemperatureDifference = TemperatureDifference");
    println!("   ✓ TemperatureDifference - TemperatureDifference = TemperatureDifference");
    println!("   ✗ AbsoluteTemperature + AbsoluteTemperature = COMPILE ERROR!");

    println!("\n=== Problem from idea.md SOLVED! ===");

    // Uncomment to see compile errors:
    // let invalid = temp1 + temp2;  // ERROR: AbsoluteTemperature does not implement CanAddSameQuantity
}
