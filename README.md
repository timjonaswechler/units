# Units - Type-Safe Physical Units for Rust

A compile-time type-safe unit system for Rust with dimensional analysis and zero runtime overhead.

## Features

- **🔒 Type-Safe**: Catch unit errors at compile time, not runtime
- **⚡ Zero-Cost**: All dimensional checks happen at compile time
- **🔧 Extensible**: Easy to add new units with macros
- **📏 Comprehensive**: Includes common SI and Imperial units
- **🌡️ Temperature-Aware**: Solves the "Celsius Problem" with separate absolute/relative types
- **🧮 Arithmetic**: Natural mathematical operations with automatic unit conversions

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
units = "0.1.0"
```

## Usage Example

```rust
use units::prelude::*;

fn main() {
    // Create values with units
    let distance = Value::<Length, Meter>::new(1000.0);
    let time = Value::<Time, Second>::new(10.0);

    // Type-safe arithmetic
    let doubled = distance * 2.0;  // 2000 m

    // Automatic unit conversion
    let km = distance.convert::<Kilometer>();  // 1 km

    // Add values with different units
    let total = distance + Value::<Length, Centimeter>::new(100.0);  // 1001 m

    // These would cause COMPILE ERRORS:
    // let invalid = distance + time;  // ❌ Cannot add Length and Time!
    // let invalid = distance.convert::<Second>();  // ❌ Cannot convert Length to Time!
}
```

## Core Concepts

### Dimensions

Physical quantities are characterized by their dimensional signature using the 7 SI base dimensions:

```
[L, T, M, I, Θ, J, N]
 │  │  │  │  │  │  └─ Amount of substance (mole)
 │  │  │  │  │  └──── Luminous intensity (candela)
 │  │  │  │  └─────── Thermodynamic temperature (kelvin)
 │  │  │  └────────── Electric current (ampere)
 │  │  └───────────── Mass (kilogram)
 │  └──────────────── Time (second)
 └─────────────────── Length (meter)
```

Examples:
- Velocity: `[1, -1, 0, 0, 0, 0, 0]` (L¹T⁻¹)
- Force: `[1, -2, 1, 0, 0, 0, 0]` (MLT⁻²)
- Energy: `[2, -2, 1, 0, 0, 0, 0]` (ML²T⁻²)

### Quantities

A quantity represents a type of physical measurement (e.g., `Length`, `Mass`, `Time`).

```rust
use units::quantity::Quantity;
use units::dimension::Dimension;

#[derive(Debug, Clone, Copy)]
pub struct Length;

impl Quantity for Length {
    const DIMENSION: Dimension = Dimension::length();
    const NAME: &'static str = "Length";
}
```

### Units

Units are specific measures of quantities (e.g., `Meter`, `Kilometer`, `Mile`).

```rust
use units::unit::Unit;

#[derive(Debug, Clone, Copy)]
pub struct Kilometer;

impl Unit for Kilometer {
    type BaseQuantity = Length;
    const SYMBOL: &'static str = "km";
    const TO_SI: f64 = 1000.0;  // 1 km = 1000 m
    const OFFSET: f64 = 0.0;     // For affine conversions (e.g., temperature)
}
```

### Values

Values combine a numeric value with its unit and quantity:

```rust
let distance = Value::<Length, Meter>::new(100.0);
let time = Value::<Time, Second>::new(10.0);
```

## Available Quantities and Units

### Length
- SI: `Meter`, `Kilometer`, `Centimeter`, `Millimeter`, `Micrometer`, `Nanometer`
- Imperial: `Inch`, `Foot`, `Yard`, `Mile`
- Astronomical: `AstronomicalUnit`, `LightYear`, `Parsec`

### Time
- `Second`, `Minute`, `Hour`, `Day`, `Week`, `Year`
- `Millisecond`, `Microsecond`, `Nanosecond`

### Mass
- SI: `Kilogram`, `Gram`, `Milligram`, `Microgram`, `Tonne`
- Imperial: `Pound`, `Ounce`, `Stone`
- Astronomical: `SolarMass`, `EarthMass`

### Temperature ⭐ Special Handling

Temperature is handled specially with two distinct types to solve the "Celsius Problem":

**AbsoluteTemperature** - An absolute temperature value (e.g., "20°C")
- Units: `Kelvin`, `Celsius`, `Fahrenheit`
- Cannot be added together (prevents meaningless operations)
- Can be subtracted to get a difference

**TemperatureDifference** - A temperature change (e.g., "+10°C change")
- Units: `KelvinDelta`, `CelsiusDelta`, `FahrenheitDelta`
- Can be added together
- Can be added to absolute temperatures

#### The Celsius Problem

The problem: If you naively add `10°C + 20°C` using SI conversion, you get:
```
(10 + 273.15) + (20 + 273.15) = 566.3 K ≠ 30°C (303.15 K)
```

Our solution: Two separate types with different arithmetic rules:

```rust
use units::prelude::*;

// ❌ This is a COMPILE ERROR (prevents the problem):
// let temp1 = Value::<AbsoluteTemperature, Celsius>::new(10.0);
// let temp2 = Value::<AbsoluteTemperature, Celsius>::new(20.0);
// let invalid = temp1 + temp2;  // ERROR: cannot add absolute temperatures

// ✓ Instead, use temperature differences:
let change1 = Value::<TemperatureDifference, CelsiusDelta>::new(10.0);
let change2 = Value::<TemperatureDifference, CelsiusDelta>::new(20.0);
let total_change = change1 + change2;  // 30°C change ✓

let start = Value::<AbsoluteTemperature, Celsius>::new(0.0);
let result = start + total_change;  // 30°C ✓
```

**Allowed Temperature Operations:**
- ✓ `AbsoluteTemperature - AbsoluteTemperature = TemperatureDifference`
- ✓ `AbsoluteTemperature + TemperatureDifference = AbsoluteTemperature`
- ✓ `AbsoluteTemperature - TemperatureDifference = AbsoluteTemperature`
- ✓ `TemperatureDifference + TemperatureDifference = TemperatureDifference`
- ✗ `AbsoluteTemperature + AbsoluteTemperature = COMPILE ERROR`

See `examples/temperature_demo.rs` for a complete demonstration.

## Arithmetic Operations

### Addition and Subtraction

Only values with the same dimension can be added or subtracted:

```rust
let m1 = Value::<Length, Meter>::new(100.0);
let m2 = Value::<Length, Meter>::new(50.0);
let sum = m1 + m2;  // 150 m

// Works with different units of same quantity
let km = Value::<Length, Kilometer>::new(1.0);
let total = m1 + km;  // 1100 m (automatic conversion)
```

### Scalar Multiplication and Division

```rust
let distance = Value::<Length, Meter>::new(100.0);
let doubled = distance * 2.0;  // 200 m
let halved = distance / 2.0;   // 50 m
let scaled = 3.0 * distance;   // 300 m
```

### Division of Same Quantities

Dividing two values of the same quantity yields a dimensionless ratio:

```rust
let a = Value::<Length, Meter>::new(1000.0);
let b = Value::<Length, Meter>::new(500.0);
let ratio: f64 = a / b;  // 2.0 (dimensionless)
```

## Type Safety

The type system prevents invalid operations at compile time:

```rust
let distance = Value::<Length, Meter>::new(100.0);
let time = Value::<Time, Second>::new(10.0);

// ❌ Compile error: cannot add Length and Time
// let invalid = distance + time;

// ❌ Compile error: cannot convert Length to Time
// let invalid = distance.convert::<Second>();
```

## Unit Conversion

Convert between units of the same quantity:

```rust
let meters = Value::<Length, Meter>::new(1000.0);
let kilometers = meters.convert::<Kilometer>();
assert_eq!(kilometers.get(), 1.0);

let inches = meters.convert::<Inch>();
assert_eq!(inches.get(), 39370.07874015748);
```

## Defining Custom Units with Macros

The library provides powerful macros for easy definition of new quantities and units:

```rust
use units::{define_quantity_with_units};
use units::dimension::Dimension;

// Define a new quantity with its units in one go
define_quantity_with_units! {
    quantity: Velocity,
    dimension: Dimension::VELOCITY,  // LT⁻¹
    base_unit: MeterPerSecond = 1.0,
    units: {
        KilometerPerHour = 0.277778,
        MilesPerHour = 0.44704,
        SpeedOfLight = 299_792_458.0,
    }
}

// Now use it!
let c = Value::<Velocity, SpeedOfLight>::new(1.0);
let mps = c.convert::<MeterPerSecond>();
// c = 299,792,458 m/s
```

### Available Predefined Dimensions

- **Base**: `Dimension::length()`, `Dimension::time()`, `Dimension::mass()`, `Dimension::temperature()`
- **Derived**: `Dimension::VELOCITY`, `Dimension::ACCELERATION`, `Dimension::FORCE`, `Dimension::ENERGY`, `Dimension::POWER`, `Dimension::PRESSURE`, `Dimension::AREA`, `Dimension::VOLUME`
- **Special**: `Dimension::DIMENSIONLESS` (for angles, ratios, etc.)

### Custom Dimensions

Build complex dimensions using operations:

```rust
// Momentum = Mass × Velocity = M × LT⁻¹ = MLT⁻¹
define_quantity!(
    Momentum,
    Dimension::mass().multiply(Dimension::VELOCITY)
);

// Jerk = Acceleration / Time = LT⁻² / T = LT⁻³
define_quantity!(
    Jerk,
    Dimension::ACCELERATION.divide(Dimension::time())
);
```

See [`MIGRATION.md`](MIGRATION.md) for more examples and a complete migration guide from old systems.

## Prefixes (In Development)

SI prefixes can be applied to units:

```rust
use units::prefix::{Kilo, Milli, Prefixed};

type Kilometer = Prefixed<Kilo, Meter>;
type Millimeter = Prefixed<Milli, Meter>;
```

## Running Examples

```bash
# Basic usage example
cargo run --example basic_usage

# Type safety demonstration
cargo run --example type_safety_demo

# Temperature handling (solves the "Celsius Problem")
cargo run --example temperature_demo

# Macro system and custom unit definitions
cargo run --example macro_usage
```

## Running Tests

```bash
cargo test
```

## Architecture

```
src/
├── lib.rs              # Main library entry
├── dimension.rs        # Dimensional analysis
├── quantity.rs         # Quantity trait + marker traits
├── unit.rs            # Unit trait
├── value.rs           # Value type with units
├── prefix.rs          # SI prefix system
├── operators.rs       # Arithmetic operations
└── quantities/        # Predefined quantities
    ├── length.rs
    ├── time.rs
    ├── mass.rs
    └── temperature.rs  # Special temperature handling
```

## Roadmap

- [x] Core type-safe unit system
- [x] Basic arithmetic operations
- [x] SI and Imperial units for Length, Time, Mass
- [x] **Temperature handling (absolute vs. relative)** ⭐ **SOLVED!**
- [x] **Macros for easy unit definition** ⭐ **DONE!**
- [x] Astronomical quantities (Distance, Mass, Luminosity, Angle, etc.)
- [ ] Compound quantities via type-level multiplication/division
- [ ] More physical quantities (Current, Voltage, Resistance, etc.)
- [ ] Improved prefix system integration
- [ ] no_std support
- [ ] Serde support for serialization

## Contributing

Contributions are welcome! Please see the [issue.md](idea.md) for design considerations.

## License

Licensed under either of:

- MIT License
- Apache License, Version 2.0

at your option.

## Inspiration

This project was inspired by the need for compile-time dimensional analysis in Rust, similar to:
- [uom](https://github.com/iliekturtles/uom)
- [dimensioned](https://github.com/paholg/dimensioned)

The key differentiator is the focus on simplicity, compile-time checking, and special handling of temperature units.
