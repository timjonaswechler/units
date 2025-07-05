# Anwendungs Design
## Definition von Einheiten und Präfixen
 Eine Einfache SI Einheit wird wie folgt definiert
 ```rust
 let length = Distance::<Meter>::new(1.0);

 let length = Distance::<Prefixed<Kilo,Meter>>::new(1.0); // Kilometer

 ```

 Eine Eineit wie Geschwindigkeit wird wie folgt definiert
 ```rust
 let vel = Velocity::<Meter, Second>::new(1.0);

 let vel = Velocity::<Meter, Pre<Second>>::new(1.0);
 ```
in diesem und folgenden mehr komplexen Fällen soll es ebenfalls möglich sein, die Einheit mit einem Präfix zu versehen.

Für Beschleunigung
```rust
let acceleration = Acceleration::<MeterPerSecond>::new(1.0); // Meter pro Sekunde Quadrat
let acceleration = Acceleration::<Meter, Second>::new(1.0);
let acceleration = Acceleration::<Meter, Prefixed<Kilo,Second>>::new(1.0); // Meter pro Sekunde Quadrat
let acceleration = Acceleration::<Prefixed<Kilo, Meter>, Second>::new(1.0); // KiloMeter pro Sekunde Quadrat
let acceleration = Acceleration::<Prefixed<Kilo, Meter>, Prefixed<Kilo, Second>>::new(1.0); // KiloMeter pro KiloSekunde Quadrat
let acceleration = Acceleration::<Prefixed<Kilo, Meter>, Per<Prefixed<Kilo, Second>>>::new(1.0); // KiloMeter pro KiloSekunde Quadrat
let acceleration = Acceleration::<Prefixed<Kilo, Meter>, Per<Exponent<Prefixed<Kilo, Second>,2>>>::new(1.0); // KiloMeter pro KiloSekunde Quadrat
```

Das soll sich durch alle Weiteren Einheiten ziehen, wie z.B. Energie, Leistung, Druck, etc.

## Verrechnen von Einheiten
Addieren und Subtrahieren von Einheiten geht nur mit gleichen Einheiten.
also `Distance + Distance = Distance`
```rust
let d1 = Distance::<Meter>::new(1.0);
let d2 = Distance::<Meter>::new(2.0);
let d3 = d1 + d2; // Distance<Meter>

let d4 = d1 - d2; // Distance<Meter>

let d5 = d1 + Distance::<Prefixed<Kilo, Meter>>::new(2.0); // ebenfalls möglich
```

Multiplikation und Division von Einheiten ist ebenfalls möglich.
```rust
let d1 = Distance::<Meter>::new(1.0);
let d2 = Distance::<Meter>::new(2.0);
let d3 = d1 * d2; // Distance<Meter> * Distance<Meter> = Area<Meter> m^2
let d4 = d1 / d2; // Distance<Meter> / Distance<Meter> = Dimensionless

```
Same for Divisions,

A Unit it self is defined through a type Parameter
```rust
/// Represents physical dimensions using const generics for compile-time dimensional analysis.
///
/// This type encodes the seven fundamental SI dimensions as compile-time constants,
/// enabling automatic tracking of physical dimensions through calculations.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions<
    const L: i8, // Length
    const M: i8, // Mass
    const T: i8, // Time
    const K: i8, // Temperature
    const I: i8, // Current
    const J: i8, // Luminous Intensity
    const N: i8, // Amount of substance
>;


/// This is the core type that represents a physical quantity (like distance, mass, time)
/// with a specific unit and dimensional information tracked at compile time.
///
/// # Type Parameters
///
/// - `Unit`: The specific unit type (e.g., `Meter`, `AstronomicalUnit`, `Kilogram`)
/// - `L, M, T, K, I, J, N`: Dimensional exponents for the seven SI base dimensions
///
/// # Examples
///
/// ```rust
/// use star_sim::physics::units::*;
///
/// // Distance in astronomical units
/// let distance: Distance<AstronomicalUnit> = Distance::new(1.5);
///
/// // Mass in earth masses
/// let mass: Mass<EarthMass> = Mass::new(0.8);
///
/// // Convert between units
/// let distance_meters = distance.convert_to::<Meter>();
/// assert_eq!(distance_meters.value(), 1.5 * 149_597_870_700.0);
/// ```
///
/// # Dimensional Safety
///
/// The type system prevents mixing incompatible units:
///
/// ```compile_fail
/// let distance = Distance::<Meter>::new(100.0);
/// let mass = Mass::<Kilogram>::new(5.0);
/// let invalid = distance + mass; // Compile error!
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Quantity<
    Unit,
    const L: i8,
    const M: i8,
    const T: i8,
    const K: i8,
    const I: i8,
    const J: i8,
    const N: i8,
> {
    /// The numerical value of this quantity in the specified unit
    pub value: f64,
    /// Phantom data to track the unit type at compile time
    _unit: PhantomData<Unit>,
    /// Phantom data to track the dimensional information at compile time
    _dims: PhantomData<Dimensions<L, M, T, K, I, J, N>>,
}
```
if A Calculation accures with untis the get multiplied or divided
 the counts of the dimensions get added or subtracted, and the Resulting typ is the Match of the resulting dimensions.
 - Prefixed are the multiplication of the units value as the factor

 if the Variables didnt have a Type defined, the type will be inferred from the calculation.

 if the Type is not implicit selectebale because ther are multiple possible types, the compiler will throw an error, with a Message like:

```bash
error[E0282]: type annotations needed
 --> src/main.rs:10:5
  |10 |     let d3 = d1 * d2;
  |     ^^^^^^^^^^^^^^^^ cannot infer type for `d3`
  | help: consider giving `d3` an explicit type: there are multiple types that could be assigned to `d3`
  |     = note: multiple `Quantity` types match the resulting dimensions
  |     = note: the following types match the resulting dimensions:
  |     = note: `Quantity<Area<Meter>, 2, 0, 0, 0, 0, 0, 0>`
  | ...

```
  Or if the Compile didnt find a matching type:
```text
error[E0282]: type annotations needed
 --> src/main.rs:10:5
  |10 |     let d3 = d1 * d2;
  |     ^^^^^^^^^^^^^^^^ cannot infer type for `d3`
  | help: consider giving `d3` an explicit type: no matching type found for the resulting dimensions
  |     = note: no `Quantity` type matches the resulting dimensions
```

## Calculation with number Variables
If a Calculation is done with a number variable, the type of the variable will be used used as a factor or as the same type. this will only manipulate the value of the unit, not the type of the unit.
```rust
let d1 = Distance::<Meter>::new(1.0);
let d2 = 2.0; // f64
let d3 = d1 * d2; // Distance<Meter> * f64 = Distance<Meter>
// The value of d3 will be 2.0 with the Type Distance<Meter>

let d4 = d1 / d2; // Distance<Meter> / f64 = Distance<Meter>

let d5 = d2 + d1; // f64 + Distance<Meter> = Distance<Meter>
let d6 = d2 - d1; // f64 - Distance<Meter> = Distance<Meter>
```


## Implementation of Units
the untis are implemented  with macro_rules! to generate the types and the implementations for the units.
the following example shows an older version of the macro, which didnt fit the new requierement.
```rust
define_quantity!(Energy, 2, 1, -2, 0, 0, 0, 0); // Mass×Length²/Time²

// Define Energy units

define_unit_dimension! {
    dimension Energy {
        base_unit: Joule = 1.0,
        units: {
            Joule = 1.0,
            Erg = JOULES_PER_ERG,
            ElectronVolt = JOULES_PER_EV,
        },
        symbols: {
            Joule = "J",
            Erg = "erg",
            ElectronVolt = "eV",
        }
    }
}

define_quantity!(Energy, 2, 1, -2, 0, 0, 0, 0); // Mass×Length²/Time²

// Define Energy units

define_unit_dimension! {
    dimension Energy {
        base_unit: Joule = 1.0,
        units: {
            Joule = 1.0,
            Erg = JOULES_PER_ERG,
            ElectronVolt = JOULES_PER_EV,
        },
        symbols: {
            Joule = "J",
            Erg = "erg",
            ElectronVolt = "eV",
        }
    }
}
```

## **Modularer Implementierungsplan: Dateistruktur & Kleine Handhabbare Dateien**
Basierend auf dem Code-Scan erstelle ich jetzt einen strukturierten Plan:
## **🗂️ Neue Dateistruktur (Klein & Modular)**
### **Phase 1: Grundlegende Infrastruktur**
```
src/
├── core/                          # ✅ Bereits vorhanden
│   ├── mod.rs                     # Haupttypen reorganisieren
│   ├── quantity.rs                # Nur Quantity<> Typ
│   ├── dimensions.rs              # Nur Dimensions<> Typ
│   └── traits.rs                  # ToSI, FromSI, UnitSymbol
├── variadic/                      # 🔄 Neu organisieren
│   ├── mod.rs                     # Haupteinstiegspunkt
│   ├── composition.rs             # 🔄 Bestehende Datei erweitern
│   └── unit_factors.rs            # ❌ NEU: Einheitenfaktor-Berechnungen
├── arithmetic/                    # 🔄 Erweitern
│   ├── mod.rs                     # Organisieren
│   ├── same_units.rs              # ✅ Bereits implementiert
│   ├── mixed_units.rs             # ❌ NEU: Verschiedene Einheiten
│   ├── scalar_ops.rs              # ✅ Bereits implementiert
│   └── dimensional_inference.rs   # ❌ NEU: Automatische Typableitung
├── mod.rs
├── per_syntax_tests.rs
├── exponent_tests.rs
├── mixed_arithmetic_tests.rs
└── integration_tests.rs
```
## **🎯 Implementierungsschritte (Klein & Handhabbar)**
### **Schritt 3: Einheitenfaktor-Berechnungen**
**Datei:** `src/variadic/unit_factors.rs` (neu)
- [ ] Trait für Faktor-Berechnung
- [ ] Implementierung für `Prefixed<>`
### **Schritt 4: Variadic Integration**
**Datei:** `src/variadic/mod.rs` (erweitern)
- [ ] Per<> und Exponent<> in Variadic-System integrieren
- [ ] ToSI/FromSI für zusammengesetzte Einheiten
- [ ] Tests für Design.md Beispiele
### **Schritt 5: Gemischte Arithmetik**
**Datei:** `src/arithmetic/mixed_units.rs` (neu)
- [ ] Addition verschiedener Einheiten mit Konvertierung
- [ ] Subtraktion verschiedener Einheiten mit Konvertierung
- [ ] Tests für `Meter + Prefixed<Kilo, Meter>`
## **🔍 Detaillierter Erster Schritt: Per<> Syntax**
Lass mich mit dem ersten konkreten Schritt beginnen - der `Per<>` Syntax:
### **Ziel:** `Velocity::<Meter, Per<Second>>::new(1.0)` funktionsfähig machen
### **Implementierung:**
1. **Neue Datei erstellen:** `src/variadic/per.rs`
2. **Per<> Typ definieren**
3. **UnitSymbol implementieren** (z.B. \"s\" → \"/s\")
4. **Unit-Faktor Berechnung** (z.B. 1/value)
5. **Tests schreiben**
### **Abhängigkeiten prüfen:**
Vor der Implementierung prüfe ich:
- ✅ Bereits vorhanden: `variadic_syntax.rs` (Grundlage)
- ✅ Bereits vorhanden: `UnitSymbol` trait
- ❌ Fehlend: Proper `Per<>` integration
Soll ich mit dem ersten Schritt beginnen - der `Per<>` Syntax Implementierung?
