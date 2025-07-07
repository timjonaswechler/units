//! Test the core foundation
//!
//! Simple test to verify the dimensional analysis system works.

use units::prelude::*;
use units::units::base::*;
use units::core::composition::*;

fn main() {
    println!("🚀 Testing Core Foundation");
    
    // Test basic quantities
    let distance: Quantity<Meter> = Quantity::new(10.0);
    let time: Quantity<Second> = Quantity::new(2.0);
    
    println!("Distance: {} m", distance.value());
    println!("Time: {} s", time.value());
    
    // Test complex unit types
    type VelocityUnit = (Meter, Per<Second>);
    type AccelerationUnit = (Meter, Per<Exponent<Second, 2>>);
    
    println!("✅ Complex unit types compile correctly");
    
    // Test dimensional extraction
    println!("Meter dimensions: L={}, M={}, T={}", Meter::L, Meter::M, Meter::T);
    println!("Second dimensions: L={}, M={}, T={}", Second::L, Second::M, Second::T);
    
    // Test tuple composition
    type ForceUnit = (Kilogram, Meter, Per<Exponent<Second, 2>>);
    println!("Force unit dimensions: L={}, M={}, T={}", 
             ForceUnit::L, ForceUnit::M, ForceUnit::T);
    
    println!("🎯 Foundation test completed successfully!");
}