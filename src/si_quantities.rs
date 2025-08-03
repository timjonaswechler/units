use crate::{Quantity, Dimension};

// Basic quantities corresponding to SI base units
#[derive(Debug, Clone, Copy)]
pub struct Distance;

impl Quantity for Distance {
    const DIMENSION: Dimension = Dimension::length();
    const NAME: &'static str = "Distance";
}

#[derive(Debug, Clone, Copy)]
pub struct Time;

impl Quantity for Time {
    const DIMENSION: Dimension = Dimension::time();
    const NAME: &'static str = "Time";
}

#[derive(Debug, Clone, Copy)]
pub struct Mass;

impl Quantity for Mass {
    const DIMENSION: Dimension = Dimension::mass();
    const NAME: &'static str = "Mass";
}

#[derive(Debug, Clone, Copy)]
pub struct ElectricCurrent;

impl Quantity for ElectricCurrent {
    const DIMENSION: Dimension = Dimension::current();
    const NAME: &'static str = "ElectricCurrent";
}

#[derive(Debug, Clone, Copy)]
pub struct Temperature;

impl Quantity for Temperature {
    const DIMENSION: Dimension = Dimension::temperature();
    const NAME: &'static str = "Temperature";
}

#[derive(Debug, Clone, Copy)]
pub struct AmountOfSubstance;

impl Quantity for AmountOfSubstance {
    const DIMENSION: Dimension = Dimension::amount();
    const NAME: &'static str = "AmountOfSubstance";
}

#[derive(Debug, Clone, Copy)]
pub struct LuminousIntensity;

impl Quantity for LuminousIntensity {
    const DIMENSION: Dimension = Dimension::luminosity();
    const NAME: &'static str = "LuminousIntensity";
}

// Derived quantities
#[derive(Debug, Clone, Copy)]
pub struct Area;

impl Quantity for Area {
    const DIMENSION: Dimension = Dimension::AREA;
    const NAME: &'static str = "Area";
}

#[derive(Debug, Clone, Copy)]
pub struct Volume;

impl Quantity for Volume {
    const DIMENSION: Dimension = Dimension::VOLUME;
    const NAME: &'static str = "Volume";
}

#[derive(Debug, Clone, Copy)]
pub struct Speed;

impl Quantity for Speed {
    const DIMENSION: Dimension = Dimension::VELOCITY;
    const NAME: &'static str = "Speed";
}

#[derive(Debug, Clone, Copy)]
pub struct Acceleration;

impl Quantity for Acceleration {
    const DIMENSION: Dimension = Dimension::ACCELERATION;
    const NAME: &'static str = "Acceleration";
}

#[derive(Debug, Clone, Copy)]
pub struct Force;

impl Quantity for Force {
    const DIMENSION: Dimension = Dimension::FORCE;
    const NAME: &'static str = "Force";
}

#[derive(Debug, Clone, Copy)]
pub struct Energy;

impl Quantity for Energy {
    const DIMENSION: Dimension = Dimension::ENERGY;
    const NAME: &'static str = "Energy";
}

#[derive(Debug, Clone, Copy)]
pub struct Power;

impl Quantity for Power {
    const DIMENSION: Dimension = Dimension::POWER;
    const NAME: &'static str = "Power";
}

#[derive(Debug, Clone, Copy)]
pub struct Pressure;

impl Quantity for Pressure {
    const DIMENSION: Dimension = Dimension::PRESSURE;
    const NAME: &'static str = "Pressure";
}

#[derive(Debug, Clone, Copy)]
pub struct Frequency;

impl Quantity for Frequency {
    const DIMENSION: Dimension = Dimension::FREQUENCY;
    const NAME: &'static str = "Frequency";
}