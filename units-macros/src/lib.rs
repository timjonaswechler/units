use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, parse::{Parse, ParseStream}, punctuated::Punctuated, 
    token::Comma, Expr, Ident, Item, ItemStatic, Lit, LitFloat, LitInt, Meta, MetaList, Type, Token,
};

/// Attribute macro for unit declarations
///
/// Syntax:
/// ```ignore
/// #[unit_attr(Meter)]
/// let distance: Distance = 10.0;
///
/// #[unit_attr(Meter, Per<Second>)]
/// let speed: Speed = 25.0;
///
/// #[unit_attr(Prefixed<Kilo, Meter>)]
/// let distance: Distance = 5.0;
/// ```
#[proc_macro_attribute]
pub fn unit_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args with Punctuated::<Meta, Comma>::parse_terminated);
    let input = parse_macro_input!(input as Item);

    match unit_attr_impl(args, input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn unit_attr_impl(args: Punctuated<Meta, Comma>, input: Item) -> syn::Result<proc_macro2::TokenStream> {
    // Parse unit specification from arguments
    let unit_spec = parse_unit_specification(&args)?;

    match input {
        Item::Static(item_static) => handle_static_declaration(unit_spec, item_static),
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "#[unit_attr] can only be applied to static variable declarations",
            ));
        }
    }
}

fn handle_static_declaration(
    unit_spec: UnitSpecification,
    mut item_static: ItemStatic,
) -> syn::Result<proc_macro2::TokenStream> {
    // Extract the quantity type from the type annotation
    let quantity_type = extract_quantity_type(&item_static.ty)?;

    // Generate the unit type from the specification
    let unit_type = generate_unit_type(&unit_spec)?;

    // Modify the type to be Value<Quantity, Unit, ValueType>
    let value_type = extract_value_type(&item_static.ty);
    item_static.ty = Box::new(parse_quote! {
        ::units::Value<#quantity_type, #unit_type, #value_type>
    });

    // Wrap the expression in Value::new()
    if let Expr::Lit(ref lit_expr) = &*item_static.expr {
        item_static.expr = Box::new(parse_quote! {
            ::units::Value::new(#lit_expr)
        });
    }

    Ok(quote! {
        #item_static
    })
}

#[derive(Debug)]
struct UnitSpecification {
    components: Vec<UnitComponent>,
}

#[derive(Debug)]
enum UnitComponent {
    Simple(Ident),
    Per(Box<UnitComponent>),
    Exponent(Box<UnitComponent>, i32),
    Prefixed(Ident, Box<UnitComponent>),
}

fn parse_unit_specification(args: &Punctuated<Meta, Comma>) -> syn::Result<UnitSpecification> {
    let mut components = Vec::new();

    for arg in args {
        let component = parse_unit_component(arg)?;
        components.push(component);
    }

    if components.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "Unit specification cannot be empty",
        ));
    }

    Ok(UnitSpecification { components })
}

fn parse_unit_component(meta: &Meta) -> syn::Result<UnitComponent> {
    match meta {
        Meta::Path(path) => {
            // Simple unit like "Meter"
            if let Some(ident) = path.get_ident() {
                Ok(UnitComponent::Simple(ident.clone()))
            } else {
                Err(syn::Error::new_spanned(path, "Expected simple identifier"))
            }
        }
        Meta::List(meta_list) => {
            // Complex units like Per<Second>, Exponent<Meter, 2>, Prefixed<Kilo, Meter>
            let path = &meta_list.path;

            if let Some(ident) = path.get_ident() {
                match ident.to_string().as_str() {
                    "Per" => {
                        let nested_args = parse_meta_list_args(meta_list)?;
                        if nested_args.len() != 1 {
                            return Err(syn::Error::new_spanned(
                                meta_list,
                                "Per expects exactly one argument",
                            ));
                        }
                        let inner = parse_unit_component(&nested_args[0])?;
                        Ok(UnitComponent::Per(Box::new(inner)))
                    }
                    "Exponent" => {
                        let nested_args = parse_meta_list_args(meta_list)?;
                        if nested_args.len() != 2 {
                            return Err(syn::Error::new_spanned(
                                meta_list,
                                "Exponent expects exactly two arguments: unit and power",
                            ));
                        }
                        let unit = parse_unit_component(&nested_args[0])?;
                        let power = parse_exponent_value(&nested_args[1])?;
                        Ok(UnitComponent::Exponent(Box::new(unit), power))
                    }
                    "Prefixed" => {
                        let nested_args = parse_meta_list_args(meta_list)?;
                        if nested_args.len() != 2 {
                            return Err(syn::Error::new_spanned(
                                meta_list,
                                "Prefixed expects exactly two arguments: prefix and unit",
                            ));
                        }
                        let prefix = parse_prefix_ident(&nested_args[0])?;
                        let unit = parse_unit_component(&nested_args[1])?;
                        Ok(UnitComponent::Prefixed(prefix, Box::new(unit)))
                    }
                    _ => Err(syn::Error::new_spanned(
                        ident,
                        "Unknown unit composition type",
                    )),
                }
            } else {
                Err(syn::Error::new_spanned(path, "Expected simple identifier"))
            }
        }
        _ => Err(syn::Error::new_spanned(
            meta,
            "Unsupported unit specification syntax",
        )),
    }
}

fn parse_meta_list_args(meta_list: &MetaList) -> syn::Result<Vec<Meta>> {
    let parsed: Punctuated<Meta, Comma> = meta_list.parse_args_with(Punctuated::parse_terminated)?;
    Ok(parsed.into_iter().collect())
}

fn parse_exponent_value(meta: &Meta) -> syn::Result<i32> {
    match meta {
        Meta::Path(path) => {
            if let Some(ident) = path.get_ident() {
                ident.to_string().parse().map_err(|_| {
                    syn::Error::new_spanned(ident, "Expected integer literal for exponent")
                })
            } else {
                Err(syn::Error::new_spanned(
                    path,
                    "Expected integer literal for exponent",
                ))
            }
        }
        _ => Err(syn::Error::new_spanned(
            meta,
            "Expected integer literal for exponent",
        )),
    }
}

fn parse_prefix_ident(meta: &Meta) -> syn::Result<Ident> {
    match meta {
        Meta::Path(path) => {
            if let Some(ident) = path.get_ident() {
                Ok(ident.clone())
            } else {
                Err(syn::Error::new_spanned(
                    path,
                    "Expected simple identifier for prefix",
                ))
            }
        }
        _ => Err(syn::Error::new_spanned(
            meta,
            "Expected identifier for prefix",
        )),
    }
}

fn generate_unit_type(spec: &UnitSpecification) -> syn::Result<Type> {
    if spec.components.len() == 1 {
        generate_unit_component_type(&spec.components[0])
    } else {
        // Multiple components - create compound unit
        let component_types: Result<Vec<_>, _> = spec
            .components
            .iter()
            .map(generate_unit_component_type)
            .collect();
        let component_types = component_types?;

        Ok(parse_quote! {
            ::units::CompoundUnit<(#(#component_types),*)>
        })
    }
}

fn generate_unit_component_type(component: &UnitComponent) -> syn::Result<Type> {
    match component {
        UnitComponent::Simple(ident) => Ok(parse_quote! { ::units::#ident }),
        UnitComponent::Per(inner) => {
            let inner_type = generate_unit_component_type(inner)?;
            Ok(parse_quote! { ::units::Per<#inner_type> })
        }
        UnitComponent::Exponent(inner, power) => {
            let inner_type = generate_unit_component_type(inner)?;
            Ok(parse_quote! { ::units::Exponent<#inner_type, #power> })
        }
        UnitComponent::Prefixed(prefix, inner) => {
            let inner_type = generate_unit_component_type(inner)?;
            Ok(parse_quote! { ::units::Prefixed<::units::#prefix, #inner_type> })
        }
    }
}

fn extract_quantity_type(ty: &Type) -> syn::Result<Type> {
    // For now, just return the type as-is
    // Later we can parse types like Distance, Speed, etc.
    Ok(ty.clone())
}

fn extract_value_type(_ty: &Type) -> Type {
    // Default to f64 for now
    // Later we can parse parametric types like Distance<f32>
    parse_quote! { f64 }
}

// =============================================================================
// PHASE 5: unit!() Function-like Macro
// =============================================================================

/// Function-like macro for creating unit values
///
/// Syntax:
/// ```ignore
/// unit!(Force, Newton, 100.0)
/// unit!(Speed, MeterPerSecond, 25.5) 
/// unit!(Distance, Kilometer, 5)
/// ```
#[proc_macro]
pub fn unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as UnitMacroInput);
    
    match unit_macro_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Input structure for unit!() macro
struct UnitMacroInput {
    quantity: Ident,
    _comma1: Token![,],
    unit: Ident, 
    _comma2: Token![,],
    value: Expr,
}

impl Parse for UnitMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(UnitMacroInput {
            quantity: input.parse()?,
            _comma1: input.parse()?,
            unit: input.parse()?,
            _comma2: input.parse()?,
            value: input.parse()?,
        })
    }
}

fn unit_macro_impl(input: UnitMacroInput) -> syn::Result<proc_macro2::TokenStream> {
    let quantity = &input.quantity;
    let unit = &input.unit;
    let value = &input.value;
    
    // Generate the Value creation with proper type annotations
    Ok(quote! {
        ::units::Value::<::units::#quantity, ::units::#unit, _>::new(#value)
    })
}
