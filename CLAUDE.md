# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 📍 CURRENT STATUS & TASK OVERVIEW

### Currently Working On: **Task 18 - Physical Constants Module** (IN PROGRESS)

**Current Issue:** Fixing compilation errors in the constants module
- ✅ Created comprehensive constants structure with 8 sub-modules
- ✅ Implemented all major physical constants (fundamental, astronomical, atomic, electromagnetic, thermodynamic, nuclear, mathematical)
- 🔄 **CURRENTLY FIXING:** ElectricCharge quantity compilation errors (missing UnitComposition trait methods)
- ⏳ **NEXT:** Test constants demo and complete Task 18

### Completed Tasks (1-17): ✅
1. ✅ Analyze current codebase structure
2. ✅ Design new granular directory structure  
3. ✅ Create brutal implementation analysis
4. ✅ Design alias-based unit composition system
5. ✅ Create core macro system for unit generation
6. ✅ Implement basic unit types (Distance, Time, Mass, etc.)
7. ✅ Add prefix system (Kilo, Mega, Milli, etc.)
8. ✅ Implement variadic multi-unit syntax
9. ✅ Add automatic dimensional analysis
10. ✅ Implement mixed unit arithmetic
11. ✅ Add alias system
12. ✅ Create tests for new system
13. ✅ Implement dimensional analysis arithmetic (distance / time = velocity)
14. ✅ Add automatic mixed-unit arithmetic (Meter + Kilometer)
15. ✅ Complete remaining quantity types (Force, Energy, Power, Area, Volume, Angle, Frequency, Luminosity, Pressure, Density, Angular Velocity, Momentum)
16. ✅ Add comprehensive composed unit aliases (Newton, Joule, Watt)
17. ✅ Implement display/formatting improvements (scientific notation, precision control, intelligent unit selection, multiple display styles)

### Pending Tasks (19-22): ⏳

**Task 19: Create comprehensive documentation** (HIGH PRIORITY NEXT)
- Add module-level documentation with examples
- Create comprehensive README.md with usage examples
- Document all quantity types and their units
- Add examples for dimensional analysis
- Document formatting system capabilities
- Add performance guidelines and best practices

**Task 20: Add serialization support (Serde)**
- Implement Serialize/Deserialize for all quantity types
- Add JSON, YAML, and binary serialization support
- Create serialization examples and tests
- Ensure precision is maintained during serialization

**Task 21: Performance optimization and benchmarks**
- Create comprehensive benchmarks for arithmetic operations
- Optimize conversion operations
- Profile memory usage and allocation patterns
- Compare performance with raw f64 operations
- Document performance characteristics

**Task 22: Create example projects and tutorials**
- Create orbital mechanics example using astronomical constants
- Build thermodynamics calculation example
- Create electromagnetic field calculation example
- Add scientific computing tutorial
- Create migration guide from other unit libraries

## 🔧 Commands

**Build and Check:**
- `cargo check` - Quick compilation check
- `cargo build` - Full build
- `cargo build --release` - Optimized build

**Testing:**
- `cargo test` - Run all tests
- `cargo test --lib` - Run library unit tests only
- `cargo test --doc` - Run documentation tests only

**Demos:**
- `cargo run --bin formatting_demo` - Test enhanced formatting system
- `cargo run --bin constants_demo` - Test physical constants (when compilation fixed)

**Documentation:**
- `cargo doc` - Generate documentation
- `cargo doc --open` - Generate and open documentation

## 🏗️ Architecture

This is a **standalone Rust crate for type-safe physics units with dimensional analysis**, designed for scientific computing and astronomical calculations. It is NOT a game engine or simulation - it's a pure data library focused on preventing unit conversion errors at compile time.

### Core Features Implemented:
- **Triple Unit System:** Tuple syntax `(Meter, Second)`, Alias syntax `MeterPerSecond`, Prefix syntax `Prefixed<Kilo, Meter>`
- **Dimensional Analysis:** Automatic type inference for operations (distance / time = velocity)
- **Comprehensive Formatting:** Scientific notation, precision control, context-aware unit selection
- **Physical Constants:** 100+ constants from fundamental physics, astronomy, atomic physics
- **Mixed Unit Arithmetic:** Automatic conversions (5.2 km + 1800 m = 7.0 km)
- **Zero-Cost Abstractions:** Compile-time dimensional checking with runtime f64 performance

### Module Structure:
```
src/
├── core/           # Core quantity and dimension system
├── prefix/         # Metric prefix system (Kilo, Mega, etc.)
├── quantities/     # All quantity types (Distance, Mass, Energy, etc.)
├── arithmetic/     # Dimensional analysis and mixed-unit operations  
├── formatting/     # Advanced display and formatting system
├── constants/      # Physical constants with proper units
├── aliases/        # Convenient unit aliases
└── macros/         # Code generation macros
```

### Current Issues to Fix:
1. **ElectricCharge compilation errors** - Missing `to_si_factor()` and `from_si_factor()` methods in UnitComposition implementations
2. **Constants demo compilation** - Constants not properly imported due to above issue
3. **Ambiguous glob imports** - Multiple modules re-export same constants, causing warnings

### Next Steps After Current Fix:
1. Complete Task 18 (Physical Constants)
2. Start Task 19 (Comprehensive Documentation)
3. Add examples showing real-world physics calculations
4. Optimize performance and add benchmarks
