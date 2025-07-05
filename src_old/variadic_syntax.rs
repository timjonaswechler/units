//! Variadic multi-unit syntax support for quantities.
//!
//! This module provides support for defining quantities with variable numbers of unit parameters:
//! - `Velocity::<Meter, Second>::new(10.0)` for m/s
//! - `Acceleration::<Meter, Second>::new(9.81)` for m/s²  
//! - `Force::<Kilogram, Meter, Second>::new(98.1)` for kg⋅m/s²
//! - `Energy::<Kilogram, Meter, Meter, Second, Second>::new(500.0)` for kg⋅m²/s²

use crate::core::*;
use crate::variadic::UnitFactor;
use std::marker::PhantomData;

// ================================================================================================
// VARIADIC UNIT TYPES
// ================================================================================================

/// Single unit type.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Unit1<U1>(PhantomData<U1>);

/// Two unit types.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Unit2<U1, U2>(PhantomData<(U1, U2)>);

/// Three unit types.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Unit3<U1, U2, U3>(PhantomData<(U1, U2, U3)>);

/// Four unit types.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Unit4<U1, U2, U3, U4>(PhantomData<(U1, U2, U3, U4)>);

/// Five unit types.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Unit5<U1, U2, U3, U4, U5>(PhantomData<(U1, U2, U3, U4, U5)>);

/// Six unit types.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Unit6<U1, U2, U3, U4, U5, U6>(PhantomData<(U1, U2, U3, U4, U5, U6)>);

/// Seven unit types.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Unit7<U1, U2, U3, U4, U5, U6, U7>(PhantomData<(U1, U2, U3, U4, U5, U6, U7)>);

// ================================================================================================
// UNIT SYMBOL IMPLEMENTATIONS
// ================================================================================================

impl<U1> UnitSymbol for Unit1<U1>
where
    U1: UnitSymbol,
{
    fn symbol() -> &'static str {
        U1::symbol()
    }
}

impl<U1, U2> UnitSymbol for Unit2<U1, U2>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
{
    fn symbol() -> &'static str {
        // For 2-unit types, assume first is numerator, second is denominator
        Box::leak(format!("{}/{}", U1::symbol(), U2::symbol()).into_boxed_str())
    }
}

impl<U1, U2, U3> UnitSymbol for Unit3<U1, U2, U3>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
    U3: UnitSymbol,
{
    fn symbol() -> &'static str {
        // Generic 3-unit format - context-dependent interpretation
        // Could be: Mass⋅Distance/Time² (Force), Mass⋅Distance²/Time² (Energy), Distance³ (Volume)
        Box::leak(format!("{}⋅{}⋅{}", U1::symbol(), U2::symbol(), U3::symbol()).into_boxed_str())
    }
}

impl<U1, U2, U3, U4> UnitSymbol for Unit4<U1, U2, U3, U4>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
    U3: UnitSymbol,
    U4: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}⋅{}⋅{}/{}²", U1::symbol(), U2::symbol(), U3::symbol(), U4::symbol()).into_boxed_str())
    }
}

impl<U1, U2, U3, U4, U5> UnitSymbol for Unit5<U1, U2, U3, U4, U5>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
    U3: UnitSymbol,
    U4: UnitSymbol,
    U5: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}⋅{}⋅{}⋅{}/{}", U1::symbol(), U2::symbol(), U3::symbol(), U4::symbol(), U5::symbol()).into_boxed_str())
    }
}

impl<U1, U2, U3, U4, U5, U6> UnitSymbol for Unit6<U1, U2, U3, U4, U5, U6>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
    U3: UnitSymbol,
    U4: UnitSymbol,
    U5: UnitSymbol,
    U6: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}⋅{}⋅{}⋅{}⋅{}/{}", U1::symbol(), U2::symbol(), U3::symbol(), U4::symbol(), U5::symbol(), U6::symbol()).into_boxed_str())
    }
}

impl<U1, U2, U3, U4, U5, U6, U7> UnitSymbol for Unit7<U1, U2, U3, U4, U5, U6, U7>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
    U3: UnitSymbol,
    U4: UnitSymbol,
    U5: UnitSymbol,
    U6: UnitSymbol,
    U7: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}⋅{}⋅{}⋅{}⋅{}⋅{}/{}", U1::symbol(), U2::symbol(), U3::symbol(), U4::symbol(), U5::symbol(), U6::symbol(), U7::symbol()).into_boxed_str())
    }
}

// ================================================================================================
// SMART QUANTITY TYPE ALIASES
// ================================================================================================

/// Smart velocity type that automatically infers the correct dimensions.
/// 
/// # Examples
/// ```rust
/// let v1 = Velocity::<Meter, Second>::new(10.0);                    // m/s
/// let v2 = Velocity::<Kilometer, Hour>::new(60.0);                  // km/h
/// let v3 = Velocity::<Meter, Prefixed<Milli, Second>>::new(10000.0); // m/ms
/// ```
pub type Velocity<U1, U2> = Quantity<Unit2<U1, U2>, 1, 0, -1, 0, 0, 0, 0>;

/// Smart acceleration type that automatically infers the correct dimensions.
/// 
/// # Examples  
/// ```rust
/// let a1 = Acceleration::<Meter, Second>::new(9.81);                // m/s²
/// let a2 = Acceleration::<Kilometer, Hour>::new(12960.0);           // km/h²
/// ```
pub type Acceleration<U1, U2> = Quantity<Unit2<U1, U2>, 1, 0, -2, 0, 0, 0, 0>;

/// Smart force type that automatically infers the correct dimensions.
/// 
/// # Examples
/// ```rust
/// let f1 = Force::<Kilogram, Meter, Second>::new(98.1);             // kg⋅m/s²
/// let f2 = Force::<Gram, Meter, Second>::new(98100.0);              // g⋅m/s²
/// ```
pub type Force<U1, U2, U3> = Quantity<Unit3<U1, U2, U3>, 1, 1, -2, 0, 0, 0, 0>;

/// Smart energy type that automatically infers the correct dimensions.
/// 
/// # Examples
/// ```rust
/// let e1 = Energy::<Kilogram, Meter, Second>::new(500.0); // kg⋅m²/s² (automatically inferred)
/// let e2 = Energy::<Gram, Meter, Second>::new(500000.0);  // g⋅m²/s²
/// ```
pub type Energy<U1, U2, U3> = Quantity<Unit3<U1, U2, U3>, 2, 1, -2, 0, 0, 0, 0>;

/// Smart power type that automatically infers the correct dimensions.
/// 
/// # Examples
/// ```rust
/// let p1 = Power::<Kilogram, Meter, Second>::new(1000.0); // kg⋅m²/s³ (automatically inferred)
/// ```
pub type Power<U1, U2, U3> = Quantity<Unit3<U1, U2, U3>, 2, 1, -3, 0, 0, 0, 0>;

/// Smart area type that automatically infers the correct dimensions.
/// 
/// # Examples
/// ```rust
/// let a1 = Area::<Meter, Meter>::new(25.0);                         // m²
/// let a2 = Area::<Kilometer, Kilometer>::new(0.000025);             // km²
/// ```
pub type Area<U1, U2> = Quantity<Unit2<U1, U2>, 2, 0, 0, 0, 0, 0, 0>;

/// Smart volume type that automatically infers the correct dimensions.
/// 
/// # Examples
/// ```rust
/// let v1 = Volume::<Meter, Meter, Meter>::new(125.0);               // m³
/// let v2 = Volume::<Centimeter, Centimeter, Centimeter>::new(125000000.0); // cm³
/// ```
pub type Volume<U1, U2, U3> = Quantity<Unit3<U1, U2, U3>, 3, 0, 0, 0, 0, 0, 0>;

// ================================================================================================
// SPECIFIC SYMBOL IMPLEMENTATIONS FOR KNOWN QUANTITIES
// ================================================================================================

/// Specialized symbol implementation for Velocity types.
impl<U1, U2> UnitSymbol for Velocity<U1, U2>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}/{}", U1::symbol(), U2::symbol()).into_boxed_str())
    }
}

/// Specialized symbol implementation for Acceleration types.
impl<U1, U2> UnitSymbol for Acceleration<U1, U2>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}/{}²", U1::symbol(), U2::symbol()).into_boxed_str())
    }
}

/// Specialized symbol implementation for Force types.
impl<U1, U2, U3> UnitSymbol for Force<U1, U2, U3>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
    U3: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}⋅{}/{}²", U1::symbol(), U2::symbol(), U3::symbol()).into_boxed_str())
    }
}

/// Specialized symbol implementation for Energy types.
impl<U1, U2, U3> UnitSymbol for Energy<U1, U2, U3>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
    U3: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}⋅{}²/{}²", U1::symbol(), U2::symbol(), U3::symbol()).into_boxed_str())
    }
}

/// Specialized symbol implementation for Power types.
impl<U1, U2, U3> UnitSymbol for Power<U1, U2, U3>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
    U3: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}⋅{}²/{}³", U1::symbol(), U2::symbol(), U3::symbol()).into_boxed_str())
    }
}

/// Specialized symbol implementation for Area types.
impl<U1, U2> UnitSymbol for Area<U1, U2>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}⋅{}", U1::symbol(), U2::symbol()).into_boxed_str())
    }
}

/// Specialized symbol implementation for Volume types.
impl<U1, U2, U3> UnitSymbol for Volume<U1, U2, U3>
where
    U1: UnitSymbol,
    U2: UnitSymbol,
    U3: UnitSymbol,
{
    fn symbol() -> &'static str {
        Box::leak(format!("{}⋅{}⋅{}", U1::symbol(), U2::symbol(), U3::symbol()).into_boxed_str())
    }
}

// ================================================================================================
// TOSI/FROMSI IMPLEMENTATIONS
// ================================================================================================

// Velocity implementations
impl<U1, U2> ToSI for Velocity<U1, U2>
where
    U1: UnitFactor,
    U2: UnitFactor,
{
    fn to_si(&self) -> f64 {
        // Velocity = Distance / Time, so factor = U1::factor() / U2::factor()
        self.value * U1::factor() / U2::factor()
    }
}

impl<U1, U2> FromSI for Velocity<U1, U2>
where
    U1: UnitFactor,
    U2: UnitFactor,
{
    fn from_si(value: f64) -> Self {
        // Convert from SI (m/s) to target units
        let factor = U1::factor() / U2::factor();
        Self::new(value / factor)
    }
}

// Acceleration implementations
impl<U1, U2> ToSI for Acceleration<U1, U2>
where
    U1: UnitFactor,
    U2: UnitFactor,
{
    fn to_si(&self) -> f64 {
        // Acceleration = Distance / Time², so factor = U1::factor() / (U2::factor())²
        self.value * U1::factor() / (U2::factor() * U2::factor())
    }
}

impl<U1, U2> FromSI for Acceleration<U1, U2>
where
    U1: UnitFactor,
    U2: UnitFactor,
{
    fn from_si(value: f64) -> Self {
        // Convert from SI (m/s²) to target units
        let factor = U1::factor() / (U2::factor() * U2::factor());
        Self::new(value / factor)
    }
}

// Force implementations
impl<U1, U2, U3> ToSI for Force<U1, U2, U3>
where
    U1: UnitFactor, // Mass
    U2: UnitFactor, // Distance 
    U3: UnitFactor, // Time
{
    fn to_si(&self) -> f64 {
        // Force = Mass * Distance / Time², so factor = U1 * U2 / U3²
        self.value * U1::factor() * U2::factor() / (U3::factor() * U3::factor())
    }
}

impl<U1, U2, U3> FromSI for Force<U1, U2, U3>
where
    U1: UnitFactor, // Mass
    U2: UnitFactor, // Distance 
    U3: UnitFactor, // Time
{
    fn from_si(value: f64) -> Self {
        // Convert from SI (kg⋅m/s²) to target units
        let factor = U1::factor() * U2::factor() / (U3::factor() * U3::factor());
        Self::new(value / factor)
    }
}

// Energy implementations
impl<U1, U2, U3> ToSI for Energy<U1, U2, U3>
where
    U1: UnitFactor, // Mass
    U2: UnitFactor, // Distance (appears squared in energy)
    U3: UnitFactor, // Time
{
    fn to_si(&self) -> f64 {
        // Energy = Mass * Distance² / Time², so factor = U1 * U2² / U3²
        self.value * U1::factor() * (U2::factor() * U2::factor()) / (U3::factor() * U3::factor())
    }
}

impl<U1, U2, U3> FromSI for Energy<U1, U2, U3>
where
    U1: UnitFactor, // Mass
    U2: UnitFactor, // Distance (appears squared in energy)
    U3: UnitFactor, // Time
{
    fn from_si(value: f64) -> Self {
        // Convert from SI (kg⋅m²/s²) to target units
        let factor = U1::factor() * (U2::factor() * U2::factor()) / (U3::factor() * U3::factor());
        Self::new(value / factor)
    }
}

// Power implementations
impl<U1, U2, U3> ToSI for Power<U1, U2, U3>
where
    U1: UnitFactor, // Mass
    U2: UnitFactor, // Distance (appears squared in power)
    U3: UnitFactor, // Time
{
    fn to_si(&self) -> f64 {
        // Power = Mass * Distance² / Time³, so factor = U1 * U2² / U3³
        self.value * U1::factor() * (U2::factor() * U2::factor()) / (U3::factor() * U3::factor() * U3::factor())
    }
}

impl<U1, U2, U3> FromSI for Power<U1, U2, U3>
where
    U1: UnitFactor, // Mass
    U2: UnitFactor, // Distance (appears squared in power)
    U3: UnitFactor, // Time
{
    fn from_si(value: f64) -> Self {
        // Convert from SI (kg⋅m²/s³) to target units
        let factor = U1::factor() * (U2::factor() * U2::factor()) / (U3::factor() * U3::factor() * U3::factor());
        Self::new(value / factor)
    }
}

// Area implementations
impl<U1, U2> ToSI for Area<U1, U2>
where
    U1: UnitFactor,
    U2: UnitFactor,
{
    fn to_si(&self) -> f64 {
        // Area = Distance * Distance, so factor = U1::factor() * U2::factor()
        self.value * U1::factor() * U2::factor()
    }
}

impl<U1, U2> FromSI for Area<U1, U2>
where
    U1: UnitFactor,
    U2: UnitFactor,
{
    fn from_si(value: f64) -> Self {
        // Convert from SI (m²) to target units
        let factor = U1::factor() * U2::factor();
        Self::new(value / factor)
    }
}

// Volume implementations
impl<U1, U2, U3> ToSI for Volume<U1, U2, U3>
where
    U1: UnitFactor,
    U2: UnitFactor,
    U3: UnitFactor,
{
    fn to_si(&self) -> f64 {
        // Volume = Distance * Distance * Distance, so factor = U1 * U2 * U3
        self.value * U1::factor() * U2::factor() * U3::factor()
    }
}

impl<U1, U2, U3> FromSI for Volume<U1, U2, U3>
where
    U1: UnitFactor,
    U2: UnitFactor,
    U3: UnitFactor,
{
    fn from_si(value: f64) -> Self {
        // Convert from SI (m³) to target units
        let factor = U1::factor() * U2::factor() * U3::factor();
        Self::new(value / factor)
    }
}

// ================================================================================================
// COMPREHENSIVE CONVERSION TESTS
// ================================================================================================

#[cfg(test)]
mod conversion_tests {
    use super::{Velocity, Acceleration, Force, Energy, Area, Volume, Unit3};
    use crate::core::{ToSI, FromSI, UnitSymbol};
    use crate::prefix::*;
    use crate::quantities::*;

    #[test]
    fn test_velocity_conversions() {
        // Test m/s to km/h conversion
        let velocity_ms = Velocity::<Meter, Second>::new(10.0); // 10 m/s
        let si_value = velocity_ms.to_si(); // Should be 10.0 (already in SI)
        assert!((si_value - 10.0).abs() < 1e-10);

        // Create equivalent velocity in km/h: 10 m/s = 36 km/h
        let velocity_kmh = Velocity::<Prefixed<Kilo, Meter>, Hour>::from_si(si_value);
        assert!((velocity_kmh.value - 36.0).abs() < 1e-10);

        // Test round-trip conversion
        let velocity_ms_back = Velocity::<Meter, Second>::from_si(velocity_kmh.to_si());
        assert!((velocity_ms_back.value - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_acceleration_conversions() {
        // Test m/s² conversion
        let accel_si = Acceleration::<Meter, Second>::new(9.81); // Earth gravity
        let si_value = accel_si.to_si();
        assert!((si_value - 9.81).abs() < 1e-10);

        // Test with different units: km/h² 
        // 9.81 m/s² = 9.81 * 3600² / 1000 km/h² = 127036.8 km/h²
        let accel_kmh2 = Acceleration::<Prefixed<Kilo, Meter>, Hour>::from_si(si_value);
        let expected = 9.81 * 3600.0 * 3600.0 / 1000.0;
        assert!((accel_kmh2.value - expected).abs() < 1e-6);
    }

    #[test]
    fn test_force_conversions() {
        // Test kg⋅m/s² (Newton)
        let force_n = Force::<Kilogram, Meter, Second>::new(100.0); // 100 N
        let si_value = force_n.to_si();
        assert!((si_value - 100.0).abs() < 1e-10);

        // Test with different mass unit: g⋅m/s²
        // 100 kg⋅m/s² = 100000 g⋅m/s²
        let force_g = Force::<Gram, Meter, Second>::from_si(si_value);
        assert!((force_g.value - 100000.0).abs() < 1e-6);
    }

    #[test]
    fn test_energy_conversions() {
        // Test kg⋅m²/s² (Joule)
        let energy_j = Energy::<Kilogram, Meter, Second>::new(1000.0); // 1000 J
        let si_value = energy_j.to_si();
        assert!((si_value - 1000.0).abs() < 1e-10);

        // Test with prefixed units: g⋅km²/s²
        // 1000 kg⋅m²/s² = 1 g⋅km²/s² (correct conversion)
        let energy_gkm = Energy::<Gram, Prefixed<Kilo, Meter>, Second>::from_si(si_value);
        assert!((energy_gkm.value - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_area_conversions() {
        // Test m²
        let area_m2 = Area::<Meter, Meter>::new(100.0); // 100 m²
        let si_value = area_m2.to_si();
        assert!((si_value - 100.0).abs() < 1e-10);

        // Test km²: 100 m² = 0.0001 km²
        let area_km2 = Area::<Prefixed<Kilo, Meter>, Prefixed<Kilo, Meter>>::from_si(si_value);
        let expected = 100.0 / (1000.0 * 1000.0);
        assert!((area_km2.value - expected).abs() < 1e-12);
    }

    #[test]
    fn test_volume_conversions() {
        // Test m³
        let volume_m3 = Volume::<Meter, Meter, Meter>::new(8.0); // 8 m³
        let si_value = volume_m3.to_si();
        assert!((si_value - 8.0).abs() < 1e-10);

        // Test km³: 8 m³ = 8e-9 km³
        let volume_km3 = Volume::<Prefixed<Kilo, Meter>, Prefixed<Kilo, Meter>, Prefixed<Kilo, Meter>>::from_si(si_value);
        let expected = 8.0 / (1000.0 * 1000.0 * 1000.0);
        assert!((volume_km3.value - expected).abs() < 1e-18);
    }

    #[test]
    fn test_astronomical_units() {
        // Test AU-based velocity
        let velocity_au_year = Velocity::<AstronomicalUnit, Year>::new(1.0); // 1 AU/year
        let si_value = velocity_au_year.to_si(); // Should be AU/year in m/s
        
        let expected = crate::constants::METERS_PER_AU / crate::constants::SECONDS_PER_YEAR;
        assert!((si_value - expected).abs() < 1e-6);

        // Convert back to m/s
        let velocity_ms = Velocity::<Meter, Second>::from_si(si_value);
        assert!((velocity_ms.value - expected).abs() < 1e-6);
    }

    #[test]
    fn test_symbol_generation() {
        // Test velocity symbols (variadic type)
        assert_eq!(Velocity::<Meter, Second>::symbol(), "m/s");
        assert_eq!(Velocity::<Prefixed<Kilo, Meter>, Hour>::symbol(), "km/h");

        // Test acceleration symbols (variadic type)
        assert_eq!(Acceleration::<Meter, Second>::symbol(), "m/s²");

        // Test force symbols (variadic type)
        assert_eq!(Force::<Kilogram, Meter, Second>::symbol(), "kg⋅m/s²");

        // Test energy symbols (variadic type)
        assert_eq!(Energy::<Kilogram, Meter, Second>::symbol(), "kg⋅m²/s²");

        // Test variadic power symbols (using our variadic Power type)
        type VariadicPower<U1, U2, U3> = Power<Unit3<U1, U2, U3>>;
        assert_eq!(VariadicPower::<Kilogram, Meter, Second>::symbol(), "kg⋅m²/s³");

        // Test area symbols (variadic type)
        assert_eq!(Area::<Meter, Meter>::symbol(), "m⋅m");

        // Test volume symbols (variadic type)
        assert_eq!(Volume::<Meter, Meter, Meter>::symbol(), "m⋅m⋅m");

        // Test prefixed units in symbols
        assert_eq!(Area::<Prefixed<Kilo, Meter>, Prefixed<Kilo, Meter>>::symbol(), "km⋅km");
    }
}