use crate::{Unit, Prefix, Dimension};

// SI Base Units
#[derive(Debug, Clone, Copy)]
pub struct Meter;

impl Unit for Meter {
    const DIMENSION: Dimension = Dimension::length();
    const SCALE: f64 = 1.0;
    const NAME: &'static str = "meter";
    const SYMBOL: &'static str = "m";
}

#[derive(Debug, Clone, Copy)]
pub struct Second;

impl Unit for Second {
    const DIMENSION: Dimension = Dimension::time();
    const SCALE: f64 = 1.0;
    const NAME: &'static str = "second";
    const SYMBOL: &'static str = "s";
}

#[derive(Debug, Clone, Copy)]
pub struct Kilogram;

impl Unit for Kilogram {
    const DIMENSION: Dimension = Dimension::mass();
    const SCALE: f64 = 1.0;
    const NAME: &'static str = "kilogram";
    const SYMBOL: &'static str = "kg";
}

#[derive(Debug, Clone, Copy)]
pub struct Ampere;

impl Unit for Ampere {
    const DIMENSION: Dimension = Dimension::current();
    const SCALE: f64 = 1.0;
    const NAME: &'static str = "ampere";
    const SYMBOL: &'static str = "A";
}

#[derive(Debug, Clone, Copy)]
pub struct Kelvin;

impl Unit for Kelvin {
    const DIMENSION: Dimension = Dimension::temperature();
    const SCALE: f64 = 1.0;
    const NAME: &'static str = "kelvin";
    const SYMBOL: &'static str = "K";
}

#[derive(Debug, Clone, Copy)]
pub struct Mole;

impl Unit for Mole {
    const DIMENSION: Dimension = Dimension::amount();
    const SCALE: f64 = 1.0;
    const NAME: &'static str = "mole";
    const SYMBOL: &'static str = "mol";
}

#[derive(Debug, Clone, Copy)]
pub struct Candela;

impl Unit for Candela {
    const DIMENSION: Dimension = Dimension::luminosity();
    const SCALE: f64 = 1.0;
    const NAME: &'static str = "candela";
    const SYMBOL: &'static str = "cd";
}

// Common derived units
#[derive(Debug, Clone, Copy)]
pub struct Newton;

impl Unit for Newton {
    const DIMENSION: Dimension = Dimension::FORCE;
    const SCALE: f64 = 1.0; // kg⋅m⋅s⁻²
    const NAME: &'static str = "newton";
    const SYMBOL: &'static str = "N";
}

#[derive(Debug, Clone, Copy)]
pub struct Joule;

impl Unit for Joule {
    const DIMENSION: Dimension = Dimension::ENERGY;
    const SCALE: f64 = 1.0; // kg⋅m²⋅s⁻²
    const NAME: &'static str = "joule";
    const SYMBOL: &'static str = "J";
}

#[derive(Debug, Clone, Copy)]
pub struct Watt;

impl Unit for Watt {
    const DIMENSION: Dimension = Dimension::POWER;
    const SCALE: f64 = 1.0; // kg⋅m²⋅s⁻³
    const NAME: &'static str = "watt";
    const SYMBOL: &'static str = "W";
}

// SI Prefixes
#[derive(Debug, Clone, Copy)]
pub struct Kilo;

impl Prefix for Kilo {
    const FACTOR: f64 = 1e3;
    const SYMBOL: &'static str = "k";
}

#[derive(Debug, Clone, Copy)]
pub struct Milli;

impl Prefix for Milli {
    const FACTOR: f64 = 1e-3;
    const SYMBOL: &'static str = "m";
}

#[derive(Debug, Clone, Copy)]
pub struct Micro;

impl Prefix for Micro {
    const FACTOR: f64 = 1e-6;
    const SYMBOL: &'static str = "μ";
}

#[derive(Debug, Clone, Copy)]
pub struct Nano;

impl Prefix for Nano {
    const FACTOR: f64 = 1e-9;
    const SYMBOL: &'static str = "n";
}

#[derive(Debug, Clone, Copy)]
pub struct Mega;

impl Prefix for Mega {
    const FACTOR: f64 = 1e6;
    const SYMBOL: &'static str = "M";
}

#[derive(Debug, Clone, Copy)]
pub struct Giga;

impl Prefix for Giga {
    const FACTOR: f64 = 1e9;
    const SYMBOL: &'static str = "G";
}

// More derived units for testing
#[derive(Debug, Clone, Copy)]
pub struct Pascal;

impl Unit for Pascal {
    const DIMENSION: Dimension = Dimension::PRESSURE;
    const SCALE: f64 = 1.0; // N⋅m⁻² = kg⋅m⁻¹⋅s⁻²
    const NAME: &'static str = "pascal";
    const SYMBOL: &'static str = "Pa";
}

#[derive(Debug, Clone, Copy)]
pub struct SquareMeter;

impl Unit for SquareMeter {
    const DIMENSION: Dimension = Dimension::AREA;
    const SCALE: f64 = 1.0; // m²
    const NAME: &'static str = "square meter";
    const SYMBOL: &'static str = "m²";
}

#[derive(Debug, Clone, Copy)]
pub struct MeterPerSecond;

impl Unit for MeterPerSecond {
    const DIMENSION: Dimension = Dimension::VELOCITY;
    const SCALE: f64 = 1.0; // m⋅s⁻¹
    const NAME: &'static str = "meter per second";
    const SYMBOL: &'static str = "m/s";
}

// Common prefixed units
use crate::Prefixed;

pub type Kilometer = Prefixed<Kilo, Meter>;