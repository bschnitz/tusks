use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::TusksModule;
use crate::codegen::util::enum_util::convert_submodule_to_enum_variant;

impl TusksModule {
    /// Generates a match arm for a submodule in the CLI command enum.
    /// 
    /// This function creates a pattern match for a submodule, handling:
    /// - Parameter pattern binding generation
    /// - Parameter initialization
    /// - Nested command matching (if submodule has commands)
    /// 
    /// # Example
    /// Input: submodule "admin" with parameters and commands
    /// Output: A match arm like:
    /// ```rust
    /// Some(Cli::Commands::Admin { user: p1, sub }) => {
    ///     let super_parameters = &parameters;
    ///     let parameters = super::admin::Parameters { user: p1, super_: super_parameters };
    ///     match sub {
    ///         Some(Cli::Commands::User { id }) => { /* handle user command */ }
    ///         None => { println!("No function defined for this command!"); }
    ///     }
    /// }
    /// ```
    pub fn build_submodule_match_arm(&self, cli_path: &TokenStream, path: &[&str]) -> TokenStream {
        let variant_ident = self.build_variant_ident();
        let pattern_bindings = self.build_parameter_pattern_bindings();
        let pattern_fields = self.build_pattern_fields(&pattern_bindings);
        let has_commands = self.has_commands();
        let pattern_fields = self.add_sub_field_if_needed(pattern_fields, has_commands);
        let params_init = self.build_parameter_initialization(&pattern_bindings, path, has_commands);
        let nested_match = self.build_nested_match_arms(path, has_commands);
        
        self.build_final_match_arm(
            cli_path,
            variant_ident,
            pattern_fields,
            params_init,
            nested_match
        )
    }

    /// Builds the enum variant identifier for the submodule.
    /// 
    /// # Example
    /// Input: submodule name "admin"
    /// Output: `Admin` (as syn::Ident)
    fn build_variant_ident(&self) -> syn::Ident {
        convert_submodule_to_enum_variant(&self.name)
    }

    /// Generates pattern bindings for submodule parameters.
    /// 
    /// Creates bindings like `p1`, `p2`, etc. for parameter fields,
    /// excluding the special "super_" field.
    /// 
    /// # Example
    /// Input: parameters with fields "user" and "super_"
    /// Output: vec![("user", p1)]
    fn build_parameter_pattern_bindings(&self) -> Vec<(syn::Ident, syn::Ident)> {
        let mut bindings = Vec::new();
        let mut param_counter = 1;

        if let Some(ref params) = self.parameters {
            for field in &params.pstruct.fields {
                if let Some(field_name) = &field.ident {
                    if field_name != "super_" {
                        let binding_name = syn::Ident::new(&format!("p{}", param_counter), Span::call_site());
                        bindings.push((field_name.clone(), binding_name.clone()));
                        param_counter += 1;
                    }
                }
            }
        }

        bindings
    }

    /// Checks if submodule has any commands (tusks, submodules, or external modules).
    /// 
    /// # Example
    /// Input: submodule with tusks or submodules
    /// Output: true
    fn has_commands(&self) -> bool {
        !self.tusks.is_empty() || 
        !self.submodules.is_empty() || 
        !self.external_modules.is_empty()
    }

    /// Adds the "sub" field to pattern if submodule has commands.
    /// 
    /// # Example
    /// Input: pattern_fields = [quote! { user: p1 }], has_commands = true
    /// Output: [quote! { user: p1 }, quote! { sub }]
    fn add_sub_field_if_needed(&self, pattern_fields: Vec<TokenStream>, has_commands: bool) -> Vec<TokenStream> {
        let mut fields = pattern_fields;
        if has_commands {
            fields.push(quote! { sub });
        }
        fields
    }

    /// Builds parameter initialization code for the submodule.
    /// 
    /// Handles both "super_" field and regular parameters, creating the
    /// parameters struct with proper path resolution.
    /// 
    /// # Example
    /// Input: parameters with "user" field, path = ["main"]
    /// Output: quote! {
    ///     let super_parameters = &parameters;
    ///     let parameters = super::main::admin::Parameters { user: p1, super_: super_parameters };
    /// }
    fn build_parameter_initialization(
        &self,
        bindings: &[(syn::Ident, syn::Ident)],
        path: &[&str],
        has_commands: bool,
    ) -> TokenStream {
        if !has_commands || self.parameters.is_none() {
            return quote! {};
        }

        let submod_name = &self.name;
        let params = self.parameters.as_ref().unwrap();
        let mut field_inits = Vec::new();

        for field in &params.pstruct.fields {
            if let Some(field_name) = &field.ident {
                if field_name == "super_" {
                    field_inits.push(quote! { super_: super_parameters, });
                }
                else if field_name == "_phantom_lifetime_marker" {
                    field_inits.push(quote! {
                        _phantom_lifetime_marker: ::std::marker::PhantomData,
                    });
                } else {
                    // Find the binding for this field
                    if let Some((_, binding_name)) = bindings.iter()
                        .find(|(fname, _)| fname == field_name) {
                        field_inits.push(quote! { #field_name: #binding_name, });
                    }
                }
            }
        }

        // Build parameters path
        let params_path = if path.is_empty() {
            quote! { super::#submod_name::Parameters }
        } else {
            let path_idents: Vec<_> = path.iter()
                .map(|p| syn::Ident::new(p, Span::call_site()))
                .collect();
            quote! { super::#(#path_idents)::*::#submod_name::Parameters }
        };

        quote! {
            let super_parameters = &parameters;
            let parameters = #params_path {
                #(#field_inits)*
            };
        }
    }

    /// Builds nested match arms for submodule commands.
    /// 
    /// Generates recursive match arms for commands within the submodule.
    /// 
    /// # Example
    /// Input: submodule with commands, path = ["main"]
    /// Output: quote! {
    ///     match sub {
    ///         Some(Cli::Commands::User { id }) => { /* handle user command */ }
    ///         None => { println!("No function defined for this command!"); }
    ///     }
    /// }
    fn build_nested_match_arms(
        &self,
        path: &[&str],
        has_commands: bool,
    ) -> TokenStream {
        if !has_commands {
            let error_msg = if let Some(&last) = path.last() {
                quote! { eprintln!("Subcommand required! Please provide a subcommand for {}!", #last); }
            } else {
                quote! { eprintln!("Command required! Please provide a command!"); }
            };
            return quote! {
                #error_msg
                Some(1)
            };
        }

        let mut new_path = path.to_vec();
        let submod_name_str = self.name.to_string();
        new_path.push(&submod_name_str);

        let nested_arms = self.build_match_arms_recursive(&new_path);

        quote! {
            match sub {
                #(#nested_arms)*
            }
        }
    }

    /// Combines all components into the final match arm.
    /// 
    /// # Example
    /// Input: variant_ident = Admin, pattern_fields = [user: p1, sub], params_init, nested_match
    /// Output: quote! {
    ///     Some(Cli::Commands::Admin { user: p1, sub }) => {
    ///         #params_init
    ///         #nested_match
    ///     }
    /// }
    fn build_final_match_arm(
        &self,
        cli_path: &TokenStream,
        variant_ident: syn::Ident,
        pattern_fields: Vec<TokenStream>,
        params_init: TokenStream,
        nested_match: TokenStream,
    ) -> TokenStream {
        quote! {
            Some(#cli_path::Commands::#variant_ident { #(#pattern_fields),* }) => {
                #params_init
                #nested_match
            }
        }
    }
}
