# Implementation Roadmap: Automatic Physics Engine

## 🎯 Ziel-Vision (Erinnerung)
```rust
let force = unit!(Force, Newton, 100);
let area = unit!(Area, Exponent<Meter,2>, 4);
let pressure = force / area; // Compiler erkennt: Force/Area = Pressure automatisch!

let work = unit!(Work, (Newton,Meter), 1000); // Compound units
let power = work / unit!(Time, Second, 10);    // Automatisch: Work/Time = Power
```

---

# 🗺️ **Implementierungsplan - 7 Phasen**

## **Phase 1: Foundations - Enhanced Dimensional System** 
*Geschätzte Zeit: 2-3 Tage*

### 1.1 **Core Dimension Refactor**
```rust
// Aktuelle Implementierung ersetzen
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dimension {
    pub data: [i8; 7], // [L, T, M, I, Θ, J, N]
}

impl Dimension {
    pub const fn new(l: i8, t: i8, m: i8, i: i8, temp: i8, lum: i8, amount: i8) -> Self {
        Self { data: [l, t, m, i, temp, lum, amount] }
    }
    
    // Const arithmetic für compile-time
    pub const fn multiply(self, other: Self) -> Self {
        Self { data: [
            self.data[0] + other.data[0], // L
            self.data[1] + other.data[1], // T
            self.data[2] + other.data[2], // M
            self.data[3] + other.data[3], // I
            self.data[4] + other.data[4], // Θ
            self.data[5] + other.data[5], // J
            self.data[6] + other.data[6], // N
        ]}
    }
    
    pub const fn divide(self, other: Self) -> Self {
        Self { data: [
            self.data[0] - other.data[0],
            self.data[1] - other.data[1],
            self.data[2] - other.data[2],
            self.data[3] - other.data[3],
            self.data[4] - other.data[4],
            self.data[5] - other.data[5],
            self.data[6] - other.data[6],
        ]}
    }
    
    pub const fn power(self, exp: i8) -> Self {
        Self { data: [
            self.data[0] * exp,
            self.data[1] * exp,
            self.data[2] * exp,
            self.data[3] * exp,
            self.data[4] * exp,
            self.data[5] * exp,
            self.data[6] * exp,
        ]}
    }
    
    // Human-readable representation für errors
    pub fn display(&self) -> String {
        format!("[L:{} T:{} M:{} I:{} Θ:{} J:{} N:{}]", 
                self.data[0], self.data[1], self.data[2], 
                self.data[3], self.data[4], self.data[5], self.data[6])
    }
}
```

### 1.2 **Quantity Trait Enhancement**
```rust
pub trait Quantity: 'static + Copy + Clone {
    const DIMENSION: Dimension;
    const NAME: &'static str;
    
    // Für Registry lookup
    fn dimension() -> Dimension { Self::DIMENSION }
    fn name() -> &'static str { Self::NAME }
}

// Alle Quantities updaten
impl Quantity for Distance {
    const DIMENSION: Dimension = Dimension::new(1, 0, 0, 0, 0, 0, 0);
    const NAME: &'static str = "Distance";
}

impl Quantity for Force {
    const DIMENSION: Dimension = Dimension::new(1, -2, 1, 0, 0, 0, 0);
    const NAME: &'static str = "Force";
}

impl Quantity for Pressure {
    const DIMENSION: Dimension = Dimension::new(-1, -2, 1, 0, 0, 0, 0);
    const NAME: &'static str = "Pressure";
}
// ... weitere Quantities
```

### 1.3 **Tests für Phase 1**
```rust
#[test]
fn test_dimensional_arithmetic() {
    let force_dim = Force::DIMENSION;
    let area_dim = Area::DIMENSION;
    let pressure_dim = force_dim.divide(area_dim);
    assert_eq!(pressure_dim, Pressure::DIMENSION);
}
```

---

## **Phase 2: Registry System** 
*Geschätzte Zeit: 1-2 Tage*

### 2.1 **Compile-Time Dimension Registry**
```rust
// Global registry aller Dimension → Quantity mappings
pub struct QuantityRegistry;

impl QuantityRegistry {
    // Const array für alle bekannten Quantities
    pub const KNOWN_QUANTITIES: &'static [(Dimension, &'static str)] = &[
        (Distance::DIMENSION, "Distance"),
        (Time::DIMENSION, "Time"),
        (Mass::DIMENSION, "Mass"),
        (Force::DIMENSION, "Force"),
        (Pressure::DIMENSION, "Pressure"),
        (Energy::DIMENSION, "Energy"),
        (Power::DIMENSION, "Power"),
        (Area::DIMENSION, "Area"),
        (Volume::DIMENSION, "Volume"),
        (Speed::DIMENSION, "Speed"),
        (Acceleration::DIMENSION, "Acceleration"),
        // ... erweitert sich automatisch
    ];
    
    // Lookup function für proc macros
    pub const fn lookup_quantity_name(dimension: Dimension) -> Option<&'static str> {
        let mut i = 0;
        while i < Self::KNOWN_QUANTITIES.len() {
            if Self::KNOWN_QUANTITIES[i].0.data[0] == dimension.data[0] &&
               Self::KNOWN_QUANTITIES[i].0.data[1] == dimension.data[1] &&
               Self::KNOWN_QUANTITIES[i].0.data[2] == dimension.data[2] &&
               Self::KNOWN_QUANTITIES[i].0.data[3] == dimension.data[3] &&
               Self::KNOWN_QUANTITIES[i].0.data[4] == dimension.data[4] &&
               Self::KNOWN_QUANTITIES[i].0.data[5] == dimension.data[5] &&
               Self::KNOWN_QUANTITIES[i].0.data[6] == dimension.data[6] {
                return Some(Self::KNOWN_QUANTITIES[i].1);
            }
            i += 1;
        }
        None
    }
}
```

### 2.2 **Registry Auto-Generation Macro**
```rust
// Macro um automatisch Registry zu erweitern
macro_rules! register_quantity {
    ($quantity:ty) => {
        // Zur Compile-Time: Registry erweitern
        // Implementation details...
    };
}

// Usage:
register_quantity!(MyCustomQuantity);
```

### 2.3 **Tests für Phase 2**
```rust
#[test]
fn test_registry_lookup() {
    let pressure_dim = Dimension::new(-1, -2, 1, 0, 0, 0, 0);
    assert_eq!(
        QuantityRegistry::lookup_quantity_name(pressure_dim),
        Some("Pressure")
    );
}
```

---

## **Phase 3: Expression Parser for Proc Macros** 
*Geschätzte Zeit: 3-4 Tage*

### 3.1 **AST für Unit Expressions**
```rust
// AST für Ausdrücke wie: force / area, (work1 + work2) / time
#[derive(Debug, Clone)]
pub enum UnitExpr {
    Variable(syn::Ident),                    // force
    Binary(Box<UnitExpr>, BinOp, Box<UnitExpr>), // force / area
    Grouped(Box<UnitExpr>),                  // (expr)
    FunctionCall(syn::Ident, Vec<UnitExpr>), // unit!(Force, Newton, 100)
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
}
```

### 3.2 **Expression Parser**
```rust
// Parser für: force / area
pub fn parse_unit_expression(input: TokenStream) -> syn::Result<UnitExpr> {
    // Implementierung mit syn::parse
    // Precedence parsing für +, -, *, /
    // Support für Gruppierung mit ()
}
```

### 3.3 **Dimensional Analysis Engine**
```rust
pub struct DimensionalAnalyzer {
    // Context: welche Variablen haben welche Dimensionen
    variable_dimensions: HashMap<String, Dimension>,
}

impl DimensionalAnalyzer {
    pub fn analyze_expression(&self, expr: &UnitExpr) -> syn::Result<Dimension> {
        match expr {
            UnitExpr::Variable(name) => {
                self.variable_dimensions.get(&name.to_string())
                    .copied()
                    .ok_or_else(|| syn::Error::new_spanned(name, "Unknown variable"))
            }
            UnitExpr::Binary(left, op, right) => {
                let left_dim = self.analyze_expression(left)?;
                let right_dim = self.analyze_expression(right)?;
                
                match op {
                    BinOp::Add | BinOp::Sub => {
                        if left_dim != right_dim {
                            return Err(syn::Error::new(
                                proc_macro2::Span::call_site(),
                                format!("Cannot {} {} and {}: dimensional mismatch", 
                                       if matches!(op, BinOp::Add) { "add" } else { "subtract" },
                                       left_dim.display(), right_dim.display())
                            ));
                        }
                        Ok(left_dim)
                    }
                    BinOp::Mul => Ok(left_dim.multiply(right_dim)),
                    BinOp::Div => Ok(left_dim.divide(right_dim)),
                }
            }
            UnitExpr::Grouped(inner) => self.analyze_expression(inner),
            UnitExpr::FunctionCall(..) => {
                // Handle unit!() calls
                todo!("Function call analysis")
            }
        }
    }
}
```

### 3.4 **Tests für Phase 3**
```rust
#[test]
fn test_expression_parsing() {
    let expr = parse_unit_expression(quote! { force / area }).unwrap();
    // Verify AST structure
}

#[test]
fn test_dimensional_analysis() {
    let mut analyzer = DimensionalAnalyzer::new();
    analyzer.variable_dimensions.insert("force".to_string(), Force::DIMENSION);
    analyzer.variable_dimensions.insert("area".to_string(), Area::DIMENSION);
    
    let expr = parse_unit_expression(quote! { force / area }).unwrap();
    let result_dim = analyzer.analyze_expression(&expr).unwrap();
    assert_eq!(result_dim, Pressure::DIMENSION);
}
```

---

## **Phase 4: Enhanced Operator Overloading** 
*Geschätzte Zeit: 2-3 Tage*

### 4.1 **Smart Division with Type Deduction**
```rust
// Statt manuelle Implementierungen für jede Kombination:
impl<Q1, Q2, U1, U2, T> Div<Value<Q2, U2, T>> for Value<Q1, U1, T>
where
    Q1: Quantity,
    Q2: Quantity,
    U1: Unit,
    U2: Unit,
    T: Copy + Div<Output = T>,
{
    type Output = Value<
        // Hier ist der Trick: Macro generiert zur Compile-Time den richtigen Typ
        AutoResolvedQuantity<DivDimension<Q1, Q2>>,
        DivUnit<U1, U2>,
        T
    >;
    
    fn div(self, rhs: Value<Q2, U2, T>) -> Self::Output {
        let result_value = self.value / rhs.value;
        let result_scale = U1::SCALE / U2::SCALE;
        Value::new(result_value * result_scale)
    }
}

// AutoResolvedQuantity wird durch Proc Macro zur Compile-Time aufgelöst
```

### 4.2 **Proc Macro für Operator Resolution**
```rust
// Macro das zur Compile-Time die richtigen Typen generiert
#[proc_macro]
pub fn resolve_division_type(input: TokenStream) -> TokenStream {
    // Input: DivDimension<Force, Area>
    // Output: Pressure (nach Registry lookup)
}
```

### 4.3 **Tests für Phase 4**
```rust
#[test]
fn test_automatic_division() {
    let force = Value::<Force, Newton>::new(100.0);
    let area = Value::<Area, SquareMeter>::new(4.0);
    let pressure = force / area;
    
    // Compiler soll automatisch Pressure als Typ deuten
    assert_eq!(pressure.si(), 25.0); // 25 Pascal
}
```

---

## **Phase 5: unit!() Macro System** 
*Geschätzte Zeit: 3-4 Tage*

### 5.1 **Basic unit!() Macro**
```rust
#[proc_macro]
pub fn unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as UnitMacroInput);
    
    match input {
        // unit!(Force, Newton, 100)
        UnitMacroInput::Simple { quantity, unit, value } => {
            generate_simple_unit(quantity, unit, value)
        }
        
        // unit!(Work, (Newton, Meter), 1000)
        UnitMacroInput::Compound { quantity, units, value } => {
            generate_compound_unit(quantity, units, value)
        }
        
        // unit!(Pressure, KiloPascal, force / area)
        UnitMacroInput::Conversion { quantity, target_unit, expression } => {
            generate_conversion_unit(quantity, target_unit, expression)
        }
    }
}
```

### 5.2 **Expression-Based unit!() **
```rust
// unit!(Pressure, KiloPascal, force / area)
fn generate_conversion_unit(
    quantity: syn::Type,
    target_unit: syn::Type, 
    expression: UnitExpr
) -> TokenStream {
    // 1. Analysiere expression dimensional
    let expr_dimension = analyze_expression_dimension(&expression)?;
    
    // 2. Prüfe Kompatibilität mit quantity
    validate_quantity_compatibility(&quantity, expr_dimension)?;
    
    // 3. Generiere Konvertierung
    let conversion_code = generate_unit_conversion(&target_unit, &expression);
    
    quote! {
        {
            let source_value = #expression;
            let converted_value = #conversion_code;
            Value::<#quantity, #target_unit>::new(converted_value)
        }
    }
}
```

### 5.3 **Tests für Phase 5**
```rust
#[test]
fn test_unit_macro_basic() {
    let force = unit!(Force, Newton, 100);
    assert_eq!(force.value(), 100.0);
}

#[test] 
fn test_unit_macro_conversion() {
    let force = unit!(Force, Newton, 100);
    let area = unit!(Area, Exponent<Meter,2>, 4);
    let pressure = unit!(Pressure, KiloPascal, force / area);
    
    assert_eq!(pressure.value(), 0.025); // 25 Pascal = 0.025 KiloPascal
}
```

---

## **Phase 6: Advanced Features** 
*Geschätzte Zeit: 2-3 Tage*

### 6.1 **Nested Prefixes**
```rust
// unit!(Energy, Prefix<Mega, Prefix<Kilo, Joule>>, 1000) = GigaJoule
impl<P1: Prefix, P2: Prefix, U: Unit> Unit for Prefixed<P1, Prefixed<P2, U>> {
    type Dimension = U::Dimension;
    const SCALE: f64 = P1::FACTOR * P2::FACTOR * U::SCALE;
    const NAME: &'static str = "nested prefixed unit";
    const SYMBOL: &'static str = "nested";
}
```

### 6.2 **Complex Compound Units**
```rust
// unit!(Work, (Kilogram, Exponent<Meter,2>, Per<Exponent<Second,2>>), 1000)
// Automatische Vereinfachung zu Joule
```

### 6.3 **Custom Quantities**
```rust
// Macro für eigene Quantities
define_quantity!(ElectricResistance, [-2, 3, 1, -2, 0, 0, 0]); // Ω = kg⋅m²⋅s⁻³⋅A⁻²
```

---

## **Phase 7: Polish & Integration** 
*Geschätzte Zeit: 2-3 Tage*

### 7.1 **Error Messages**
```rust
// Gute Compile-Time Fehlermeldungen:
// "Cannot convert Force [M L T^-2] to Distance [L]: dimensional mismatch"
// "Unknown quantity for dimension [M L^-1 T^-2]: consider defining a custom quantity"
```

### 7.2 **Documentation & Examples**
```rust
/// # Examples
/// ```
/// let force = unit!(Force, Newton, 100);
/// let area = unit!(Area, Exponent<Meter,2>, 4);  
/// let pressure = force / area; // Automatically resolves to Pressure
/// ```
```

### 7.3 **Performance Benchmarks**
```rust
// Sicherstellen dass Zero-Cost Abstraction funktioniert
#[bench]
fn bench_unit_arithmetic(b: &mut Bencher) {
    // ...
}
```

---

# 🎯 **Erfolgskriterien**

## **Funktionale Ziele:**
- ✅ `let pressure = force / area;` funktioniert mit automatischer Typ-Deduktion
- ✅ `unit!(Pressure, KiloPascal, force/area)` mit automatischer Konvertierung
- ✅ Komplexe Ausdrücke: `(force1 + force2) / (area1 * 2.0)`
- ✅ Nested prefixes: `Prefix<Mega,Prefix<Kilo,Gram>>`
- ✅ Compile-time dimensionale Validierung

## **Non-funktionale Ziele:**
- ✅ Zero-cost runtime performance
- ✅ Gute Compile-Time Fehlermeldungen
- ✅ Saubere, erweiterbare Architektur
- ✅ Umfangreiche Tests

## **Meilensteine:**
1. **Week 1**: Phases 1-3 (Foundations, Registry, Parser)
2. **Week 2**: Phases 4-5 (Operators, Macros)  
3. **Week 3**: Phases 6-7 (Advanced Features, Polish)

---

**Was denkst du über diesen detaillierten Plan? Sehen die Phasen machbar aus? Gibt es Bereiche die wir noch genauer planen sollten?** 🚀