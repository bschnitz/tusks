use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::codegen::module_path::ModulePath;
use crate::codegen::util::enum_util::to_variant_ident;
use crate::codegen::util::field_util::is_generated_field;
use crate::{TusksModule, models::Tusk};

/// Generates the expression for the `Ok(val)` branch of a Result return type.
fn build_result_ok_handler(ok_ty: &syn::Type, val: TokenStream) -> TokenStream {
    if Tusk::is_u8_type(ok_ty) {
        quote! { Some(#val) }
    } else if Tusk::is_option_u8_type(ok_ty) {
        quote! { #val }
    } else {
        // () or unknown — treat as success
        quote! { let _ = #val; None }
    }
}

impl TusksModule {
    /// Generates a match arm for a command function.
    ///
    /// For `fn my_func(arg1: String, arg2: i32)` this produces:
    /// ```ignore
    /// Some(cli::Commands::my_func { arg1: p1, arg2: p2 }) => {
    ///     super::my_func(p1.clone(), p2.clone());
    /// }
    /// ```
    pub fn build_function_match_arm(
        &self,
        tusk: &Tusk,
        cli_path: &TokenStream,
        path: &ModulePath,
    ) -> TokenStream {
        let variant_ident = to_variant_ident(&tusk.func.sig.ident);
        let pattern_bindings = self.build_pattern_bindings(tusk);
        let pattern_fields = self.build_pattern_fields(&pattern_bindings);
        let function_call = self.build_function_call(tusk, &pattern_bindings, path, false, false);

        quote! {
            Some(#cli_path::Commands::#variant_ident { #(#pattern_fields),* }) => {
                #function_call
            }
        }
    }

    pub fn build_default_function_match_arm(
        &self,
        tusk: &Tusk,
        path: &ModulePath,
        is_external_subcommand_case: bool,
    ) -> TokenStream {
        let pattern_bindings = self.build_pattern_bindings(tusk);
        let function_call = self.build_function_call(
            tusk, &pattern_bindings, path, true, is_external_subcommand_case,
        );

        quote! {
            None => {
                #function_call
            }
        }
    }

    pub fn build_external_subcommand_match_arm(
        &self,
        tusk: &Tusk,
        path: &ModulePath,
    ) -> TokenStream {
        let pattern_bindings = self.build_pattern_bindings(tusk);
        let function_call = self.build_function_call(
            tusk, &pattern_bindings, path, false, true,
        );

        let cli_path = path.cli_path();

        quote! {
            Some(#cli_path::Commands::ClapExternalSubcommand(external_subcommand_args)) => {
                #function_call
            }
        }
    }

    fn build_function_call(
        &self,
        tusk: &Tusk,
        pattern_bindings: &[(syn::Ident, syn::Ident)],
        path: &ModulePath,
        is_default_case: bool,
        is_external_subcommand_case: bool,
    ) -> TokenStream {
        let func_args = self.build_function_arguments(
            tusk, pattern_bindings, is_default_case, is_external_subcommand_case,
        );
        let func_name = &tusk.func.sig.ident;
        let func_path = path.super_path_to(func_name);
        let maybe_await = if tusk.is_async { quote! { .await } } else { quote! {} };

        let call = quote! { #func_path(#(#func_args),*)#maybe_await };

        match &tusk.func.sig.output {
            syn::ReturnType::Default => {
                quote! { #call; None }
            }
            syn::ReturnType::Type(_, ty) => {
                if let Some(ok_ty) = Tusk::result_ok_type(ty) {
                    // Result<T, E> — unwrap with error handling
                    let ok_handler = build_result_ok_handler(ok_ty, quote! { __ok_val });
                    quote! {
                        match #call {
                            Ok(__ok_val) => { #ok_handler }
                            Err(__err) => {
                                eprintln!("Error: {}", __err);
                                Some(1)
                            }
                        }
                    }
                } else if Tusk::is_u8_type(ty) {
                    quote! { Some(#call) }
                } else if Tusk::is_option_u8_type(ty) {
                    quote! { #call }
                } else {
                    quote! { None }
                }
            }
        }
    }

    /// Creates bindings `[(field_name, p1), (field_name, p2), ...]`
    /// for function parameters, skipping the first if it's `&Parameters`.
    fn build_pattern_bindings(&self, tusk: &Tusk) -> Vec<(syn::Ident, syn::Ident)> {
        let skip = if self.tusk_has_parameters_arg(tusk) { 1 } else { 0 };
        let mut bindings = Vec::new();
        let mut counter = 1;

        for param in tusk.func.sig.inputs.iter().skip(skip) {
            if let syn::FnArg::Typed(pat_type) = param {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    let binding = syn::Ident::new(&format!("p{}", counter), Span::call_site());
                    bindings.push((pat_ident.ident.clone(), binding));
                    counter += 1;
                }
            }
        }

        bindings
    }

    /// Converts bindings to pattern fields `[field: p1, field: p2, ...]`,
    /// filtering out generated fields.
    pub fn build_pattern_fields(
        &self,
        pattern_bindings: &[(syn::Ident, syn::Ident)],
    ) -> Vec<TokenStream> {
        pattern_bindings
            .iter()
            .filter(|(name, _)| !is_generated_field(&name.to_string()))
            .map(|(name, binding)| quote! { #name: #binding })
            .collect()
    }

    fn build_function_arguments(
        &self,
        tusk: &Tusk,
        pattern_bindings: &[(syn::Ident, syn::Ident)],
        is_default_case: bool,
        is_external_subcommand_case: bool,
    ) -> Vec<TokenStream> {
        let has_params_arg = self.tusk_has_parameters_arg(tusk);
        let mut func_args = Vec::new();

        let mut number_of_non_params_args = tusk.func.sig.inputs.len();
        if has_params_arg {
            func_args.push(quote! { &parameters });
            number_of_non_params_args -= 1;
        }

        if is_default_case {
            if is_external_subcommand_case {
                func_args.push(quote! { Vec::new() });
            }
            return func_args;
        }

        if is_external_subcommand_case && number_of_non_params_args > 0 {
            func_args.push(quote! { external_subcommand_args.clone() });
            return func_args;
        }

        for (_, binding_name) in pattern_bindings {
            func_args.push(quote! { #binding_name.clone() });
        }

        func_args
    }
}
