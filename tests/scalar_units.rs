use std::mem::{align_of, size_of};

use units::{
    AstronomicalUnit, Centi, Day, EarthMass, Giga, Gram, Hour, JulianYear, Kelvin, Kilo, Kilogram,
    Kilometer, Length, LightYear, Mass, Mega, Meter, Milli, Millisecond, Minute, Parsec, Prefixed,
    Second, SolarMass, Temperature, Time,
};

fn assert_close(actual: f64, expected: f64, relative_tolerance: f64) {
    let scale = expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= relative_tolerance * scale,
        "expected {expected:e}, got {actual:e}"
    );
}

macro_rules! assert_unit_roundtrip {
    ($quantity:ty, $unit:ty, $value:expr) => {{
        let quantity = <$quantity>::new::<$unit>($value);
        assert_close(quantity.to::<$unit>(), $value, 1e-14);
    }};
}

#[test]
fn quantities_construct_and_convert_through_canonical_si() {
    let sun = Mass::new::<SolarMass>(1.0);
    assert_close(sun.si(), 1.988_409_870_698_051e30, 1e-15);
    assert_close(sun.to::<SolarMass>(), 1.0, 1e-15);
    assert_close(sun.to::<Kilogram>(), 1.988_409_870_698_051e30, 1e-15);

    let earth = Mass::new::<EarthMass>(1.0);
    assert_close(earth.si(), 5.972_167_867_791_379e24, 1e-15);
    assert_close(earth.to::<EarthMass>(), 1.0, 1e-15);

    let grams = Mass::new::<Gram>(2_500.0);
    assert_eq!(grams.si(), 2.5);
    assert_eq!(grams.to::<Kilogram>(), 2.5);
}

#[test]
fn length_scales_match_the_agreed_astronomical_definitions() {
    assert_eq!(Length::new::<AstronomicalUnit>(1.0).si(), 149_597_870_700.0);
    assert_eq!(Length::new::<LightYear>(1.0).si(), 9_460_730_472_580_800.0);
    assert_close(
        Length::new::<Parsec>(1.0).si(),
        3.085_677_581_491_367e16,
        1e-15,
    );

    let distance = Length::new::<Kilometer>(2.5);
    assert_eq!(distance.si(), 2_500.0);
    assert_eq!(distance.to::<Meter>(), 2_500.0);
    assert_eq!(distance.to::<Kilometer>(), 2.5);
}

#[test]
fn every_agreed_unit_roundtrips_through_si() {
    assert_unit_roundtrip!(Mass, Gram, 12.5);
    assert_unit_roundtrip!(Mass, Kilogram, 12.5);
    assert_unit_roundtrip!(Mass, SolarMass, 12.5);
    assert_unit_roundtrip!(Mass, EarthMass, 12.5);

    assert_unit_roundtrip!(Length, Meter, 12.5);
    assert_unit_roundtrip!(Length, Kilometer, 12.5);
    assert_unit_roundtrip!(Length, AstronomicalUnit, 12.5);
    assert_unit_roundtrip!(Length, LightYear, 12.5);
    assert_unit_roundtrip!(Length, Parsec, 12.5);

    assert_unit_roundtrip!(Time, Second, 12.5);
    assert_unit_roundtrip!(Time, Millisecond, 12.5);
    assert_unit_roundtrip!(Time, Minute, 12.5);
    assert_unit_roundtrip!(Time, Hour, 12.5);
    assert_unit_roundtrip!(Time, Day, 12.5);
    assert_unit_roundtrip!(Time, JulianYear, 12.5);

    assert_unit_roundtrip!(Temperature, Kelvin, 12.5);
}

#[test]
fn every_agreed_prefix_scales_prefixable_units() {
    type Millimeter = Prefixed<Milli, Meter>;
    type Centimeter = Prefixed<Centi, Meter>;
    type Megameter = Prefixed<Mega, Meter>;
    type Gigameter = Prefixed<Giga, Meter>;
    type Millikelvin = Prefixed<Milli, Kelvin>;

    assert_eq!(Length::new::<Millimeter>(1.0).si(), 1e-3);
    assert_eq!(Length::new::<Centimeter>(1.0).si(), 1e-2);
    assert_eq!(Length::new::<Prefixed<Kilo, Meter>>(1.0).si(), 1e3);
    assert_eq!(Length::new::<Megameter>(1.0).si(), 1e6);
    assert_eq!(Length::new::<Gigameter>(1.0).si(), 1e9);
    assert_eq!(Temperature::new::<Millikelvin>(1.0).si(), 1e-3);
}

#[test]
fn time_scales_and_prefixes_convert_to_seconds() {
    assert_eq!(Time::new::<Minute>(1.0).si(), 60.0);
    assert_eq!(Time::new::<Hour>(1.0).si(), 3_600.0);
    assert_eq!(Time::new::<Day>(1.0).si(), 86_400.0);
    assert_eq!(Time::new::<JulianYear>(1.0).si(), 31_557_600.0);
    assert_eq!(Time::new::<Millisecond>(250.0).si(), 0.25);
    assert_eq!(Time::new::<Second>(0.25).to::<Millisecond>(), 250.0);
}

#[test]
fn kelvin_uses_a_linear_si_scale() {
    let temperature = Temperature::new::<Kelvin>(5_778.0);
    assert_eq!(temperature.si(), 5_778.0);
    assert_eq!(temperature.to::<Kelvin>(), 5_778.0);
}

#[test]
fn from_si_preserves_every_f64_category() {
    assert_eq!(Mass::from_si(-1.0).si(), -1.0);
    assert_eq!(Length::from_si(-1.0).si(), -1.0);
    assert_eq!(Time::from_si(-1.0).si(), -1.0);
    assert_eq!(Temperature::from_si(-1.0).si(), -1.0);

    assert!(Mass::from_si(f64::INFINITY).si().is_infinite());
    assert!(Length::from_si(f64::NEG_INFINITY).si().is_infinite());
    assert!(Time::from_si(f64::NAN).si().is_nan());
    assert!(Temperature::from_si(f64::NAN).si().is_nan());
}

#[test]
fn quantities_have_transparent_f64_layout() {
    assert_eq!(size_of::<Mass>(), size_of::<f64>());
    assert_eq!(align_of::<Mass>(), align_of::<f64>());
    assert_eq!(size_of::<Length>(), size_of::<f64>());
    assert_eq!(size_of::<Time>(), size_of::<f64>());
    assert_eq!(size_of::<Temperature>(), size_of::<f64>());
}

#[cfg(feature = "serde")]
#[test]
fn serde_uses_the_bare_canonical_si_value() {
    let mass = Mass::new::<Kilogram>(12.5);

    assert_eq!(serde_json::to_string(&mass).unwrap(), "12.5");
    assert_eq!(serde_json::from_str::<Mass>("12.5").unwrap(), mass);

    let ron = ron::to_string(&mass).unwrap();
    assert_eq!(ron, "12.5");
    assert_eq!(ron::from_str::<Mass>(&ron).unwrap(), mass);

    let negative = Mass::from_si(-12.5);
    assert_eq!(serde_json::from_str::<Mass>("-12.5").unwrap(), negative);
    assert_eq!(ron::from_str::<Mass>("-12.5").unwrap(), negative);
}

#[cfg(feature = "serde")]
#[test]
fn serde_formats_define_their_own_non_finite_number_behavior() {
    let nan = Mass::from_si(f64::NAN);
    assert_eq!(serde_json::to_string(&nan).unwrap(), "null");
    assert!(serde_json::from_str::<Mass>("null").is_err());

    let ron = ron::to_string(&nan).unwrap();
    assert!(ron::from_str::<Mass>(&ron).unwrap().si().is_nan());
}
