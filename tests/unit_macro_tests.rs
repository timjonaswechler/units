// Tests for the unit!() macro system - Phase 5

use units::*;

#[test]
fn test_unit_macro_basic() {
    // Test basic unit!() macro usage
    let distance = unit!(Distance, Meter, 10.0);
    assert_eq!(distance.value(), 10.0);
    
    let time = unit!(Time, Second, 5.0);
    assert_eq!(time.value(), 5.0);
    
    let mass = unit!(Mass, Kilogram, 2.5);
    assert_eq!(mass.value(), 2.5);
}

#[test]
fn test_unit_macro_with_derived_quantities() {
    // Test with derived quantities like Force
    let force = unit!(Force, Newton, 100.0);
    assert_eq!(force.value(), 100.0);
    
    let area = unit!(Area, SquareMeter, 4.0);
    assert_eq!(area.value(), 4.0);
    
    let speed = unit!(Speed, MeterPerSecond, 25.0);
    assert_eq!(speed.value(), 25.0);
}

#[test]
fn test_unit_macro_arithmetic() {
    // Test that unit!() values work with arithmetic operators
    let force = unit!(Force, Newton, 100.0);
    let area = unit!(Area, SquareMeter, 4.0);
    
    // Force / Area should give Pressure
    let pressure_result = force / area;
    assert_eq!(pressure_result.value, 25.0);
    assert_eq!(pressure_result.resolve_quantity_name(), Some("Pressure"));
}

#[test]
fn test_unit_macro_with_integers() {
    // Test that unit!() works with integer literals
    let distance = unit!(Distance, Meter, 42);
    assert_eq!(distance.value(), 42);
    
    let time = unit!(Time, Second, 10);
    assert_eq!(time.value(), 10);
}

#[test] 
fn test_unit_macro_comprehensive_physics() {
    // Test a comprehensive physics scenario
    let mass = unit!(Mass, Kilogram, 10.0);      // 10 kg
    // NOTE: We don't have a m/s² unit yet, so using direct acceleration dimension
    // let acceleration = unit!(Acceleration, MeterPerSecondSquared, 9.8); // 9.8 m/s²
    
    // F = ma (this should work once we implement acceleration units properly)
    // For now, let's test force directly
    let force = unit!(Force, Newton, 98.0);      // 98 N
    let area = unit!(Area, SquareMeter, 2.0);    // 2 m²
    
    let pressure_result = force / area;           // Should be 49 Pa
    assert_eq!(pressure_result.value, 49.0);
    assert_eq!(pressure_result.resolve_quantity_name(), Some("Pressure"));
}

#[test]
fn test_unit_macro_type_safety() {
    // Verify that unit!() creates properly typed values
    let distance1 = unit!(Distance, Meter, 5.0);
    let distance2 = unit!(Distance, Meter, 3.0);
    
    // Addition should work (same quantity types)
    let total_distance = distance1 + distance2;
    assert_eq!(total_distance.value(), 8.0);
    
    // Multiplication should create area
    let area_result = distance1 * distance2;
    assert_eq!(area_result.value, 15.0);
    assert_eq!(area_result.resolve_quantity_name(), Some("Area"));
}