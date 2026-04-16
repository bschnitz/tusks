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

        if is_default {
            default_function::validate(&item_fn, default_exists, allow_external_subcommands)?;
        }

        Ok(Some(Tusk {
            func: item_fn,
            is_default
        }))
    }
    
    /// Validate that the return type is either nothing, u8, or Option<u8>
    fn validate_return_type(output: &syn::ReturnType) -> syn::Result<()> {
        match output {
            syn::ReturnType::Default => {
                // No return type - that's fine
                Ok(())
            }
            syn::ReturnType::Type(_, ty) => {
                // Check if it's u8 or Option<u8>
                if Self::is_u8_type(ty) || Self::is_option_u8_type(ty) {
                    Ok(())
                } else {
                    Err(syn::Error::new_spanned(
                        ty,
                        "command function must return (), u8, or Option<u8>"
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


    // Helper function to check if a type is Vec<String>
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
