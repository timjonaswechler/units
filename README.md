# Physics Units

A type-safe, high-performance unit system with dimensional analysis for scientific computing.

## 🌟 Features

- **🚀 Hub-and-Spoke Conversions**: O(n) complexity instead of O(n²)
- **🛡️ Compile-Time Dimensional Safety**: Prevents unit mixing errors at compile time
- **🏭 Macro-Generated Boilerplate**: Minimal code required for adding new units
- **🎯 Astronomy-Focused Design**: Built specifically for stellar simulation with astronomical units
- **🔧 Variadic Syntax**: Experimental support for flexible multi-unit syntax
- **⚡ High Performance**: Zero-cost abstractions with minimal runtime overhead

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
physics-units = "0.1.0"
```

### Basic Usage

```rust
use physics_units::*;

// Create quantities with specific units
let distance = Distance::<AstronomicalUnit>::new(1.5);
let mass = Mass::<SolarMass>::new(0.7);
let time = Time::<Gigayear>::new(6.0);

// Convert between units using hub-and-spoke conversion
let distance_meters = distance.convert_to::<Meter>();
let mass_earth = mass.convert_to::<EarthMass>();

// Type-safe arithmetic operations
let total_distance = distance + Distance::<AstronomicalUnit>::new(0.5);
let velocity = distance / time;

// Display with proper symbols
println!("Distance: {}", distance); // "1.5 AU"
println!("Mass: {}", mass);         // "0.7 M☉"
println!("Velocity: {}", velocity); // "0.25 AU/Gyr"
```

### Astronomy-Focused Units

```rust
use physics_units::*;

// Stellar distances
let proxima_distance = Distance::<Parsec>::new(1.3);
let galactic_center = Distance::<KiloParsec>::new(8.2);

// Stellar masses and radii  
let star_mass = Mass::<SolarMass>::new(1.4);
let planet_mass = Mass::<EarthMass>::new(0.8);
let star_radius = Distance::<SunRadius>::new(1.2);

// Stellar evolution timescales
let main_sequence_lifetime = Time::<Gigayear>::new(10.0);
let stellar_luminosity = Power::<SolarLuminosity>::new(2.5);
```

### Variadic Multi-Unit Syntax (Experimental)

```rust
use physics_units::variadic::*;

// Intuitive multi-unit syntax
let velocity = Velocity::<Meter, Second>::new(10.0);           // 10 m/s
let acceleration = Acceleration::<Meter, Second>::new(9.81);   // 9.81 m/s²  
let force = Force::<Kilogram, Meter, Second>::new(98.1);       // 98.1 kg⋅m/s²
let energy = Energy::<Kilogram, Meter, Second>::new(500.0);    // 500 kg⋅m²/s²

// Works with prefixed units too
let speed_kmh = Velocity::<Prefixed<Kilo, Meter>, Hour>::new(100.0); // 100 km/h
```

## 🏗️ Architecture

### Hub-and-Spoke Conversions

Traditional unit systems require O(n²) conversion implementations. This system uses SI units as a conversion hub, reducing complexity to O(n):

- **Traditional**: 6 units × 6 units = 36 conversion functions  
- **Hub-and-spoke**: 6 units × 2 conversions each = 12 conversion functions
- **Adding units**: O(1) instead of O(n) new conversions required

### Compile-Time Dimensional Safety

The system tracks physical dimensions at compile time, preventing unit mixing errors:

```rust,compile_fail
let distance = Distance::<Meter>::new(100.0);
let mass = Mass::<Kilogram>::new(5.0);
let invalid = distance + mass; // Compile error!
```

### Prefix System

Avoid combinatorial explosion with a generic prefix system:

```rust
use physics_units::*;

let distance = Distance::<Prefixed<Kilo, Meter>>::new(5.0); // 5 km
let mass = Mass::<Prefixed<Mega, Gram>>::new(2.0);          // 2 Mg  
let time = Time::<Prefixed<Micro, Second>>::new(100.0);     // 100 μs
```

## 📚 Available Units

### Distance/Length
- **Meter** (`m`) - SI base unit
- **AstronomicalUnit** (`AU`) - Earth-Sun distance  
- **LightYear** (`ly`) - Distance light travels in one year
- **Parsec** (`pc`) - Parallax arcsecond
- **EarthRadius** (`R⊕`) - Mean radius of Earth
- **SunRadius** (`R☉`) - Mean radius of the Sun

### Mass
- **Kilogram** (`kg`) - SI base unit
- **Gram** (`g`) - Common metric unit
- **EarthMass** (`M⊕`) - Mass of Earth
- **SolarMass** (`M☉`) - Mass of the Sun

### Time  
- **Second** (`s`) - SI base unit
- **Hour** (`h`), **Day** (`d`), **Year** (`yr`)
- **Megayear** (`Myr`), **Gigayear** (`Gyr`) - Stellar evolution timescales

### Power/Luminosity
- **Watt** (`W`) - SI base unit
- **SolarLuminosity** (`L☉`) - Luminosity of the Sun

### Temperature
- **Kelvin** (`K`) - SI base unit
- **Celsius** (`°C`), **Fahrenheit** (`°F`)

## 🔬 Examples

See the `examples/` directory for comprehensive usage examples:

- `basic_usage.rs` - Fundamental operations and conversions
- `astronomical_calculations.rs` - Stellar system modeling  
- `variadic_syntax.rs` - Multi-unit syntax demonstrations

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.