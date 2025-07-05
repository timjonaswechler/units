//! Demo für Frequency quantity

use units::prelude::*;

fn main() {
    println!("🎵 Frequency Quantity Demo");
    println!("=========================\n");

    // 1. Basic Frequency units
    println!("1. Basic Frequency units:");
    let freq_hz = Frequency::<Hertz>::new(1000.0);
    let freq_khz = Frequency::<Kilohertz>::new(1.0);
    println!("   {} Hz = {} kHz", freq_hz.value(), freq_khz.value());
    
    let converted: Frequency<Kilohertz> = freq_hz.convert_to();
    println!("   Conversion check: {} Hz = {} kHz\n", freq_hz.value(), converted.value());

    // 2. Electromagnetic spectrum
    println!("2. Electromagnetic spectrum:");
    let radio_fm = Frequency::<Megahertz>::new(100.0);  // FM radio
    let wifi = Frequency::<Gigahertz>::new(2.4);        // WiFi
    let visible = Frequency::<Terahertz>::new(500.0);   // Green light
    
    println!("   FM Radio: {} MHz", radio_fm.value());
    println!("   WiFi: {} GHz", wifi.value());
    println!("   Green light: {} THz", visible.value());
    
    let wifi_hz: Frequency<Hertz> = wifi.convert_to();
    let visible_hz: Frequency<Hertz> = visible.convert_to();
    println!("   WiFi: {:.1e} Hz", wifi_hz.value());
    println!("   Visible: {:.1e} Hz\n", visible_hz.value());

    // 3. Pulsar astronomy
    println!("3. Pulsar frequencies:");
    let crab_pulsar = Frequency::<Hertz>::new(30.0);     // Crab pulsar
    let ms_pulsar = Frequency::<Hertz>::new(716.0);      // Fast millisecond pulsar
    
    println!("   Crab pulsar: {} Hz", crab_pulsar.value());
    println!("   Millisecond pulsar: {} Hz", ms_pulsar.value());
    
    let crab_rpm: Frequency<RevolutionsPerMinute> = crab_pulsar.convert_to();
    let ms_rpm: Frequency<RevolutionsPerMinute> = ms_pulsar.convert_to();
    println!("   Crab: {:.0} rpm", crab_rpm.value());
    println!("   MS pulsar: {:.0} rpm\n", ms_rpm.value());

    // 4. Stellar oscillations (asteroseismology)
    println!("4. Stellar oscillations:");
    let solar_osc = Frequency::<Microhertz>::new(3000.0); // Solar 5-minute oscillations
    let solar_hz: Frequency<Hertz> = solar_osc.convert_to();
    let solar_period = 1.0 / solar_hz.value(); // Period in seconds
    
    println!("   Solar oscillations: {} μHz", solar_osc.value());
    println!("   = {:.3e} Hz", solar_hz.value());
    println!("   Period: {:.0} seconds = {:.1} minutes\n", solar_period, solar_period / 60.0);

    // 5. Rotational frequencies
    println!("5. Rotational frequencies:");
    let earth_rotation = Frequency::<CyclesPerDay>::new(1.0);
    let earth_hz: Frequency<Hertz> = earth_rotation.convert_to();
    let earth_nHz: Frequency<Nanohertz> = earth_rotation.convert_to();
    
    println!("   Earth rotation: 1 cycle/day");
    println!("   = {:.2e} Hz", earth_hz.value());
    println!("   = {:.1} nHz", earth_nHz.value());
    
    let turbine = Frequency::<RevolutionsPerMinute>::new(3600.0); // Gas turbine
    let turbine_hz: Frequency<Hertz> = turbine.convert_to();
    println!("   Gas turbine: {} rpm = {} Hz\n", turbine.value(), turbine_hz.value());

    // 6. Orbital frequencies
    println!("6. Orbital frequencies:");
    let earth_orbit = Frequency::<CyclesPerYear>::new(1.0);
    let earth_orbit_hz: Frequency<Hertz> = earth_orbit.convert_to();
    let earth_orbit_nHz: Frequency<Nanohertz> = earth_orbit.convert_to();
    
    println!("   Earth orbital frequency: 1 cycle/year");
    println!("   = {:.2e} Hz", earth_orbit_hz.value());
    println!("   = {:.3} nHz\n", earth_orbit_nHz.value());

    // 7. Mixed unit arithmetic
    println!("7. Mixed unit arithmetic:");
    let freq1 = Frequency::<Hertz>::new(1000.0);        // 1 kHz
    let freq2 = Frequency::<Kilohertz>::new(0.5);       // 500 Hz
    let sum = freq1 + freq2;  // Result in SI units (Hz)
    println!("   {} Hz + {} kHz = {} Hz (SI)", 
             freq1.value(), freq2.value(), sum.value());
    
    let sum_khz: Frequency<Kilohertz> = sum.convert_to();
    println!("   = {} kHz\n", sum_khz.value());

    // 8. Dimensionless ratios (harmonic relationships)
    println!("8. Harmonic relationships:");
    let fundamental = Frequency::<Hertz>::new(440.0);    // A4 note
    let octave = Frequency::<Hertz>::new(880.0);         // A5 note
    let ratio = octave / fundamental;
    println!("   A5 ({} Hz) ÷ A4 ({} Hz) = {} (octave ratio)", 
             octave.value(), fundamental.value(), ratio);
    
    let third_harmonic = Frequency::<Hertz>::new(1320.0); // 3rd harmonic
    let harmonic_ratio = third_harmonic / fundamental;
    println!("   3rd harmonic ratio: {}\n", harmonic_ratio);

    // 9. Precision timing and atomic clocks
    println!("9. Precision frequencies:");
    let cesium_transition = Frequency::<Gigahertz>::new(9.192_631_770); // Cs-133 hyperfine
    let cesium_hz: Frequency<Hertz> = cesium_transition.convert_to();
    
    println!("   Cesium atomic clock: {:.9} GHz", cesium_transition.value());
    println!("   = {:.0} Hz (exact by definition)", cesium_hz.value());
    
    // GPS satellite frequency
    let gps_l1 = Frequency::<Megahertz>::new(1575.42);
    println!("   GPS L1 frequency: {} MHz\n", gps_l1.value());

    println!("✅ Frequency quantity funktioniert perfekt!");
    println!("🎯 Pulsar timing from Hz to nHz precision");
    println!("🎯 Electromagnetic spectrum from radio to light");
    println!("📡 Astronomical oscillations and orbital frequencies");
    println!("⚛️ Atomic clock precision and GPS applications!");
}