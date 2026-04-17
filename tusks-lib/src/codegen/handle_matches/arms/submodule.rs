use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::TusksModule;
use crate::codegen::module_path::ModulePath;
use crate::codegen::util::enum_util::to_variant_ident;
use crate::codegen::util::field_util::is_generated_field;

impl TusksModule {
    /// Generates a match arm for a submodule. The complete generated pattern:
    ///
    /// ```ignore
    /// Some(cli::Commands::admin { user: p1, sub }) => {
    ///     let super_parameters = &parameters;
    ///     let parameters = super::admin::Parameters { user: p1, super_: super_parameters, .. };
    ///     match sub {
    ///         // ... nested arms ...
    ///     }
    /// }
    /// ```
    pub fn build_submodule_match_arm(
        &self,
        cli_path: &TokenStream,
        path: &ModulePath,
    ) -> TokenStream {
        let variant_ident = to_variant_ident(&self.name);
        let param_bindings = self.build_parameter_pattern_bindings();
        let mut pattern_fields = self.build_pattern_fields(&param_bindings);
        let has_commands = self.has_commands();

        if has_commands {
            pattern_fields.push(quote! { sub });
        }

        let params_init = self.build_parameter_initialization(&param_bindings, path, has_commands);
        let nested_match = self.build_nested_match_arms(path, has_commands);

        quote! {
            Some(#cli_path::Commands::#variant_ident { #(#pattern_fields),* }) => {
                #params_init
                #nested_match
            }
        }
    }

    fn build_parameter_pattern_bindings(&self) -> Vec<(syn::Ident, syn::Ident)> {
        let mut bindings = Vec::new();
        let mut counter = 1;

        if let Some(ref params) = self.parameters {
            for field in &params.pstruct.fields {
                if let Some(field_name) = &field.ident {
                    if !is_generated_field(&field_name.to_string()) {
                        let binding = syn::Ident::new(
                            &format!("p{}", counter),
                            Span::call_site(),
                        );
                        bindings.push((field_name.clone(), binding));
                        counter += 1;
                    }
                }
            }
        }

        bindings
    }

    fn has_commands(&self) -> bool {
        !self.tusks.is_empty()
            || !self.submodules.is_empty()
            || !self.external_modules.is_empty()
    }

    fn build_parameter_initialization(
        &self,
        bindings: &[(syn::Ident, syn::Ident)],
        path: &ModulePath,
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
                } else if field_name == "_phantom_lifetime_marker" {
                    field_inits.push(quote! {
                        _phantom_lifetime_marker: ::std::marker::PhantomData,
                    });
                } else if let Some((_, binding)) =
                    bindings.iter().find(|(fname, _)| fname == field_name)
                {
                    field_inits.push(quote! { #field_name: #binding, });
                }
            }
        }

        let params_path = path.super_path_to(submod_name);

        quote! {
            let super_parameters = &parameters;
            let parameters = #params_path::Parameters {
                #(#field_inits)*
            };
        }
    }

    fn build_nested_match_arms(
        &self,
        path: &ModulePath,
        has_commands: bool,
    ) -> TokenStream {
        if !has_commands {
            return Self::build_no_command_error(path);
        }

        let new_path = path.join(&self.name.to_string());
        let nested_arms = self.build_match_arms_recursive(&new_path);

        quote! {
            match sub {
                #(#nested_arms)*
            }
        }
    }
}
