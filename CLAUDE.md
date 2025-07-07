# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 📍 PROJECT STATUS

### Current Phase: **CORE IMPLEMENTATION - PHASE 2** 🚀

**Status**: Foundation complete! Core dimensional system working perfectly.

**Current Task**: Task 18G - Implement dimensional arithmetic operations (Mul, Div, Add, Sub)

**Next Action**: Implement automatic dimensional arithmetic for `distance / time → Velocity<(Meter, Per<Second>)>`

## 🎯 COMPLETED DESIGN PHASE (Tasks 18A-18E)

### ✅ Task 18A: Dimensional Analysis System
**Core Innovation**: Trait-based dimensional composition with intuitive syntax

**Key Design**:
- `Quantity<U, V>` where `U: DimensionExtractor`
- Compositional syntax: `Velocity<(Meter, Per<Second>)>`
- Complex units: `StefanBoltzmann<(Watt, Per<Exponent<Meter,2>>, Per<Exponent<Kelvin,4>>)>`
- Automatic arithmetic: `distance / time → Quantity<(Meter, Per<Second>)>`
```rust
// Core trait for dimensional extraction
trait DimensionExtractor {
    const L: i8 = 0;      // Length (meters)
    const M: i8 = 0;      // Mass (kilograms)
    const T: i8 = 0;      // Time (seconds)
    const THETA: i8 = 0;  // Temperature (kelvin)
    const I: i8 = 0;      // Current (amperes)
    const J: i8 = 0;      // Luminous Intensity (candela)
    const N: i8 = 0;      // Amount of Substance (moles)
}

// Compositional operators
struct Per<U>(PhantomData<U>);  // Inverts dimensions
struct Exponent<U, const N: i8>(PhantomData<U>);  // Raises to power N

// New quantity type
pub struct Quantity<U, V = f64> where U: DimensionExtractor {
    value: V,
    _phantom: PhantomData<U>,
}


// Base units
impl DimensionExtractor for Meter {
    const L: i8 = 1;
}

impl DimensionExtractor for Second {
    const T: i8 = 1;
}

impl DimensionExtractor for Kelvin {
    const THETA: i8 = 1;
}

impl DimensionExtractor for Watt {
    const L: i8 = 2;   // kg⋅m²⋅s⁻³
    const M: i8 = 1;
    const T: i8 = -3;
}

Compositional Operators

// Per<U> inverts all dimensions of U
struct Per<U>(PhantomData<U>);
impl<U: DimensionExtractor> DimensionExtractor for Per<U> {
    const L: i8 = -U::L;
    const M: i8 = -U::M;
    const T: i8 = -U::T;
    const THETA: i8 = -U::THETA;
    const I: i8 = -U::I;
    const J: i8 = -U::J;
    const N: i8 = -U::N;
}

// Exponent<U, N> raises U to power N
struct Exponent<U, const N: i8>(PhantomData<U>);
impl<U: DimensionExtractor, const N: i8> DimensionExtractor for Exponen<U, N> {
    const L: i8 = U::L * N;
    const M: i8 = U::M * N;
    const T: i8 = U::T * N;
    const THETA: i8 = U::THETA * N;
    const I: i8 = U::I * N;
    const J: i8 = U::J * N;
    const N: i8 = U::N * N;
}

// Tuple composition (multiplication)
impl<U1: DimensionExtractor, U2: DimensionExtractor> DimensionExtractorfor (U1, U2) {
    const L: i8 = U1::L + U2::L;
    const M: i8 = U1::M + U2::M;
    const T: i8 = U1::T + U2::T;
    const THETA: i8 = U1::THETA + U2::THETA;
    const I: i8 = U1::I + U2::I;
    const J: i8 = U1::J + U2::J;
    const N: i8 = U1::N + U2::N;
}

// Extended for 3-tuples, 4-tuples, etc.
impl<U1, U2, U3> DimensionExtractor for (U1, U2, U3)
where
    U1: DimensionExtractor,
    U2: DimensionExtractor,
    U3: DimensionExtractor,
{
    const L: i8 = U1::L + U2::L + U3::L;
    const M: i8 = U1::M + U2::M + U3::M;
    const T: i8 = U1::T + U2::T + U3::T;
    const THETA: i8 = U1::THETA + U2::THETA + U3::THETA;
    const I: i8 = U1::I + U2::I + U3::I;
    const J: i8 = U1::J + U2::J + U3::J;
    const N: i8 = U1::N + U2::N + U3::N;
}

New Quantity Definition

pub struct Quantity<U, V = f64>
where
    U: DimensionExtractor,
{
    value: V,
    _phantom: PhantomData<U>,
}

// Type aliases using dimensional extraction
pub type Distance<U, V = f64> = Quantity<U, V>;
pub type Velocity<U, V = f64> = Quantity<U, V>;
pub type StefanBoltzmann<U, V = f64> = Quantity<U, V>;

Automatic Dimensional Arithmetic

// Multiplication automatically creates correct unit composition
impl<U1, U2, V> Mul<Quantity<U2, V>> for Quantity<U1, V>
where
    U1: DimensionExtractor,
    U2: DimensionExtractor,
    V: Mul<V, Output = V>,
{
    type Output = Quantity<(U1, U2), V>;

    fn mul(self, rhs: Quantity<U2, V>) -> Self::Output {
        Quantity {
            value: self.value * rhs.value,
            _phantom: PhantomData,
        }
    }
}

// Division automatically creates correct unit composition
impl<U1, U2, V> Div<Quantity<U2, V>> for Quantity<U1, V>
where
    U1: DimensionExtractor,
    U2: DimensionExtractor,
    V: Div<V, Output = V>,
{
    type Output = Quantity<(U1, Per<U2>), V>;

    fn div(self, rhs: Quantity<U2, V>) -> Self::Output {
        Quantity {
            value: self.value / rhs.value,
            _phantom: PhantomData,
        }
    }
}
```

### ✅ Task 18B: Feature Flag System
```toml
[features]
default = ["f64", "precision-6"]

# Value type selection (mutually exclusive)
f32 = []
f64 = []
f128 = []

# Precision configuration
precision-3 = []
precision-6 = []
precision-9 = []
precision-12 = []

# Display style
compact = []
verbose = []
scientific = []

[dependencies]
static_assertions = "1.0"  # For compile-time validation
```

**Implementation with Validation:**
```rust
// Mutually exclusive value type validation
#[cfg(all(feature = "f32", feature = "f64"))]
compile_error!("Cannot enable both f32 and f64 features");

#[cfg(all(feature = "f32", feature = "f128"))]
compile_error!("Cannot enable both f32 and f128 features");

#[cfg(all(feature = "f64", feature = "f128"))]
compile_error!("Cannot enable both f64 and f128 features");

#[cfg(not(any(feature = "f32", feature = "f64", feature = "f128")))]
compile_error!("Must enable one of: f32, f64, f128");

// Value type selection
#[cfg(feature = "f64")]
pub type DefaultFloat = f64;
#[cfg(feature = "f32")]
pub type DefaultFloat = f32;
#[cfg(feature = "f128")]
pub type DefaultFloat = f128;

// Precision configuration
#[cfg(feature = "precision-3")]
pub const DEFAULT_PRECISION: usize = 3;
#[cfg(feature = "precision-6")]
pub const DEFAULT_PRECISION: usize = 6;
#[cfg(feature = "precision-9")]
pub const DEFAULT_PRECISION: usize = 9;
#[cfg(feature = "precision-12")]
pub const DEFAULT_PRECISION: usize = 12;

// Integration with Quantity type
pub struct Quantity<U, V = DefaultFloat>
where
    U: DimensionExtractor,
    V: Float + Copy + Debug,
{
    value: V,
    _phantom: PhantomData<U>,
}
```

**Domain-Specific Feature Combinations:**
```toml
# Game Development Profile
game = ["f32", "precision-3", "compact"]

# Scientific Computing Profile
science = ["f64", "precision-9", "scientific"]

# High-Precision Physics Profile
physics = ["f128", "precision-12", "verbose"]
```

### ✅ Task 18C: Unit Composition System
**Complete flexibility** with dimensional operators:
- `Per<U>` - inverts dimensions (e.g., `Per<Second>` = T⁻¹)
- `Exponent<U, N>` - raises to power N (e.g., `Exponent<Meter, 2>` = L²)
- Tuples - multiply dimensions (e.g., `(Meter, Second)` = L¹T¹)
- Simplification rules for nested compositions
**Additional Implementations:**
```rust
// Extended tuple support for complex compositions
impl<U1, U2, U3, U4> DimensionExtractor for (U1, U2, U3, U4)
where
    U1: DimensionExtractor,
    U2: DimensionExtractor,
    U3: DimensionExtractor,
    U4: DimensionExtractor,
{
    const L: i8 = U1::L + U2::L + U3::L + U4::L;
    const M: i8 = U1::M + U2::M + U3::M + U4::M;
    const T: i8 = U1::T + U2::T + U3::T + U4::T;
    const THETA: i8 = U1::THETA + U2::THETA + U3::THETA + U4::THETA;
    const I: i8 = U1::I + U2::I + U3::I + U4::I;
    const J: i8 = U1::J + U2::J + U3::J + U4::J;
    const N: i8 = U1::N + U2::N + U3::N + U4::N;
}

// Unit simplification rules
impl<U: DimensionExtractor> DimensionExtractor for Per<Per<U>> {
    // Per<Per<U>> = U (double inversion)
    const L: i8 = U::L;
    const M: i8 = U::M;
    const T: i8 = U::T;
    const THETA: i8 = U::THETA;
    const I: i8 = U::I;
    const J: i8 = U::J;
    const N: i8 = U::N;
}

impl<U: DimensionExtractor> DimensionExtractor for Exponent<U, 1> {
    // Exponent<U, 1> = U (power of 1)
    const L: i8 = U::L;
    const M: i8 = U::M;
    const T: i8 = U::T;
    const THETA: i8 = U::THETA;
    const I: i8 = U::I;
    const J: i8 = U::J;
    const N: i8 = U::N;
}

impl<U: DimensionExtractor> DimensionExtractor for Exponent<U, 0> {
    // Exponent<U, 0> = dimensionless (power of 0)
    const L: i8 = 0;
    const M: i8 = 0;
    const T: i8 = 0;
    const THETA: i8 = 0;
    const I: i8 = 0;
    const J: i8 = 0;
    const N: i8 = 0;
}
```

**Complex Composition Examples:**
```rust
// All these are now supported:
type Force = (Kilogram, Meter, Per<Exponent<Second, 2>>);  // kg⋅m⋅s⁻²
type Pressure = (Newton, Per<Exponent<Meter, 2>>);         // N⋅m⁻²
type StefanBoltzmann = (Watt, Per<Exponent<Meter, 2>>, Per<Exponent<Kelvin, 4>>);  // W⋅m⁻²⋅K⁻⁴
type ComplexConstant = (Joule, Second, Per<Kilogram>, Per<Exponent<Meter, 3>>);    // J⋅s⋅kg⁻¹⋅m⁻³
```

**Note**: This task was largely completed within Task 18A. The core composition system using `Per<U>`, `Exponent<U, N>`, and tuples provides complete flexibility for any unit combination.


### ✅ Task 18D: Physical Constants System
**Perfect syntax** matching physics notation:

```rust
const PLANCK_CONSTANT: Quantity<(Joule, Second)> = Quantity::new(6.62607015e-34);
const STEFAN_BOLTZMANN: Quantity<(Watt, Per<Exponent<Meter, 2>>, Per<Exponent<Kelvin, 4>>)> =
    Quantity::new(5.670374419e-8);
const GRAVITATIONAL_CONSTANT: Quantity<(Exponent<Meter, 3>, Per<Kilogram>, Per<Exponent<Second, 2>>)> =
    Quantity::new(6.67430e-11);

// Usage in physics calculations
let energy_photon = PLANCK_CONSTANT * frequency;  // E = hν
let black_body_flux = STEFAN_BOLTZMANN * temperature.powi(4);  // σT⁴
```

### ✅ Task 18E: Migration Strategy
**Fresh start approach** for clean implementation:

```bash
# Phase 1: Preserve existing code
mv src src_old
mkdir src

# Phase 2: Implement new architecture with proper module structure
```

## 🏗️ NEW ARCHITECTURE DESIGN

### Core System
```rust
// Foundation trait for dimensional extraction
trait DimensionExtractor {
    const L: i8 = 0;      // Length (meters)
    const M: i8 = 0;      // Mass (kilograms)
    const T: i8 = 0;      // Time (seconds)
    const THETA: i8 = 0;  // Temperature (kelvin)
    const I: i8 = 0;      // Current (amperes)
    const J: i8 = 0;      // Luminous Intensity (candela)
    const N: i8 = 0;      // Amount of Substance (moles)
}

// Core quantity type
pub struct Quantity<U, V = DefaultFloat>
where
    U: DimensionExtractor,
    V: Float + Copy + Debug,
{
    value: V,
    _phantom: PhantomData<U>,
}

// Compositional operators
struct Per<U>(PhantomData<U>);              // Inverts dimensions
struct Exponent<U, const N: i8>(PhantomData<U>); // Raises to power N

// Automatic dimensional arithmetic
impl<U1, U2, V> Mul<Quantity<U2, V>> for Quantity<U1, V> {
    type Output = Quantity<(U1, U2), V>;    // Multiplication composes
}

impl<U1, U2, V> Div<Quantity<U2, V>> for Quantity<U1, V> {
    type Output = Quantity<(U1, Per<U2>), V>; // Division creates ratios
}
```

### Module Structure
```
src/
├── lib.rs                  # Public API and feature flag configuration
├── core/
│   ├── mod.rs             # Core exports
│   ├── quantity.rs        # Quantity<U, V> implementation
│   ├── dimension.rs       # DimensionExtractor trait system
│   └── composition.rs     # Per<U>, Exponent<U, N> operators
├── units/
│   ├── mod.rs             # Unit definitions
│   ├── base.rs            # SI base units (Meter, Kilogram, etc.)
│   ├── derived.rs         # Derived units (Newton, Joule, etc.)
│   └── prefixes.rs        # Metric prefixes (Kilo, Mega, etc.)
├── constants/
│   ├── mod.rs             # Physical constants exports
│   ├── fundamental.rs     # Planck, c, e, etc.
│   ├── thermodynamic.rs   # Boltzmann, Stefan-Boltzmann, etc.
│   ├── electromagnetic.rs # μ₀, ε₀, fine structure, etc.
│   ├── gravitational.rs   # G, g, etc.
│   ├── atomic.rs          # Bohr radius, Rydberg, etc.
│   └── astronomical.rs    # AU, parsec, solar mass, etc.
├── arithmetic/
│   ├── mod.rs             # Arithmetic operations
│   ├── ops.rs             # Add, Sub, Mul, Div implementations
│   └── conversion.rs      # Unit conversion system
└── formatting/
    ├── mod.rs             # Display and formatting
    ├── display.rs         # Basic Display implementation
    ├── scientific.rs      # Scientific notation
    └── precision.rs       # Precision control via features
```

## 📋 IMPLEMENTATION ROADMAP (Tasks 18F-18M)

### Phase 2: Core Implementation (HIGH PRIORITY)
- **✅ 18F**: Implement `DimensionExtractor` trait and `Quantity<U, V>` type ✅ COMPLETED
- **✅ 18G**: Implement dimensional arithmetic operations (Mul, Div, Add, Sub) - COMPLETED
- **18H**: Implement feature flag system and compile-time configuration

### Phase 3: System Components (MEDIUM PRIORITY)
- **✅ 18I**: Implement all SI base and derived units
- **✅ 18J**:  Implement comprehensive physical constants with new syntax
- **18K**: Implement conversion and formatting systems
  1. ✅ Unicode Superscripts: K⁴, s³ - genau wie gewünscht
  2. ✅ SI Basis-Einheiten: kg/(s³⋅K⁴) - korrekte dimensionale Analyse
  3. ✅ Mathematische Notation: /() mit proper Klammern
  4. ✅ Feature Flag System: compact format funktioniert
  5. noch Offen:
    - Komplexe Einheiten wie W/(m²⋅K⁴) erkennung
    - Automatische Unit-Optimierung
    - Verbose/Scientific Modi
- **18L**: Advanced formatting (scientific/verbose modes)
- **18M**: Testing


### Phase 5: Future Development (LOW PRIORITY - UNBLOCKED)
- **19**: Comprehensive documentation and examples
- **20**: Serialization support (Serde)
- **21**: Performance optimization and benchmarks
- **22**: Example projects and tutorials

## 🔧 DEVELOPMENT COMMANDS

### Build and Check
```bash
cargo check                    # Quick compilation check
cargo build                    # Full build
cargo build --release          # Optimized build
```

### Testing
```bash
cargo test                     # Run all tests
cargo test --lib               # Library tests only
cargo test --doc               # Documentation tests only
```

### Feature Testing
```bash
cargo check --features "f32,precision-3,compact"     # Game profile
cargo check --features "f64,precision-9,scientific"  # Science profile
cargo check --features "f128,precision-12,verbose"   # Physics profile
```

### Documentation
```bash
cargo doc                      # Generate documentation
cargo doc --open               # Generate and open docs
```

## 🎯 PROJECT GOALS

### Core Mission
**Type-safe physics units with dimensional analysis** for scientific computing and astronomical calculations. This is a pure data library focused on preventing unit conversion errors at compile time.

### Key Features
- **Intuitive Syntax**: `Velocity<(Meter, Per<Second>)>` matches physics notation
- **Compile-Time Safety**: Invalid operations caught during compilation
- **Zero-Cost Abstractions**: Runtime performance identical to raw floating point
- **Complete Flexibility**: Any unit combination expressible
- **Domain Optimization**: Configurable precision and value types
- **Physics Accuracy**: All constants with proper dimensional signatures

### Success Criteria
1. **Stefan-Boltzmann constant** properly represented and usable
2. **Planck constant** with correct dimensional type (action units)
3. **Automatic dimensional arithmetic** working flawlessly
4. **Compile-time error detection** for dimensional mismatches
5. **Feature flag system** enabling domain-specific optimization
6. **Complete test coverage** ensuring correctness

## 📚 REFERENCE INFORMATION

### Historical Context
- **Tasks 1-17**: Implemented flawed tuple-based system (preserved in `src_old/`)
- **Critical Issues Identified**: Tuple composition limitations, missing dimensional foundation
- **Architecture Decision**: Fresh start implementation for clean design

### Design Principles
1. **Physics-First**: Build on proper SI dimensional analysis
2. **Type Safety**: Leverage Rust's type system for compile-time checking
3. **Zero-Cost**: All abstractions resolved at compile time
4. **Ergonomics**: Syntax should match mathematical notation
5. **Flexibility**: Support any unit combination needed in physics

### Validation Strategy
- Preserve old implementation for reference and comparison
- Test dimensional correctness of all physical constants
- Verify performance characteristics match raw floating point
- Ensure API ergonomics meet scientific computing standards
