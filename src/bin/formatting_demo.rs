//! Demo för enhanced formatting capabilities

use units::prelude::*;

fn main() {
    println!("🎨 Enhanced Formatting Demo");
    println!("===========================\n");

    // Create test quantities with extreme values
    let tiny_distance = Distance::<Meter>::new(1.23e-12);      // Picometer scale
    let huge_distance = Distance::<Meter>::new(4.56e26);       // Galactic scale
    let mass_solar = Mass::<Kilogram>::new(1.989e30);         // Solar mass
    let tiny_time = Time::<Second>::new(3.14e-15);            // Femtosecond scale
    let energy_huge = Energy::<Joule>::new(9.87e45);          // Massive energy
    let power_stellar = Power::<Watt>::new(3.828e26);         // Solar luminosity

    println!("1. Default Display (Enhanced):");
    println!("   Tiny distance: {}", tiny_distance);
    println!("   Huge distance: {}", huge_distance);
    println!("   Solar mass: {}", mass_solar);
    println!("   Tiny time: {}", tiny_time);
    println!("   Huge energy: {}", energy_huge);
    println!("   Stellar power: {}\n", power_stellar);

    println!("2. Scientific Formatting:");
    println!("   Tiny distance: {}", tiny_distance.format_scientific());
    println!("   Huge distance: {}", huge_distance.format_scientific());
    println!("   Solar mass: {}", mass_solar.format_scientific());
    println!("   Tiny time: {}", tiny_time.format_scientific());
    println!("   Huge energy: {}", energy_huge.format_scientific());
    println!("   Stellar power: {}\n", power_stellar.format_scientific());

    println!("3. Engineering Formatting:");
    println!("   Tiny distance: {}", tiny_distance.format_engineering());
    println!("   Huge distance: {}", huge_distance.format_engineering());
    println!("   Solar mass: {}", mass_solar.format_engineering());
    println!("   Tiny time: {}", tiny_time.format_engineering());
    println!("   Huge energy: {}", energy_huge.format_engineering());
    println!("   Stellar power: {}\n", power_stellar.format_engineering());

    println!("4. Astronomy Formatting:");
    println!("   Tiny distance: {}", tiny_distance.format_astronomy());
    println!("   Huge distance: {}", huge_distance.format_astronomy());
    println!("   Solar mass: {}", mass_solar.format_astronomy());
    println!("   Tiny time: {}", tiny_time.format_astronomy());
    println!("   Huge energy: {}", energy_huge.format_astronomy());
    println!("   Stellar power: {}\n", power_stellar.format_astronomy());

    println!("5. UI-Friendly Formatting:");
    println!("   Tiny distance: {}", tiny_distance.format_ui());
    println!("   Huge distance: {}", huge_distance.format_ui());
    println!("   Solar mass: {}", mass_solar.format_ui());
    println!("   Tiny time: {}", tiny_time.format_ui());
    println!("   Huge energy: {}", energy_huge.format_ui());
    println!("   Stellar power: {}\n", power_stellar.format_ui());

    println!("6. LaTeX Formatting:");
    println!("   Tiny distance: {}", tiny_distance.format_latex());
    println!("   Huge distance: {}", huge_distance.format_latex());
    println!("   Solar mass: {}", mass_solar.format_latex());
    println!("   Tiny time: {}", tiny_time.format_latex());
    println!("   Huge energy: {}", energy_huge.format_latex());
    println!("   Stellar power: {}\n", power_stellar.format_latex());

    println!("7. Debug Formatting:");
    println!("   Tiny distance: {}", tiny_distance.format_debug());
    println!("   Huge distance: {}", huge_distance.format_debug());
    println!("   Solar mass: {}", mass_solar.format_debug());
    println!("   Tiny time: {}", tiny_time.format_debug());
    println!("   Huge energy: {}", energy_huge.format_debug());
    println!("   Stellar power: {}\n", power_stellar.format_debug());

    // Custom formatters demo
    println!("8. Custom Formatters:");
    
    // Verbose formatter with unit names
    let verbose_formatter = QuantityFormatter::new(DisplayStyle::Verbose, FormattingContext::General);
    let verbose_formatter = QuantityFormatter {
        show_unit_name: true,
        ..verbose_formatter
    };
    
    println!("   Verbose: {}", tiny_distance.format_with(&verbose_formatter));
    
    // ASCII-only formatter (no Unicode)
    let ascii_formatter = QuantityFormatter::new(DisplayStyle::ASCII, FormattingContext::Engineering);
    println!("   ASCII: {}", power_stellar.format_with(&ascii_formatter));
    
    // Ultra-precise formatter
    let precise_formatter = QuantityFormatter {
        precision_formatter: PrecisionFormatter::fixed(8),
        ..QuantityFormatter::scientific()
    };
    println!("   Precise: {}", tiny_time.format_with(&precise_formatter));

    // Compact with auto unit selection
    let auto_formatter = QuantityFormatter {
        auto_unit_selection: true,
        ..QuantityFormatter::compact()
    };
    println!("   Auto units: {}", huge_distance.format_with(&auto_formatter));

    println!("\n9. Precision Control Demo:");
    let value = Distance::<Meter>::new(3.14159265359);
    
    // Different precision modes
    let fixed_2 = PrecisionFormatter::fixed(2);
    let sig_3 = PrecisionFormatter::significant(3);
    let auto = PrecisionFormatter::auto();
    let engineering = PrecisionFormatter::engineering();
    
    println!("   Original: {} m", value.value());
    println!("   Fixed(2): {} m", fixed_2.format(value.value()));
    println!("   Sig(3): {} m", sig_3.format(value.value()));
    println!("   Auto: {} m", auto.format(value.value()));
    println!("   Engineering: {} m", engineering.format(value.value()));

    println!("\n10. Scientific Notation Control:");
    let medium_value = Distance::<Meter>::new(12345.67);
    
    // Different scientific thresholds
    let conservative = ScientificFormatter::conservative();
    let standard = ScientificFormatter::standard();
    let aggressive = ScientificFormatter::aggressive();
    let always_sci = ScientificFormatter::always_scientific(3);
    
    println!("   Original: {} m", medium_value.value());
    println!("   Conservative: {} m", conservative.format(medium_value.value()));
    println!("   Standard: {} m", standard.format(medium_value.value()));
    println!("   Aggressive: {} m", aggressive.format(medium_value.value()));
    println!("   Always sci: {} m", always_sci.format(medium_value.value()));

    println!("\n11. Unit Selection Demo:");
    
    // Test different scales with smart unit selection
    let distances = vec![
        1e-12,     // pm scale
        1e-9,      // nm scale
        1e-6,      // μm scale
        1e-3,      // mm scale
        1.0,       // m scale
        1e3,       // km scale
        1.496e11,  // AU scale
        9.461e15,  // ly scale
    ];
    
    let selector = SmartUnitSelector::for_distance();
    println!("   Smart Distance Unit Selection:");
    for &dist in &distances {
        let (unit, value, _) = selector.select_best_unit(dist);
        println!("     {:.2e} m → {:.3} {}", dist, value, unit);
    }

    println!("\n12. Context-Aware Formatting:");
    
    let astronomical_distance = Distance::<Meter>::new(7.785e11); // Jupiter distance
    
    // Different contexts
    let general = UnitRecommendation::for_context("distance", "general");
    let astronomy = UnitRecommendation::for_context("distance", "astronomy");
    let engineering = UnitRecommendation::for_context("distance", "engineering");
    let microscopy = UnitRecommendation::for_context("distance", "microscopy");
    
    println!("   Jupiter distance in different contexts:");
    
    let (unit, value, _) = general.select_unit(astronomical_distance.value());
    println!("     General: {:.2} {}", value, unit);
    
    let (unit, value, _) = astronomy.select_unit(astronomical_distance.value());
    println!("     Astronomy: {:.2} {}", value, unit);
    
    let (unit, value, _) = engineering.select_unit(astronomical_distance.value());
    println!("     Engineering: {:.2} {}", value, unit);

    println!("\n13. Extreme Scale Demo:");
    
    // Test across enormous range
    let scales = vec![
        ("Planck length", 1.616e-35),
        ("Proton radius", 0.84e-15),
        ("Atomic radius", 0.53e-10),
        ("DNA width", 2.5e-9),
        ("Cell size", 10e-6),
        ("Human height", 1.8),
        ("Mt. Everest", 8849.0),
        ("Earth radius", 6.371e6),
        ("Sun radius", 6.96e8),
        ("Earth-Sun distance", 1.496e11),
        ("Solar system", 1e13),
        ("Nearest star", 4.0e16),
        ("Galaxy size", 9.5e20),
        ("Observable universe", 4.4e26),
    ];
    
    println!("   Extreme scale formatting:");
    for &(name, size) in &scales {
        let distance = Distance::<Meter>::new(size);
        let formatted = distance.format_astronomy();
        println!("     {:<20}: {}", name, formatted);
    }

    println!("\n14. Mixed Unit Arithmetic with Formatting:");
    
    let dist1 = Distance::<Kilometer>::new(5.2);
    let dist2 = Distance::<Meter>::new(1800.0);
    let total = dist1 + dist2;
    
    println!("   Distance arithmetic:");
    println!("     {} + {} = {}", dist1, dist2, total);
    println!("     Scientific: {}", total.format_scientific());
    println!("     Engineering: {}", total.format_engineering());

    let power1 = Power::<Watt>::new(1500.0);
    let power2 = Power::<Horsepower>::new(2.0);
    let total_power = power1 + power2;
    
    println!("   Power arithmetic:");
    println!("     {} + {} = {}", power1, power2, total_power);
    println!("     UI: {}", total_power.format_ui());

    println!("\n✅ Enhanced formatting system works perfectly!");
    println!("🎯 Scientific notation for extreme values");
    println!("📏 Precision control and significant figures");
    println!("🔄 Intelligent unit selection");
    println!("🎨 Multiple display styles and contexts");
    println!("🌌 Perfect for scientific applications!");
}