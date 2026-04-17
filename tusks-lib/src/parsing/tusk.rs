use syn::ItemFn;
use crate::parsing::util::attr::AttributeCheck;

use crate::models::Tusk;

impl Tusk {
    pub fn from_fn(
        item_fn: ItemFn,
        default_exists: bool,
        allow_external_subcommands: bool
    ) -> syn::Result<Option<Self>> {
        // Only consider pub functions
        if !matches!(item_fn.vis, syn::Visibility::Public(_)) || item_fn.has_attr("skip") {
            return Ok(None);
        }

        // Validate return type is either nothing, i32, or Option<i32>
        Self::validate_return_type(&item_fn.sig.output)?;

        let is_default = item_fn.has_attr("default");
        let is_async = item_fn.sig.asyncness.is_some();

        if is_async && !cfg!(feature = "async") {
            return Err(syn::Error::new_spanned(
                &item_fn.sig.asyncness,
                "async command functions require the `async` feature. \
                 Add `tusks = { version = \"...\", features = [\"async\"] }` to your Cargo.toml"
            ));
        }

        if is_default {
            default_function::validate(&item_fn, default_exists, allow_external_subcommands)?;
        }

        Ok(Some(Tusk {
            func: item_fn,
            is_default,
            is_async,
        }))
    }
    
    /// Validate that the return type is one of:
    /// `()`, `u8`, `Option<u8>`, or `Result<T, E>` where T is one of those.
    fn validate_return_type(output: &syn::ReturnType) -> syn::Result<()> {
        match output {
            syn::ReturnType::Default => Ok(()),
            syn::ReturnType::Type(_, ty) => {
                if Self::is_u8_type(ty)
                    || Self::is_option_u8_type(ty)
                    || Self::is_result_type(ty)
                {
                    Ok(())
                } else {
                    Err(syn::Error::new_spanned(
                        ty,
                        "command function must return (), u8, Option<u8>, \
                         or Result<T, E> where T is (), u8, or Option<u8>"
                    ))
                }
            }
        }
    }

    /// Check if a type is u8
    pub fn is_u8_type(ty: &syn::Type) -> bool {
        let syn::Type::Path(type_path) = ty else {
            return false;
        };

        let Some(segment) = type_path.path.segments.last() else {
            return false;
        };

        segment.ident == "u8"
    }

    /// Check if a type is Option<u8>
    pub fn is_option_u8_type(ty: &syn::Type) -> bool {
        let syn::Type::Path(type_path) = ty else {
            return false;
        };

        let Some(segment) = type_path.path.segments.last() else {
            return false;
        };

        if segment.ident != "Option" {
            return false;
        }

        let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
            return false;
        };

        let Some(first_arg) = args.args.first() else {
            return false;
        };

        let syn::GenericArgument::Type(inner_ty) = first_arg else {
            return false;
        };

        Self::is_u8_type(inner_ty)
    }

    /// Check if a type is Result<T, E> where T is (), u8, or Option<u8>.
    /// The error type E is unconstrained (must implement Display at use-site).
    pub fn is_result_type(ty: &syn::Type) -> bool {
        let syn::Type::Path(type_path) = ty else {
            return false;
        };

        let Some(segment) = type_path.path.segments.last() else {
            return false;
        };

        if segment.ident != "Result" {
            return false;
        }

        let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
            return false;
        };

        // Result needs at least the Ok type (E defaults to () in some contexts, but typically both are present)
        let Some(first_arg) = args.args.first() else {
            return false;
        };

        let syn::GenericArgument::Type(ok_ty) = first_arg else {
            return false;
        };

        // The Ok type must be one of: (), u8, Option<u8>
        Self::is_unit_type(ok_ty) || Self::is_u8_type(ok_ty) || Self::is_option_u8_type(ok_ty)
    }

    /// Check if a type is the unit type `()`
    fn is_unit_type(ty: &syn::Type) -> bool {
        matches!(ty, syn::Type::Tuple(tuple) if tuple.elems.is_empty())
    }

    /// Extract the Ok type from a Result<T, E>. Returns None if not a Result.
    pub fn result_ok_type(ty: &syn::Type) -> Option<&syn::Type> {
        let syn::Type::Path(type_path) = ty else { return None };
        let segment = type_path.path.segments.last()?;
        if segment.ident != "Result" { return None; }
        let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
        let syn::GenericArgument::Type(ok_ty) = args.args.first()? else { return None };
        Some(ok_ty)
    }
}

mod default_function {
    use syn::ItemFn;

    pub fn validate(
        item_fn: &ItemFn,
        default_exists: bool,
        allow_external_subcommands: bool
    ) -> syn::Result<()> {
        check_duplicate_default(item_fn, default_exists)?;
        validate_default_function_arguments(item_fn, allow_external_subcommands)
    }

    fn check_duplicate_default(item_fn: &ItemFn, default_exists: bool) -> syn::Result<()> {
        if default_exists {
            if let Some(attr) = item_fn.attrs.iter().find(|a| a.path().is_ident("default")) {
                return Err(syn::Error::new_spanned(
                    attr,
                    "only one function can be marked with #[default]"
                ));
            }
        }
        Ok(())
    }

    fn validate_default_function_arguments(
        item_fn: &ItemFn,
        allow_external_subcommands: bool
    ) -> syn::Result<()> {
        match item_fn.sig.inputs.len() {
            0 => Ok(()),
            1 => validate_single_argument(&item_fn.sig.inputs[0], allow_external_subcommands),
            2 => validate_two_arguments(
                &item_fn.sig.inputs[0],
                &item_fn.sig.inputs[1],
                allow_external_subcommands
            ),
            _ => Err(syn::Error::new_spanned(
                &item_fn.sig.inputs,
                error_message_too_many_args(allow_external_subcommands)
            ))
        }
    }

    fn validate_single_argument(
        arg: &syn::FnArg,
        allow_external_subcommands: bool
    ) -> syn::Result<()> {
        let syn::FnArg::Typed(pat_type) = arg else {
            return Err(error_single_argument(arg, allow_external_subcommands));
        };

        // Check if it's &Parameters (reference without path)
        if is_parameters_reference(&pat_type.ty) {
            return Ok(());
        }

        // Check if it's Vec<String> (not a reference)
        if allow_external_subcommands {
            if let syn::Type::Path(type_path) = &*pat_type.ty {
                if is_vec_string(type_path) {
                    return Ok(());
                }
            }
        }

        Err(error_single_argument(arg, allow_external_subcommands))
    }

    fn validate_two_arguments(
        arg1: &syn::FnArg,
        arg2: &syn::FnArg,
        allow_external_subcommands: bool
    ) -> syn::Result<()> {
        if !allow_external_subcommands {
            return Err(syn::Error::new_spanned(
                quote::quote! { #arg1, #arg2 },
                "default function must have either no arguments \
                    or exactly one argument of type &Parameters"
            ));
        }

        let (syn::FnArg::Typed(pat_type1), syn::FnArg::Typed(pat_type2)) = (arg1, arg2) else {
            return Err(error_two_arguments_signature(arg1, arg2));
        };

        // First must be &Parameters, second must be Vec<String>
        if !is_parameters_reference(&pat_type1.ty) {
            return Err(error_two_arguments_signature(arg1, arg2));
        }

        let syn::Type::Path(type_path2) = &*pat_type2.ty else {
            return Err(error_two_arguments_signature(arg1, arg2));
        };

        if !is_vec_string(type_path2) {
            return Err(error_two_arguments_signature(arg1, arg2))
        }

        Ok(())
    }

    /// Check if type is &Parameters (reference to Parameters without path)
    fn is_parameters_reference(ty: &syn::Type) -> bool {
        let syn::Type::Reference(type_ref) = ty else {
            return false;
        };

        let syn::Type::Path(type_path) = &*type_ref.elem else {
            return false;
        };

        // Check that it's just "Parameters" without any path segments
        type_path.path.segments.len() == 1 
        && type_path.path.segments[0].ident == "Parameters"
        && type_path.qself.is_none()
    }

    fn error_single_argument(
        arg: &syn::FnArg,
        allow_external_subcommands: bool
    ) -> syn::Error {
        let message = if allow_external_subcommands {
            "default function must have either no arguments, \
                a &Parameters argument, \
                a Vec<String> argument, \
                or both (&Parameters, Vec<String>)"
        } else {
            "default function must have either no arguments \
                or exactly one argument of type &Parameters"
        };
        syn::Error::new_spanned(arg, message)
    }

    fn error_two_arguments_signature(arg1: &syn::FnArg, arg2: &syn::FnArg) -> syn::Error {
        syn::Error::new_spanned(
            quote::quote! { #arg1, #arg2 },
            "default function with two arguments must have signature: \
                (&Parameters, Vec<String>)"
        )
    }

    fn error_message_too_many_args(allow_external_subcommands: bool) -> &'static str {
        if allow_external_subcommands {
            "default function must have at most two arguments: \
                &Parameters and Vec<String>"
        } else {
            "default function must have either no arguments \
                or exactly one argument of type &Parameters"
        }
    }


    /// Helper function to check if a type is Vec<String>
    fn is_vec_string(type_path: &syn::TypePath) -> bool {
        let Some(segment) = type_path.path.segments.last() else {
            return false;
        };

        if segment.ident != "Vec" {
            return false;
        }

        let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
            return false;
        };

        if args.args.len() != 1 {
            return false;
        }

        let syn::GenericArgument::Type(syn::Type::Path(inner_type)) = &args.args[0] else {
            return false;
        };

        inner_type.path.is_ident("String")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_fn(code: &str) -> ItemFn {
        syn::parse_str::<ItemFn>(code).unwrap()
    }

    // --- Type checking helpers ---

    fn parse_type(code: &str) -> syn::Type {
        syn::parse_str::<syn::Type>(code).unwrap()
    }

    #[test]
    fn is_u8_type_accepts_u8() {
        assert!(Tusk::is_u8_type(&parse_type("u8")));
    }

    #[test]
    fn is_u8_type_rejects_u32() {
        assert!(!Tusk::is_u8_type(&parse_type("u32")));
    }

    #[test]
    fn is_u8_type_rejects_string() {
        assert!(!Tusk::is_u8_type(&parse_type("String")));
    }

    #[test]
    fn is_option_u8_accepts_option_u8() {
        assert!(Tusk::is_option_u8_type(&parse_type("Option<u8>")));
    }

    #[test]
    fn is_option_u8_rejects_option_u32() {
        assert!(!Tusk::is_option_u8_type(&parse_type("Option<u32>")));
    }

    #[test]
    fn is_option_u8_rejects_option_string() {
        assert!(!Tusk::is_option_u8_type(&parse_type("Option<String>")));
    }

    #[test]
    fn is_option_u8_rejects_bare_u8() {
        assert!(!Tusk::is_option_u8_type(&parse_type("u8")));
    }

    #[test]
    fn is_option_u8_rejects_vec_u8() {
        assert!(!Tusk::is_option_u8_type(&parse_type("Vec<u8>")));
    }

    // --- Tusk::from_fn ---

    #[test]
    fn from_fn_public_no_return() {
        let f = parse_fn("pub fn hello() {}");
        let result = Tusk::from_fn(f, false, false).unwrap();
        assert!(result.is_some());
        assert!(!result.unwrap().is_default);
    }

    #[test]
    fn from_fn_public_returns_u8() {
        let f = parse_fn("pub fn hello() -> u8 { 0 }");
        let result = Tusk::from_fn(f, false, false).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn from_fn_public_returns_option_u8() {
        let f = parse_fn("pub fn hello() -> Option<u8> { None }");
        let result = Tusk::from_fn(f, false, false).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn from_fn_rejects_invalid_return_type() {
        let f = parse_fn("pub fn hello() -> String { String::new() }");
        let err = Tusk::from_fn(f, false, false).unwrap_err();
        assert!(err.to_string().contains("command function must return"));
    }

    #[test]
    fn from_fn_rejects_i32_return() {
        let f = parse_fn("pub fn hello() -> i32 { 0 }");
        let err = Tusk::from_fn(f, false, false).unwrap_err();
        assert!(err.to_string().contains("command function must return"));
    }

    #[test]
    fn from_fn_skips_private() {
        let f = parse_fn("fn hello() {}");
        let result = Tusk::from_fn(f, false, false).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn from_fn_skips_skip_attribute() {
        let f = parse_fn("#[skip] pub fn hello() {}");
        let result = Tusk::from_fn(f, false, false).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn from_fn_default_attribute() {
        let f = parse_fn("#[default] pub fn hello() {}");
        let result = Tusk::from_fn(f, false, false).unwrap();
        assert!(result.is_some());
        assert!(result.unwrap().is_default);
    }

    #[test]
    fn from_fn_duplicate_default_errors() {
        let f = parse_fn("#[default] pub fn hello() {}");
        let err = Tusk::from_fn(f, true, false).unwrap_err();
        assert!(err.to_string().contains("only one function can be marked with #[default]"));
    }

    // --- Default function argument validation ---

    #[test]
    fn default_fn_zero_args_ok() {
        let f = parse_fn("#[default] pub fn hello() {}");
        assert!(Tusk::from_fn(f, false, false).is_ok());
    }

    #[test]
    fn default_fn_parameters_arg_ok() {
        let f = parse_fn("#[default] pub fn hello(p: &Parameters) {}");
        assert!(Tusk::from_fn(f, false, false).is_ok());
    }

    #[test]
    fn default_fn_vec_string_without_external_subcommands_fails() {
        let f = parse_fn("#[default] pub fn hello(args: Vec<String>) {}");
        let err = Tusk::from_fn(f, false, false).unwrap_err();
        assert!(err.to_string().contains("&Parameters"));
    }

    #[test]
    fn default_fn_vec_string_with_external_subcommands_ok() {
        let f = parse_fn("#[default] pub fn hello(args: Vec<String>) {}");
        assert!(Tusk::from_fn(f, false, true).is_ok());
    }

    #[test]
    fn default_fn_two_args_with_external_subcommands_ok() {
        let f = parse_fn("#[default] pub fn hello(p: &Parameters, args: Vec<String>) {}");
        assert!(Tusk::from_fn(f, false, true).is_ok());
    }

    #[test]
    fn default_fn_two_args_without_external_subcommands_fails() {
        let f = parse_fn("#[default] pub fn hello(p: &Parameters, args: Vec<String>) {}");
        let err = Tusk::from_fn(f, false, false).unwrap_err();
        assert!(err.to_string().contains("&Parameters"));
    }

    #[test]
    fn default_fn_three_args_fails() {
        let f = parse_fn("#[default] pub fn hello(a: String, b: String, c: String) {}");
        let err = Tusk::from_fn(f, false, true).unwrap_err();
        assert!(err.to_string().contains("at most two arguments"));
    }

    #[test]
    fn default_fn_wrong_single_arg_type_fails() {
        let f = parse_fn("#[default] pub fn hello(x: String) {}");
        let err = Tusk::from_fn(f, false, false).unwrap_err();
        assert!(err.to_string().contains("&Parameters"));
    }

    #[test]
    fn default_fn_two_args_wrong_order_fails() {
        // Vec<String> first, &Parameters second — wrong order
        let f = parse_fn("#[default] pub fn hello(args: Vec<String>, p: &Parameters) {}");
        let err = Tusk::from_fn(f, false, true).unwrap_err();
        assert!(err.to_string().contains("(&Parameters, Vec<String>)"));
    }

    // --- Async detection ---

    #[test]
    fn sync_fn_is_not_async() {
        let f = parse_fn("pub fn hello() {}");
        let tusk = Tusk::from_fn(f, false, false).unwrap().unwrap();
        assert!(!tusk.is_async);
    }

    #[test]
    #[cfg(feature = "async")]
    fn async_fn_detected() {
        let f = parse_fn("pub async fn hello() {}");
        let tusk = Tusk::from_fn(f, false, false).unwrap().unwrap();
        assert!(tusk.is_async);
    }

    #[test]
    #[cfg(not(feature = "async"))]
    fn async_fn_without_feature_errors() {
        let f = parse_fn("pub async fn hello() {}");
        let err = Tusk::from_fn(f, false, false).unwrap_err();
        assert!(err.to_string().contains("async"));
    }

    // --- Result return types ---

    #[test]
    fn from_fn_accepts_result_unit() {
        let f = parse_fn("pub fn hello() -> Result<(), String> { Ok(()) }");
        assert!(Tusk::from_fn(f, false, false).unwrap().is_some());
    }

    #[test]
    fn from_fn_accepts_result_u8() {
        let f = parse_fn("pub fn hello() -> Result<u8, String> { Ok(0) }");
        assert!(Tusk::from_fn(f, false, false).unwrap().is_some());
    }

    #[test]
    fn from_fn_accepts_result_option_u8() {
        let f = parse_fn("pub fn hello() -> Result<Option<u8>, String> { Ok(None) }");
        assert!(Tusk::from_fn(f, false, false).unwrap().is_some());
    }

    #[test]
    fn from_fn_rejects_result_with_bad_ok_type() {
        let f = parse_fn("pub fn hello() -> Result<String, String> { Ok(String::new()) }");
        assert!(Tusk::from_fn(f, false, false).is_err());
    }

    #[test]
    fn is_result_type_checks() {
        assert!(Tusk::is_result_type(&parse_type("Result<(), String>")));
        assert!(Tusk::is_result_type(&parse_type("Result<u8, Box<dyn std::error::Error>>")));
        assert!(Tusk::is_result_type(&parse_type("Result<Option<u8>, anyhow::Error>")));
        assert!(!Tusk::is_result_type(&parse_type("Result<String, String>")));
        assert!(!Tusk::is_result_type(&parse_type("Option<u8>")));
    }
}
