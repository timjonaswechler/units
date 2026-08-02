# Migration Guide: From Old Units System to New

This guide shows how to migrate your old unit definitions to the new macro-based system.

## Quick Comparison

### Old System
```rust
#![allow(non_snake_case)]
use crate::{define_quantity, define_units};

const METERS_PER_AU: f64 = 1.495978707e11;

define_quantity!(Distance); // Length

define_units! {
    base_unit: Meter = 1.0,
    units: {
        AstronomicalUnit = METERS_PER_AU,
    }
}
```

### New System
```rust
use units::{define_quantity, define_units};
use units::dimension::Dimension;

const METERS_PER_AU: f64 = 1.495978707e11;

define_quantity!(Distance, Dimension::length());

define_units! {
    quantity: Distance,
    base_unit: Meter = 1.0,
    units: {
        AstronomicalUnit = METERS_PER_AU,
    }
}
```

## Key Differences

### 1. Dimension Must Be Specified

**Old:**
```rust
define_quantity!(Distance); // Comment indicated dimension
```

**New:**
```rust
define_quantity!(Distance, Dimension::length());  // Dimension is required
```

Available dimensions:
- `Dimension::length()` - L
- `Dimension::time()` - T
- `Dimension::mass()` - M
- `Dimension::temperature()` - Θ
- `Dimension::VELOCITY` - LT⁻¹
- `Dimension::ACCELERATION` - LT⁻²
- `Dimension::FORCE` - MLT⁻²
- `Dimension::ENERGY` - ML²T⁻²
- `Dimension::POWER` - ML²T⁻³
- `Dimension::PRESSURE` - ML⁻¹T⁻²
- `Dimension::AREA` - L²
- `Dimension::VOLUME` - L³
- `Dimension::DIMENSIONLESS` - For angles, ratios, etc.

### 2. Units Must Reference Quantity

**Old:**
```rust
define_units! {
    base_unit: Meter = 1.0,
    units: { ... }
}
```

**New:**
```rust
define_units! {
    quantity: Distance,  // <-- Add this
    base_unit: Meter = 1.0,
    units: { ... }
}
```

### 3. Combined Macro Available

For simpler definitions, use the combined macro:

```rust
define_quantity_with_units! {
    quantity: Velocity,
    dimension: Dimension::VELOCITY,
    base_unit: MeterPerSecond = 1.0,
    units: {
        KilometerPerHour = 0.277778,
    }
}
```

## Migration Steps

### Step 1: Update Imports

**Old:**
```rust
use crate::{define_quantity, define_units};
```

**New:**
```rust
use units::{define_quantity, define_units};
use units::dimension::Dimension;
```

### Step 2: Add Dimensions to Quantities

**Before:**
```rust
define_quantity!(Distance);
define_quantity!(Mass);
define_quantity!(Velocity);
```

**After:**
```rust
define_quantity!(Distance, Dimension::length());
define_quantity!(Mass, Dimension::mass());
define_quantity!(Velocity, Dimension::VELOCITY);
```

### Step 3: Add Quantity to Units

**Before:**
```rust
define_units! {
    base_unit: Meter = 1.0,
    units: { Kilometer = 1000.0 }
}
```

**After:**
```rust
define_units! {
    quantity: Distance,
    base_unit: Meter = 1.0,
    units: { Kilometer = 1000.0 }
}
```

## Complete Example Migration

### Old: acceleration.rs

```rust
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{define_quantity, define_units};

define_quantity!(Acceleration); // Length/Time²

define_units! {
    base_unit: MeterPerSecondSquared = 1.0,
    units: {
        StandardGravity = 9.80665,
    }
}
```

### New: acceleration.rs

```rust
use units::{define_quantity_with_units};
use units::dimension::Dimension;

const STANDARD_GRAVITY: f64 = 9.80665;

define_quantity_with_units! {
    quantity: Acceleration,
    dimension: Dimension::ACCELERATION,
    base_unit: MeterPerSecondSquared = 1.0,
    units: {
        StandardGravity = STANDARD_GRAVITY,
    }
}
```

## Advanced: Custom Dimensions

For quantities not in the predefined set, build dimensions:

```rust
// Angular Velocity: dimensionless/time = T⁻¹ (if angle is dimensionless)
// Or: angle/time
define_quantity!(
    AngularVelocity,
    Dimension::DIMENSIONLESS.divide(Dimension::time())
);

// Or use predefined FREQUENCY which is also T⁻¹
define_quantity!(AngularVelocity, Dimension::FREQUENCY);
```

```rust
// Momentum: mass × velocity = M × LT⁻¹ = MLT⁻¹
define_quantity!(
    Momentum,
    Dimension::mass().multiply(Dimension::VELOCITY)
);
```

## Ported Quantities

The following quantities have already been ported in `src/quantities/astronomy.rs`:

- ✅ `Distance` (with AU, LightYear, Parsec, etc.)
- ✅ `AstroMass` (with SolarMass, EarthMass)
- ✅ `Luminosity` (with SolarLuminosity)
- ✅ `Angle` (with Degree, Arcsecond, etc.)
- ✅ `Velocity` (with SpeedOfLight)
- ✅ `Acceleration` (with StandardGravity)
- ✅ `Area` (with SquareKilometer)
- ✅ `Pressure` (with Bar, Atmosphere)

See `examples/macro_usage.rs` for usage examples!

## Troubleshooting

### Error: "cannot find `Dimension`"

**Fix:** Add import:
```rust
use units::dimension::Dimension;
```

### Error: "expected `DIMENSION` to be a const fn"

**Fix:** Use predefined dimensions or const operations:
```rust
// ✓ Good
Dimension::VELOCITY
Dimension::length().divide(Dimension::time())

// ✗ Bad
Dimension::from_array([1, -1, 0, 0, 0, 0, 0])  // Not const in current implementation
```

### Conflict with existing `Kilometer`

If you have name conflicts (e.g., `Kilometer` in both `Length` and `Distance`):

**Option 1:** Use specific imports
```rust
use units::quantities::length::Kilometer as LengthKilometer;
use units::quantities::astronomy::Kilometer as DistanceKilometer;
```

**Option 2:** Use module prefix
```rust
use units::quantities::{length, astronomy};

let l = Value::<length::Length, length::Kilometer>::new(5.0);
let d = Value::<astronomy::Distance, astronomy::Kilometer>::new(5.0);
```

## Need Help?

- See `examples/macro_usage.rs` for complete working examples
- Check `src/quantities/astronomy.rs` for ported definitions
- Read the main README.md for general library usage
