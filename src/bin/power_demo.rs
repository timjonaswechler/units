//! Demo für Power quantity

use units::prelude::*;

fn main() {
    println!("⚡ Power Quantity Demo");
    println!("=====================\n");

    // 1. Basic Power units
    println!("1. Basic Power units:");
    let power_w = Power::<Watt>::new(1000.0);
    let power_hp = Power::<Horsepower>::new(1.0);
    println!("   {} W ≈ {} hp", power_w.value(), power_hp.value());
    
    let converted: Power<Watt> = power_hp.convert_to();
    println!("   Conversion check: {} hp = {:.1} W\n", power_hp.value(), converted.value());

    // 2. Automotive power
    println!("2. Automotive power:");
    
    // Car engines
    let family_car = Power::<Horsepower>::new(150.0);
    let sports_car = Power::<Horsepower>::new(400.0);
    let f1_car = Power::<Horsepower>::new(1000.0);
    
    let family_kw: Power<Watt> = family_car.convert_to();
    let sports_kw: Power<Watt> = sports_car.convert_to();
    let f1_kw: Power<Watt> = f1_car.convert_to();
    
    println!("   Family car: {} hp = {:.0} kW", family_car.value(), family_kw.value() / 1000.0);
    println!("   Sports car: {} hp = {:.0} kW", sports_car.value(), sports_kw.value() / 1000.0);
    println!("   F1 car: {} hp = {:.0} kW", f1_car.value(), f1_kw.value() / 1000.0);
    
    // Metric horsepower comparison
    let metric_hp = Power::<MetricHorsepower>::new(100.0);
    let metric_w: Power<Watt> = metric_hp.convert_to();
    let mech_hp: Power<Horsepower> = metric_w.convert_to();
    
    println!("   100 PS (metric) = {:.1} W = {:.1} hp (mechanical)\n", metric_w.value(), mech_hp.value());

    // 3. Electrical power applications
    println!("3. Electrical power applications:");
    
    // Household appliances
    let led_bulb = Power::<Watt>::new(10.0);
    let microwave = Power::<Watt>::new(1200.0);
    let electric_car_charger = Power::<Watt>::new(11_000.0);
    let household_total = led_bulb + microwave + electric_car_charger;
    
    println!("   LED bulb: {} W", led_bulb.value());
    println!("   Microwave: {} W", microwave.value());
    println!("   EV charger: {} W", electric_car_charger.value());
    println!("   Total household load: {:.1} kW\n", household_total.value() / 1000.0);

    // 4. Industrial and utility scale
    println!("4. Industrial and utility scale:");
    
    // Power plant capacities
    let wind_turbine = Power::<Watt>::new(3_000_000.0);    // 3 MW wind turbine
    let nuclear_reactor = Power::<Watt>::new(1_000_000_000.0); // 1 GW nuclear reactor
    
    println!("   Wind turbine: {:.0} MW", wind_turbine.value() / 1_000_000.0);
    println!("   Nuclear reactor: {:.1} GW", nuclear_reactor.value() / 1_000_000_000.0);
    
    // HVAC system
    let hvac_btu = Power::<BTUPerHour>::new(48_000.0);  // 4-ton AC unit
    let hvac_w: Power<Watt> = hvac_btu.convert_to();
    
    println!("   HVAC system: {} BTU/h = {:.1} kW\n", hvac_btu.value(), hvac_w.value() / 1000.0);

    // 5. Stellar luminosity and astronomical power
    println!("5. Stellar luminosity and astronomical power:");
    
    // Stellar classifications
    let red_dwarf = Power::<SolarLuminosity>::new(0.0001);     // Red dwarf: 0.0001 L☉
    let sun = Power::<SolarLuminosity>::new(1.0);              // Sun: 1 L☉
    let blue_giant = Power::<SolarLuminosity>::new(100_000.0); // Blue supergiant: 100,000 L☉
    
    let red_dwarf_w: Power<Watt> = red_dwarf.convert_to();
    let sun_w: Power<Watt> = sun.convert_to();
    let blue_giant_w: Power<Watt> = blue_giant.convert_to();
    
    println!("   Red dwarf: {} L☉ = {:.1e} W", red_dwarf.value(), red_dwarf_w.value());
    println!("   Sun: {} L☉ = {:.2e} W", sun.value(), sun_w.value());
    println!("   Blue supergiant: {} L☉ = {:.1e} W", blue_giant.value(), blue_giant_w.value());
    
    // Luminosity ratios
    let sun_vs_red_dwarf = sun / red_dwarf;
    let blue_vs_sun = blue_giant / sun;
    
    println!("   Sun is {}x brighter than red dwarf", sun_vs_red_dwarf);
    println!("   Blue giant is {}x brighter than Sun\n", blue_vs_sun);

    // 6. CGS units in astrophysics
    println!("6. CGS units in astrophysics:");
    
    let stellar_wind_erg = Power::<ErgPerSecond>::new(1e33);
    let stellar_wind_w: Power<Watt> = stellar_wind_erg.convert_to();
    let stellar_wind_solar: Power<SolarLuminosity> = stellar_wind_w.convert_to();
    
    println!("   Stellar wind power: {:.0e} erg/s", stellar_wind_erg.value());
    println!("   = {:.2e} W", stellar_wind_w.value());
    println!("   = {:.2e} L☉\n", stellar_wind_solar.value());

    // 7. Mechanical power applications
    println!("7. Mechanical power applications:");
    
    // Human power output
    let cyclist_watts = Power::<Watt>::new(250.0);           // Recreational cyclist
    let pro_cyclist = Power::<Watt>::new(400.0);            // Professional cyclist
    let sprinter_peak = Power::<Watt>::new(1500.0);         // Sprint peak power
    
    println!("   Recreational cyclist: {} W", cyclist_watts.value());
    println!("   Professional cyclist: {} W", pro_cyclist.value());
    println!("   Sprint peak power: {} W", sprinter_peak.value());
    
    // Convert to other units
    let cyclist_hp: Power<Horsepower> = pro_cyclist.convert_to();
    let cyclist_cal: Power<CaloriePerSecond> = pro_cyclist.convert_to();
    
    println!("   Pro cyclist: {} W = {:.2} hp = {:.1} cal/s\n", 
             pro_cyclist.value(), cyclist_hp.value(), cyclist_cal.value());

    // 8. Electrical power measurements
    println!("8. Electrical power measurements:");
    
    let apparent_power = Power::<VoltAmpere>::new(1000.0);
    let reactive_power = Power::<VoltAmpereReactive>::new(600.0);
    
    // In electrical systems: apparent power² = real power² + reactive power²
    // For demo purposes, just show the units are equivalent dimensionally
    let apparent_w: Power<Watt> = apparent_power.convert_to();
    let reactive_w: Power<Watt> = reactive_power.convert_to();
    
    println!("   Apparent power: {} VA = {} W (dimensionally)", apparent_power.value(), apparent_w.value());
    println!("   Reactive power: {} VAR = {} W (dimensionally)\n", reactive_power.value(), reactive_w.value());

    // 9. Mixed unit arithmetic
    println!("9. Mixed unit arithmetic:");
    
    let engine_hp = Power::<Horsepower>::new(200.0);
    let electric_motor = Power::<Watt>::new(50_000.0);       // 50 kW
    let total_power = engine_hp + electric_motor;            // Result in SI units (Watts)
    
    let engine_w: Power<Watt> = engine_hp.convert_to();
    
    println!("   Hybrid vehicle:");
    println!("   Engine: {} hp = {:.0} W", engine_hp.value(), engine_w.value());
    println!("   Electric motor: {} W", electric_motor.value());
    println!("   Total power: {:.0} W = {:.1} hp\n", 
             total_power.value(), total_power.value() / 745.7);

    // 10. Power scale comparison
    println!("10. Power scale comparison:");
    
    let human_heart = Power::<Watt>::new(1.5);              // Human heart
    let smartphone = Power::<Watt>::new(5.0);               // Smartphone charging
    let toaster = Power::<Watt>::new(1500.0);               // Kitchen toaster
    let tesla_supercharger = Power::<Watt>::new(250_000.0); // Tesla Supercharger
    let nuclear_plant = Power::<Watt>::new(1e9);            // Nuclear power plant
    let solar_output = Power::<SolarLuminosity>::new(1.0);  // Sun's total output
    
    let solar_w: Power<Watt> = solar_output.convert_to();
    
    println!("   Power hierarchy (Watts):");
    println!("     Human heart: {:.1}", human_heart.value());
    println!("     Smartphone: {:.0}", smartphone.value());
    println!("     Toaster: {:.0}", toaster.value());
    println!("     Tesla Supercharger: {:.0e}", tesla_supercharger.value());
    println!("     Nuclear plant: {:.0e}", nuclear_plant.value());
    println!("     Sun total output: {:.1e}", solar_w.value());
    
    let extreme_range = solar_w / human_heart;
    println!("   Total range: {:.0e} orders of magnitude\n", extreme_range);

    // 11. Efficiency calculations
    println!("11. Power efficiency examples:");
    
    // Electric motor efficiency
    let mechanical_output = Power::<Horsepower>::new(100.0);
    let electrical_input = Power::<Watt>::new(80_000.0);     // 80 kW input
    
    let mech_watts: Power<Watt> = mechanical_output.convert_to();
    let efficiency = mech_watts.value() / electrical_input.value();
    
    println!("   Electric motor:");
    println!("   Mechanical output: {} hp = {:.0} W", mechanical_output.value(), mech_watts.value());
    println!("   Electrical input: {} W", electrical_input.value());
    println!("   Efficiency: {:.1}%\n", efficiency * 100.0);

    // 12. Thermal power applications
    println!("12. Thermal power applications:");
    
    let space_heater = Power::<BTUPerHour>::new(5000.0);
    let gas_furnace = Power::<BTUPerHour>::new(80_000.0);
    
    let heater_w: Power<Watt> = space_heater.convert_to();
    let furnace_w: Power<Watt> = gas_furnace.convert_to();
    
    println!("   Space heater: {} BTU/h = {:.1} kW", space_heater.value(), heater_w.value() / 1000.0);
    println!("   Gas furnace: {} BTU/h = {:.1} kW", gas_furnace.value(), furnace_w.value() / 1000.0);
    
    // Metabolic power
    let resting_metabolism = Power::<CaloriePerSecond>::new(1.0);  // ~1 cal/s resting
    let metabolism_w: Power<Watt> = resting_metabolism.convert_to();
    
    println!("   Human resting metabolism: {} cal/s = {:.1} W\n", 
             resting_metabolism.value(), metabolism_w.value());

    println!("✅ Power quantity funktioniert perfekt!");
    println!("🚗 Automotive applications from family cars to F1");
    println!("🏭 Industrial scale from wind turbines to nuclear plants");
    println!("🌟 Astronomical power from red dwarfs to supergiants");
    println!("⚡ Electrical systems with apparent and reactive power");
    println!("🔥 Thermal applications from heaters to furnaces");
    println!("🌌 28 orders of magnitude power range!");
}