# Physics Units System

A type-safe, ergonomic unit system for physical calculations in Rust with automatic SI conversions and precision control.

## Features

- **Type Safety**: Units are checked at compile time, preventing mixing incompatible quantities
- **Ergonomic API**: Clean, readable syntax with no boilerplate
- **Flexible Units**: Support for SI base units, prefixed units, and compound units
- **Precision Control**: Built-in rounding and truncation methods
- **Zero Runtime Cost**: All unit information is encoded in types

## Quick Start

```rust
use units::features::DefaultFloat;
use units::prelude::*;
use units::{define_quantity, define_units};

// Conversion constants
const METERS_PER_AU: DefaultFloat = 1.495978707e11;
const METERS_PER_EARTH_RADIUS: DefaultFloat = 6.3781e6;
const METERS_PER_SUN_RADIUS: DefaultFloat = 6.96e8;
const METERS_PER_LIGHT_YEAR: DefaultFloat = 9.4607304725808e15;
const METERS_PER_PARSEC: DefaultFloat = 3.0856775814913673e16;

define_quantity!(Distance); // Length

define_units! {
    base_unit: Meter = 1.0,
    units: {
        AstronomicalUnit = METERS_PER_AU,
        EarthRadius = METERS_PER_EARTH_RADIUS,
        SunRadius = METERS_PER_SUN_RADIUS,
        LightYear = METERS_PER_LIGHT_YEAR,
        Parsec = METERS_PER_PARSEC,
    }
}

pub type Kilometer = Prefixed<Kilo, Meter>;

// Use the units
let distance: Distance<Meter> = Distance::new(1500.0);
let distance_km: Distance<Kilometer> = Distance::new(1.5);

// Both represent the same physical quantity
assert_eq!(distance.si(), 1500.0);  // 1500 m
assert_eq!(distance_km.si(), 1500.0); // 1.5 km = 1500 m

// Compound units
let velocity: Velocity<(Kilometer, Per<Hour>)> = Velocity::new(100.0);
assert_eq!(velocity.si_rounded(1), 27.8); // 100 km/h = 27.8 m/s
```

## Core Concepts

### Physical Quantities
Define type-safe wrappers for physical measurements:
```rust
define_quantity!(Mass);
define_quantity!(Power);
define_quantity!(Energy);
```

### Units and Prefixes
```rust
// Metric prefixes
define_prefix!(Kilo, 1000.0);
define_prefix!(Milli, 0.001);

const KG_PER_GRAM: f32 = 0.001;
const KG_PER_EARTH_MASS: f32 = 5.972e24;
const KG_PER_SOLAR_MASS: f32 = 1.989e30;

define_quantity!(Mass);

// Define Mass units with astronomical focus
// Note: Using Gram as base unit to avoid confusion with prefix system
// Kilogram will be available as Prefixed<Kilo, Gram>
define_units! {
    base_unit: Gram = KG_PER_GRAM,
    units: {
        EarthMass = KG_PER_EARTH_MASS,
        SolarMass = KG_PER_SOLAR_MASS,
    }
}
```

### Flexible Unit Combinations
```rust
// Prefixed units
Distance<Prefixed<Kilo, Meter>>     // Kilometer
Power<Prefixed<Mega, Watt>>         // Megawatt

// Compound units
Velocity<(Meter, Per<Second>)>      // m/s
Acceleration<(Meter, Per<Exponent<Second, 2>>)>  // m/s²
```

## Precision Control

Get values with specified decimal precision:

```rust
let velocity: Velocity<(Kilometer, Per<Hour>)> = Velocity::new(100.0);

// SI conversions with precision
velocity.si()              // 27.77777777777778
velocity.si_rounded(2)     // 27.78
velocity.si_truncated(1)   // 27.7

// Original values with precision
let distance: Distance<Meter> = Distance::new(12.3456789);
distance.value_rounded(2)    // 12.35
distance.value_truncated(3)  // 12.345
```

## API Reference

### Core Methods
- `.value()` - Get value in original units
- `.si()` - Get value in SI base units
- `.si_rounded(n)` - SI value rounded to n decimal places
- `.si_truncated(n)` - SI value truncated to n decimal places
- `.value_rounded(n)` - Original value rounded to n decimal places
- `.value_truncated(n)` - Original value truncated to n decimal places

### Composition Types
- `Per<Unit>` - Inverse units (1/Unit)
- `Prefixed<Prefix, Unit>` - Metric prefixes
- `Exponent<Unit, N>` - Unit powers (Unit^N)
- `(Unit1, Unit2)` - Unit multiplication

## Design Philosophy

This library prioritizes:
1. **Compile-time safety** over runtime flexibility
2. **Ergonomic syntax** over complex type systems
3. **Clear documentation** in code through readable unit types
4. **Practical usability** for scientific computing

## ToDo
- [ ] Create tests for all defined units and quantities calculate their SI equivalents
- [ ] Add more Unit-Tupel combinations
- [ ] Check if Watt can be described as Joule/Second and with SI units
- [ ] Test if a Unit-Ristriction can be implemented to prevent mixing incompatible units
- [ ] Test if there is a way to implement to dynamicly change defined prefixes
- [ ] Change current presision control to a rust crate feature
- [ ] Add more examples to the documentation
- [ ] Add unit String output for all defined units if a variable is printed e.g. `println!("{:?}", distance);` should print `Distance<Meter>(1500.0 m)` or `1500.0 m`


## License

Licensed under [MIT license](LICENSE).
```
