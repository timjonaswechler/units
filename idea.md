# Unit-System

Die Idee des crates ist folgende:
- Jede Phyiskliasche Größe  hat eine eindeutige Dimensions - Signatur. Unter der Dimension versteht man das es eine Kombination aus den SI-Größen in Form eines Arrasy ist. Dieser Array beinhaltet die Potenzen der SI-Größen. Als Beispiel ist die Dimension der Geschwindigkeit [1, 0, -1, 0, 0, 0] wobei die Indizes für die SI-Größen wie folgt belegt sind.
- Mit diesem Array kann man für die Arithmetik von Einheiten direkt sagen ob eine Berechnung die Richtige Einheiten beinhaltet und ergibt wie man angegeben hatte.
- Für Jede Physikaische Größe gibt es wiederum Einheiten. Diese Einheiten sind alleinstehende Structs die die Dimension der Physikalischen Größe beinhalten,
einen Faktor der die Einheit in Bezug auf die SI-Einheit angibt, einen Faktor der den Prefix wie Kilo oder Milli angibt und einen Namen der Einheit, wie auch das Symbol der Einheit.
- Zu den Einheiten und Größen gibt es dann auch noch Operationen, Per, Exponent und Prefix. Mit diesen Operation sollen die Einheiten zu komplexe Einheiten kombiniert werden können.

## Syntax
- Per<Unit> : dreht die Vorzeichen der Werte der Dimension um.
- Exponent<Unit, i32> : addiert die Dimensionswerte der angegebenen Einheit mit dem Exponenten.
- Prefix<Unit, Prefix> : ändert den Faktor der Einheit.

### Definition von Variablen
Definitionen von Variablen und die Anwendung des Crates erfolgt über Rust Macros.
es gibt zum einen das Macro
- `define_unit!(<Gewünschte Physikalische Größe>, <Name der Einheit>, <Faktor in Bezug auf Si-Größe>)`,
- `define_prefix!(<Name>, <Faktor>)`
- `define_quantity!(<Name>,<Dimensionssignatur als Array>)`.

### Die Anwendung

```rust
// Prefixed units
Distance<Prefixed<Kilo, Meter>>     // Kilometer
Power<Prefixed<Mega, Watt>>         // Megawatt

// Compound units
let a: Velocity<(Meter, Per<Second>)> = 1;      // 1 m/s
Acceleration<(Meter, Per<Exponent<Second, 2>>)>  // m/s²
```

Der Wunsch wäre das man so jede Einheit, jede Naturkonstante und jede Berechnung Typensicher durchführen kann.

Probleme die es aber hier direkt gibt ist die behandlung von Temperaturen. Die Handhabe der unterschiedlichen Temperaturen biergt durch die Offsets (Kelvin u. Grad Celsius) und durch verrechnung zweier Temperaturen die als Absolut angegeben wurden man aber die Relative Temperatur auf summieren will ihre tücken. wenn man 10 °C + 20°C verrechnet kommt 30 °C heraus.. wenn man das aber in SI intern verrechnen würde kommt ein ganz anderer WErt dabei heraus da 10+273.15 + 20+273.15 nicht 30+273.15 ergeben. Diese Sonder handlungen müssten intelligent integiert werden.



- Die Anwendung des  crates erwartet das die Einheiten Und Physikalischen Größen die an nutzen will immer definiert sind.
## Bei einfachen Berechnungen
unter einfache Berechnung versteht man hier:
zwei oder mehrere Einheiten die addiert, subtrahiert, multipliziert oder dividiert werden `a + b * c / d - e`.
### Addieren und Subtrahieren
In diesem Fall von  Subrahieren und addieren wird geprüft ob die Dimensionen der Einheiten gleich sind.
Falls die Signatur stimmt werden die Werte der Einheiten verrechnet. Jede Einheit hat einen Value und einen SI-Value.
Die Werte die verechnet werden sind die SI-Werte. die anderen Werte werden danach aktualisiert, Falls vorhanden oder notwendig wird ein Prefix auf das Ergebnis angewendet/hinzugefügt.
mann kann auch die einzlenen Werte der Einheiten durch die Funktionen der Einheiten abfragen und diese dann mit einander verrechnen. Dabei ist dann aber die Verantwortung des Nutzers zu prüfen ob die Einheiten kompatibel sind und die prüfung ob das Ergebnis einen Prefix benötigt oder nicht.
### Multiplikation und Division
In diesem Fall von Multiplikation und Division wird geprüft ob die Dimensionen der Einheiten kompatibel sind, im Sinne von ist eine Dimension mit der Signatur die am ende der Berechnung herauskommt registriert.
Falls die Signatur stimmt werden die Werte der Einheiten verrechnet. Jede Einheit hat einen Value und einen SI-Value.
Die Werte die verrechnet werden sind die SI-Werte. Die anderen Werte werden danach aktualisiert, falls vorhanden oder notwendig wird ein Prefix auf das Ergebnis angewendet/hinzugefügt.
Die Einheiten die am Ende herauskommen sind dann eine neue Einheit mit der entsprechenden Dimension.
(evtl.) wird auch vom User werwartet das er die Einheit des Ergbnisses definiert hat bevor das ergebnis zurückgegeben wird.
Die Dimensionen des Ergebnisses werden aus den Dimensionen der Einheiten die multipliziert oder dividiert wurden abgeleitet.
