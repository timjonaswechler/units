use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{parse::Parse, Ident, Token};
use std::collections::HashMap;
use crate::{Dimension, QuantityRegistry};

/// AST für Unit-Ausdrücke wie `force / area`, `(work1 + work2) / time`
/// 
/// Dieses AST wird von Proc Macros verwendet um physikalische Ausdrücke
/// zur Compile-Time zu parsen und dimensional zu analysieren.
#[derive(Debug, Clone)]
pub enum UnitExpr {
    /// Variable reference: `force`
    Variable(Ident),
    
    /// Binary operation: `force / area`, `energy + work`
    Binary(Box<UnitExpr>, BinOp, Box<UnitExpr>),
    
    /// Grouped expression: `(force + gravity)`
    Grouped(Box<UnitExpr>),
    
    /// Function call: `unit!(Force, Newton, 100)`
    FunctionCall(Ident, Vec<UnitExpr>),
}

/// Binary operators für physikalische Ausdrücke
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    /// Addition: `force1 + force2` (gleiche Dimensionen erforderlich)
    Add,
    
    /// Subtraktion: `energy1 - energy2` (gleiche Dimensionen erforderlich)  
    Sub,
    
    /// Multiplikation: `force * distance` (Dimensionen werden multipliziert)
    Mul,
    
    /// Division: `force / area` (Dimensionen werden dividiert)
    Div,
}

impl BinOp {
    /// Gibt das entsprechende Token für diesen Operator zurück
    pub fn to_token(&self) -> TokenStream {
        match self {
            BinOp::Add => quote::quote!(+),
            BinOp::Sub => quote::quote!(-),
            BinOp::Mul => quote::quote!(*),
            BinOp::Div => quote::quote!(/),
        }
    }
    
    /// Gibt die Operator-Precedence zurück (für Parsing)
    pub fn precedence(&self) -> u8 {
        match self {
            BinOp::Add | BinOp::Sub => 1,
            BinOp::Mul | BinOp::Div => 2,
        }
    }
}

/// Parser für Unit-Ausdrücke
/// 
/// Implementiert vereinfachtes Parsing für mathematische Operationen
/// mit physikalischen Einheiten.
pub struct UnitExprParser;

impl UnitExprParser {
    /// Parst einen Unit-Ausdruck aus einem TokenStream
    /// 
    /// # Beispiel
    /// ```ignore
    /// let expr = UnitExprParser::parse(quote! { force / area })?;
    /// ```
    pub fn parse(input: TokenStream) -> syn::Result<UnitExpr> {
        // Vereinfachte Implementation - parst nur einfache binäre Ausdrücke
        let tokens: Vec<_> = input.into_iter().collect();
        
        if tokens.len() == 1 {
            // Single variable
            if let proc_macro2::TokenTree::Ident(ident) = &tokens[0] {
                let syn_ident = Ident::new(&ident.to_string(), ident.span());
                return Ok(UnitExpr::Variable(syn_ident));
            }
        } else if tokens.len() == 3 {
            // Binary expression: left op right
            if let (
                proc_macro2::TokenTree::Ident(left),
                proc_macro2::TokenTree::Punct(op),
                proc_macro2::TokenTree::Ident(right),
            ) = (&tokens[0], &tokens[1], &tokens[2])
            {
                let left_ident = Ident::new(&left.to_string(), left.span());
                let right_ident = Ident::new(&right.to_string(), right.span());
                
                let bin_op = match op.as_char() {
                    '+' => BinOp::Add,
                    '-' => BinOp::Sub,
                    '*' => BinOp::Mul,
                    '/' => BinOp::Div,
                    _ => return Err(syn::Error::new(op.span(), "Unsupported operator")),
                };
                
                return Ok(UnitExpr::Binary(
                    Box::new(UnitExpr::Variable(left_ident)),
                    bin_op,
                    Box::new(UnitExpr::Variable(right_ident)),
                ));
            }
        }
        
        Err(syn::Error::new(
            Span::call_site(),
            "Unsupported expression format. Only simple binary expressions supported for now.",
        ))
    }
}

/// Dimensional Analysis Engine
/// 
/// Analysiert Unit-Ausdrücke und berechnet die resultierenden Dimensionen.
/// Dies ist der Kern für die automatische Typ-Deduktion.
pub struct DimensionalAnalyzer {
    /// Context: Variable → Dimension mapping
    variable_dimensions: HashMap<String, Dimension>,
}

impl DimensionalAnalyzer {
    /// Erstellt einen neuen Analyzer
    pub fn new() -> Self {
        Self {
            variable_dimensions: HashMap::new(),
        }
    }
    
    /// Fügt eine Variable mit ihrer Dimension hinzu
    pub fn add_variable(&mut self, name: String, dimension: Dimension) {
        self.variable_dimensions.insert(name, dimension);
    }
    
    /// Analysiert einen Ausdruck und berechnet die resultierende Dimension
    /// 
    /// # Beispiel
    /// ```ignore
    /// let mut analyzer = DimensionalAnalyzer::new();
    /// analyzer.add_variable("force".to_string(), Dimension::FORCE);
    /// analyzer.add_variable("area".to_string(), Dimension::AREA);
    /// 
    /// let expr = UnitExprParser::parse(quote! { force / area })?;
    /// let result_dim = analyzer.analyze_expression(&expr)?;
    /// 
    /// assert_eq!(result_dim, Dimension::PRESSURE);
    /// ```
    pub fn analyze_expression(&self, expr: &UnitExpr) -> syn::Result<Dimension> {
        match expr {
            UnitExpr::Variable(name) => {
                self.variable_dimensions
                    .get(&name.to_string())
                    .copied()
                    .ok_or_else(|| {
                        syn::Error::new_spanned(
                            name,
                            format!("Unknown variable: {}", name),
                        )
                    })
            }
            
            UnitExpr::Binary(left, op, right) => {
                let left_dim = self.analyze_expression(left)?;
                let right_dim = self.analyze_expression(right)?;
                
                match op {
                    BinOp::Add | BinOp::Sub => {
                        // Addition/Subtraktion: Dimensionen müssen gleich sein
                        if left_dim != right_dim {
                            return Err(syn::Error::new(
                                Span::call_site(),
                                format!(
                                    "Cannot {} {} and {}: dimensional mismatch\nLeft:  {}\nRight: {}",
                                    if matches!(op, BinOp::Add) { "add" } else { "subtract" },
                                    self.guess_quantity_name(left_dim),
                                    self.guess_quantity_name(right_dim),
                                    left_dim.display(),
                                    right_dim.display()
                                ),
                            ));
                        }
                        Ok(left_dim)
                    }
                    
                    BinOp::Mul => {
                        // Multiplikation: Dimensionen werden multipliziert
                        Ok(left_dim.multiply(right_dim))
                    }
                    
                    BinOp::Div => {
                        // Division: Dimensionen werden dividiert
                        Ok(left_dim.divide(right_dim))
                    }
                }
            }
            
            UnitExpr::Grouped(inner) => {
                // Gruppierte Ausdrücke: Dimension des inneren Ausdrucks
                self.analyze_expression(inner)
            }
            
            UnitExpr::FunctionCall(name, args) => {
                // Function calls: Implementierung abhängig von der Funktion
                match name.to_string().as_str() {
                    "unit" => {
                        // unit!(Quantity, Unit, Value) - nimmt Dimension vom Quantity
                        if args.is_empty() {
                            return Err(syn::Error::new_spanned(
                                name,
                                "unit!() macro requires at least quantity argument",
                            ));
                        }
                        
                        // Für unit!() calls müssten wir die Quantity-Dimension extrahieren
                        // Das ist komplex und würde weitere Parsing-Logik erfordern
                        // Für jetzt geben wir einen Fehler zurück
                        Err(syn::Error::new_spanned(
                            name,
                            "Function call analysis not yet implemented",
                        ))
                    }
                    
                    _ => Err(syn::Error::new_spanned(
                        name,
                        format!("Unknown function: {}", name),
                    )),
                }
            }
        }
    }
    
    /// Versucht den Quantity-Namen für eine Dimension zu erraten (für Fehlermeldungen)
    fn guess_quantity_name(&self, dimension: Dimension) -> String {
        QuantityRegistry::lookup_quantity_name(dimension)
            .unwrap_or("UnknownQuantity")
            .to_string()
    }
    
    /// Analysiert einen Ausdruck und resolved den Quantity-Namen
    pub fn resolve_quantity_name(&self, expr: &UnitExpr) -> syn::Result<String> {
        let dimension = self.analyze_expression(expr)?;
        
        QuantityRegistry::lookup_quantity_name(dimension)
            .map(|name| name.to_string())
            .ok_or_else(|| {
                syn::Error::new(
                    Span::call_site(),
                    format!(
                        "No known quantity for dimension {}: consider defining a custom quantity",
                        dimension.display()
                    ),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_expression_parsing_simple() {
        let expr = UnitExprParser::parse(quote! { force }).unwrap();
        
        match expr {
            UnitExpr::Variable(ident) => assert_eq!(ident.to_string(), "force"),
            _ => panic!("Expected variable"),
        }
    }
    
    #[test]
    fn test_expression_parsing_binary() {
        let expr = UnitExprParser::parse(quote! { force / area }).unwrap();
        
        match expr {
            UnitExpr::Binary(left, op, right) => {
                assert!(matches!(*left, UnitExpr::Variable(_)));
                assert_eq!(op, BinOp::Div);
                assert!(matches!(*right, UnitExpr::Variable(_)));
            }
            _ => panic!("Expected binary expression"),
        }
    }
    
    #[test]
    fn test_dimensional_analysis() {
        let mut analyzer = DimensionalAnalyzer::new();
        analyzer.add_variable("force".to_string(), Dimension::FORCE);
        analyzer.add_variable("area".to_string(), Dimension::AREA);
        
        let expr = UnitExprParser::parse(quote! { force / area }).unwrap();
        let result_dim = analyzer.analyze_expression(&expr).unwrap();
        
        assert_eq!(result_dim, Dimension::PRESSURE);
    }
    
    #[test]
    fn test_dimensional_analysis_multiplication() {
        let mut analyzer = DimensionalAnalyzer::new();
        analyzer.add_variable("force".to_string(), Dimension::FORCE);
        analyzer.add_variable("distance".to_string(), Dimension::length());
        
        let expr = UnitExprParser::parse(quote! { force * distance }).unwrap();
        let result_dim = analyzer.analyze_expression(&expr).unwrap();
        
        assert_eq!(result_dim, Dimension::ENERGY);
    }
    
    #[test]
    fn test_dimensional_analysis_error() {
        let mut analyzer = DimensionalAnalyzer::new();
        analyzer.add_variable("force".to_string(), Dimension::FORCE);
        analyzer.add_variable("time".to_string(), Dimension::time());
        
        let expr = UnitExprParser::parse(quote! { force + time }).unwrap();
        let result = analyzer.analyze_expression(&expr);
        
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("dimensional mismatch"));
    }
    
    #[test]
    fn test_quantity_name_resolution() {
        let mut analyzer = DimensionalAnalyzer::new();
        analyzer.add_variable("force".to_string(), Dimension::FORCE);
        analyzer.add_variable("area".to_string(), Dimension::AREA);
        
        let expr = UnitExprParser::parse(quote! { force / area }).unwrap();
        let quantity_name = analyzer.resolve_quantity_name(&expr).unwrap();
        
        assert_eq!(quantity_name, "Pressure");
    }
}