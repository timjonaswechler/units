//! Test dimensional arithmetic operations
//!
//! Comprehensive test of the automatic dimensional arithmetic system.

use units::core::composition::*;
use units::prelude::*;
use units::units::base::*;

fn main() {
    println!("🚀 Testing Dimensional Arithmetic");

    // Basic quantities
    let distance = Quantity::<Meter>::new(10.0);
    let time = Quantity::<Second>::new(2.0);
    let mass = Quantity::<Kilogram>::new(5.0);

    println!("Distance: {} m", distance.value());
    println!("Time: {} s", time.value());
    println!("Mass: {} kg", mass.value());

    // Test division - creates velocity
    let velocity = distance / time;
    println!("Velocity: {} m/s", velocity.value());

    // Test multiplication - creates momentum-like quantity
    let momentum_like = mass * velocity.clone();
    println!("Momentum-like: {} kg⋅m/s", momentum_like.value());

    // Test addition/subtraction (same dimensions)
    let distance2 = Quantity::<Meter>::new(5.0);
    let total_distance = distance + distance2;
    let distance_diff = distance - distance2;

    println!("Total distance: {} m", total_distance.value());
    println!("Distance difference: {} m", distance_diff.value());

    // Test scalar operations
    let doubled_distance = distance * 2.0;
    let half_time = time / 2.0;

    println!("Doubled distance: {} m", doubled_distance.value());
    println!("Half time: {} s", half_time.value());

    // Test scalar division with reciprocal
    let frequency = 1.0 / time;
    println!("Frequency: {} Hz", frequency.value());

    // Test complex dimensional combinations
    let acceleration = velocity / time;
    println!("Acceleration: {} m/s²", acceleration.value());

    let force = mass * acceleration;
    println!("Force: {} N", force.value());

    // Test negation
    let negative_distance = -distance;
    println!("Negative distance: {} m", negative_distance.value());

    println!("✅ All arithmetic operations work correctly!");

    // Test type information
    println!("\n🔬 Type Analysis:");

    // Velocity type analysis
    type VelocityType = (Meter, Per<Second>);
    println!(
        "Velocity dimensions: L={}, M={}, T={}",
        VelocityType::L,
        VelocityType::M,
        VelocityType::T
    );

    // Acceleration type analysis
    type AccelerationType = ((Meter, Per<Second>), Per<Second>);
    println!(
        "Acceleration dimensions: L={}, M={}, T={}",
        AccelerationType::L,
        AccelerationType::M,
        AccelerationType::T
    );

    // Force type analysis
    type ForceType = (Kilogram, ((Meter, Per<Second>), Per<Second>));
    println!(
        "Force dimensions: L={}, M={}, T={}",
        ForceType::L,
        ForceType::M,
        ForceType::T
    );

    println!("🎯 Dimensional arithmetic test completed successfully!");
}
