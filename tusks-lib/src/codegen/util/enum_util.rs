use syn::Ident;

/// Clone an identifier for use as an enum variant name.
///
/// Variant names are kept as-is (snake_case) and paired with
/// `#[allow(non_camel_case_types)]` in the generated code.
pub fn to_variant_ident(ident: &Ident) -> Ident {
    ident.clone()
}
