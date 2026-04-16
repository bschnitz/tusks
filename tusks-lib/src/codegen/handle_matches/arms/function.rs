use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};

use crate::codegen::util::enum_util::convert_function_to_enum_variant;

use crate::{TusksModule, models::Tusk};

impl TusksModule {
    /// Coordinates the construction of a match arm for a function.
    /// Creates all necessary parts and combines them into the final code.
    /// 
    /// # Examples
    /// 
    /// For a function `fn my_func(arg1: String, arg2: i32)` this generates:
    /// ```rust
    /// Some(cli::Commands::MyFunction { arg1: p1, arg2: p2 }) => {
    ///     super::my_func(p1.clone(), p2.clone());
    /// }
    /// ```
    /// 
    /// For a function with parameters `fn my_func(params: &Parameters, arg1: String)`
    /// this generates:
    /// ```rust
    /// Some(cli::Commands::MyFunction { arg1: p1 }) => {
    ///     super::my_func(&parameters, p1.clone());
    /// }
    /// ```
    pub fn build_function_match_arm(
        &self,
        tusk: &Tusk,
        cli_path: &TokenStream,
        path: &[&str]
    ) -> TokenStream {
        let variant_ident = convert_function_to_enum_variant(&tusk.func.sig.ident);
        let pattern_bindings = self.build_pattern_bindings(tusk);
        let pattern_fields = self.build_pattern_fields(&pattern_bindings);
        let function_call = self.build_function_call(
            tusk,
            &pattern_bindings,
            path,
            false,
            false
        );

        quote! {
            Some(#cli_path::Commands::#variant_ident { #(#pattern_fields),* }) => {
                #function_call
            }
        }
    }

    pub fn build_default_function_match_arm(
        &self,
        tusk: &Tusk,
        path: &[&str],
        is_external_subcommand_case: bool
    ) -> TokenStream {
        let pattern_bindings = self.build_pattern_bindings(tusk);
        let function_call = self.build_function_call(
            tusk,
            &pattern_bindings,
            path,
            true,
            is_external_subcommand_case
        );

        quote! {
            None => {
                #function_call
            }
        }
    }

    pub fn build_external_subcommand_match_arm(&self, tusk: &Tusk, path: &[&str]) -> TokenStream
    {
        let pattern_bindings = self.build_pattern_bindings(tusk);
        let function_call = self.build_function_call(
            tusk,
            &pattern_bindings,
            path,
            false,
            true
        );

        let path_tokens = path.iter().map(|segment| {
            let ident = format_ident!("{}", segment);
            quote! { #ident:: }
        });

        quote! {
            Some(cli::#(#path_tokens)*Commands::ClapExternalSubcommand(external_subcommand_args)) => {
                #function_call
            }
        }
    }

    /// Creates the function call with proper arguments and path, always returning Option<i32>
    fn build_function_call(
        &self,
        tusk: &Tusk,
        pattern_bindings: &[(syn::Ident, syn::Ident)],
        path: &[&str],
        is_default_case: bool,
        is_external_subcommand_case: bool
    ) -> TokenStream {
        let func_args = self.build_function_arguments(
            tusk,
            pattern_bindings,
            is_default_case,
            is_external_subcommand_case
        );
        let func_path = self.build_function_path(tusk, path);
        
        match &tusk.func.sig.output {
            syn::ReturnType::Default => {
                // Function returns () - call it and return None
                quote! { #func_path(#(#func_args),*); None }
            }
            syn::ReturnType::Type(_, ty) => {
                if Tusk::is_u8_type(ty) {
                    // Function returns u8 - call it and wrap in Some
                    quote! { Some(#func_path(#(#func_args),*)) }
                } else if Tusk::is_option_u8_type(ty) {
                    // Function returns Option<u8> - call it and return as is
                    quote! { #func_path(#(#func_args),*) }
                } else {
                    // This should not happen due to validation
                    quote! { None }
                }
            }
        }
    }

    /// Creates bindings for function parameters (p1, p2, p3, ...).
    /// Skips the first parameter if it's &Parameters.
    /// 
    /// # Examples
    /// 
    /// For function `fn my_func(params: &Params, arg1: String, arg2: i32)`:
    /// ```rust
    /// [("arg1", "p1"), ("arg2", "p2")]
    /// ```
    fn build_pattern_bindings(&self, tusk: &Tusk) -> Vec<(syn::Ident, syn::Ident)> {
        let has_params_arg = self.tusk_has_parameters_arg(tusk);
        let skip = if has_params_arg { 1 } else { 0 };

        let mut pattern_bindings = Vec::new();
        let mut param_counter = 1;

        for param in tusk.func.sig.inputs.iter().skip(skip) {
            if let syn::FnArg::Typed(pat_type) = param {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    let field_name = &pat_ident.ident;
                    let binding_name = syn::Ident::new(
                        &format!("p{}", param_counter),
                        Span::call_site()
                    );
                    pattern_bindings.push((field_name.clone(), binding_name.clone()));
                    param_counter += 1;
                }
            }
        }

        pattern_bindings
    }

    /// Creates the fields for the match arm pattern.
    /// 
    /// # Examples
    /// 
    /// For bindings `[("arg1", "p1"), ("arg2", "p2")]`:
    /// ```rust
    /// ["arg1: p1", "arg2: p2"]
    /// ```
    pub fn build_pattern_fields(
        &self,
        pattern_bindings: &[(syn::Ident, syn::Ident)]
    ) -> Vec<TokenStream> {
        pattern_bindings.iter()
            .filter(|(field_name, _)| {
                let field_name_str = field_name.to_string();
                field_name_str != "_phantom_lifetime_marker"
            })
            .map(|(field_name, binding_name)| {
                quote! { #field_name: #binding_name }
            })
            .collect()
    }

    /// Creates the arguments for the function call.
    /// Adds &parameters if present, followed by the bound parameters.
    /// 
    /// # Examples
    /// 
    /// For function with parameters:
    /// ```rust
    /// [&parameters, p1.clone(), p2.clone()]
    /// ```
    fn build_function_arguments(
        &self,
        tusk: &Tusk,
        pattern_bindings: &[(syn::Ident, syn::Ident)],
        is_default_case: bool,
        is_external_subcommand_case: bool
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

    /// Creates the full path to the function.
    /// Handles both local and nested paths.
    /// 
    /// # Examples
    /// 
    /// - Local path: `super::my_function`
    /// - Nested path: `super::module1::module2::my_function`
    fn build_function_path(&self, tusk: &Tusk, path: &[&str]) -> TokenStream {
        let func_name = &tusk.func.sig.ident;

        if path.is_empty() {
            quote! { super::#func_name }
        } else {
            let path_idents: Vec<_> = path.iter()
                .map(|p| syn::Ident::new(p, Span::call_site()))
                .collect();
            quote! { super::#(#path_idents)::*::#func_name }
        }
    }
}
