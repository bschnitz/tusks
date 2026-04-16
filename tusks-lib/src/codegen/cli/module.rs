use quote::quote;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::codegen::util::enum_util::{
    convert_external_module_to_enum_variant,
    convert_function_to_enum_variant,
    convert_submodule_to_enum_variant
};

use crate::{TusksModule, models::{Tusk, TusksParameters}};

impl TusksModule{
    /// Generate all code inside `pub mod cli`
    pub fn build_cli(&self, path: Vec<&Ident>, debug: bool) -> TokenStream {
        let mut items = Vec::new();

        items.push(quote! {use ::tusks::clap;});
        
        // 1. If root (path empty): generate Cli struct
        if path.is_empty() {
            items.push(self.build_cli_struct(debug));
        }
        
        // 2. Generate ExternalCommands enum if needed
        if !self.external_modules.is_empty() {
            items.push(self.build_external_commands_enum(&path, debug));
        }
        
        // 3. Generate Commands enum if needed
        if !self.tusks.is_empty() || !self.external_modules.is_empty() {
            items.push(self.build_commands_enum(debug));
        }
        
        // 4. Generate submodule modules and recurse
        for submodule in &self.submodules {
            let mut sub_path = path.clone();
            sub_path.push(&submodule.name);
            let submod_name = &submodule.name;
            let submod_content = submodule.build_cli(sub_path, debug);
            
            items.push(quote! {
                pub mod #submod_name {
                    #submod_content
                }
            });
        }
        
        quote! {
            #(#items)*
        }
    }

    /// Generate the root Cli struct
    fn build_cli_struct(&self, debug: bool) -> TokenStream {
        // Extract fields from parameters struct
        let fields = if let Some(ref params) = self.parameters {
            self.build_cli_fields_from_parameters(params)
        } else {
            quote! {}
        };
        
        // Add subcommand field if we have commands
        let subcommand_field = if !self.tusks.is_empty() || !self.external_modules.is_empty() {
            let subcommand_attr = self.generate_command_attribute_for_subcommands();
            quote! {
                #subcommand_attr
                pub sub: Option<Commands>,
            }
        } else {
            quote! {}
        };
        
        let derive_attr = if debug {
            quote! {}
        } else {
            quote! { #[derive(::tusks::clap::Parser)] }
        };
        
        let command_attr = self.generate_command_attribute();
        
        quote! {
            #derive_attr
            #command_attr
            pub struct Cli {
                #fields
                #subcommand_field
            }
        }
    }
    
    /// Build fields for Cli struct from Parameters struct
    fn build_cli_fields_from_parameters(&self, params: &TusksParameters) -> TokenStream {
        let mut fields = Vec::new();

        for field in &params.pstruct.fields {
            let field_name = &field.ident;

            // Skip the super_ field
            if field_name.as_ref().map(|id| id == "super_").unwrap_or(false) {
                continue;
            }

            // Skip the _phantom_lifetime_marker
            if field_name.as_ref().map(|id| id == "_phantom_lifetime_marker").unwrap_or(false) {
                continue;
            }

            let field_type = Self::dereference_type(&field.ty);

            // Filter and keep #[arg(...)] attributes with original spans
            let attrs: Vec<_> = field.attrs.iter()
                .filter(|attr| attr.path().is_ident("arg"))
                .collect();

            fields.push(quote! {
                #(#attrs)*
                pub #field_name: #field_type,
            });
        }

        quote! {
            #(#fields)*
        }
    }
    
    /// Convert a reference type to its dereferenced type
    /// e.g., &Option<String> -> Option<String>
    fn dereference_type(ty: &syn::Type) -> syn::Type {
        if let syn::Type::Reference(type_ref) = ty {
            (*type_ref.elem).clone()
        } else {
            ty.clone()
        }
    }
    
    /// Generate the ExternalCommands enum
    fn build_external_commands_enum(&self, path: &Vec<&Ident>, debug: bool) -> TokenStream {
        let variants: Vec<_> = self.external_modules.iter().map(|ext_mod| {
            let variant_ident = convert_external_module_to_enum_variant(&ext_mod.alias);

            // Anzahl super:: Prefixe: path.len() + 2
            let mut full_path: Vec<syn::Ident> = (0..path.len() + 2)
                .map(|_| syn::Ident::new("super", ext_mod.alias.span()))
                .collect();

            // Originalpfad anhängen
            for p in path {
                full_path.push((*p).clone());
            }

            // Alias anhängen
            full_path.push(ext_mod.alias.clone());

            let command_attr = ext_mod.generate_command_attribute();
            
            quote! {
                #command_attr
                #[allow(non_camel_case_types)]
                #variant_ident(
                    #(#full_path)::*::__internal_tusks_module::cli::Cli
                ),
            }
        }).collect();

        let derive_attr = if debug {
            quote! {}
        } else {
            quote! { #[derive(::tusks::clap::Subcommand)] }
        };

        quote! {
            #derive_attr
            pub enum ExternalCommands {
                #(#variants)*
            }
        }
    }

    /// Generate the Commands enum
    fn build_commands_enum(&self, debug: bool) -> TokenStream {
        let mut variants = Vec::new();
        
        // Add variants for tusks (command functions)
        for tusk in &self.tusks {
            variants.push(self.build_command_variant_from_tusk(tusk));
        }
        
        // Add variants for submodules
        for submodule in &self.submodules {
            variants.push(self.build_command_variant_from_submodule(submodule));
        }
        
        if !self.external_modules.is_empty() {
            let attr = self.generate_command_attribute_for_external_subcommands();
            variants.push(quote! {
                #attr
                TuskExternalCommands(ExternalCommands),
            });
        }

        if self.allow_external_subcommands {
            variants.push(quote! {
                #[command(external_subcommand)]
                ClapExternalSubcommand(Vec<String>),
            });
        }
        
        let derive_attr = if debug {
            quote! {}
        } else {
            quote! { #[derive(::tusks::clap::Subcommand)] }
        };
        
        quote! {
            #derive_attr
            pub enum Commands {
                #(#variants)*
            }
        }
    }
    
    /// Build a command variant from a Tusk (command function)
    fn build_command_variant_from_tusk(&self, tusk: &Tusk) -> TokenStream {
        let func_name = &tusk.func.sig.ident;
        let variant_ident = convert_function_to_enum_variant(func_name);

        // Extract fields from function parameters (skip first parameter which is &Parameters)
        let fields = self.build_fields_from_tusk_params(tusk);

        let command_attr = tusk.generate_command_attribute();
        
        quote! {
            #command_attr
            #[allow(non_camel_case_types)]
            #variant_ident {
                #fields
            },
        }
    }

    /// Build fields from tusk function parameters
    fn build_fields_from_tusk_params(&self, tusk: &Tusk) -> TokenStream {
        let mut fields = Vec::new();

        let mut params_iter = tusk.func.sig.inputs.iter();

        // Check if first parameter is &Parameters (matching our parameters struct)
        let skip_first = if let Some(syn::FnArg::Typed(first_param)) = params_iter.next() {
            if let Some(ref params) = self.parameters {
                // Check if the type matches &ParametersStructName
                Self::is_parameters_type(&first_param.ty, &params.pstruct.ident)
            } else {
                false
            }
        } else {
            false
        };

        // If we didn't skip first, reset iterator
        let params_to_process: Vec<_> = if skip_first {
            params_iter.collect()
        } else {
            tusk.func.sig.inputs.iter().collect()
        };

        for param in params_to_process {
            if let syn::FnArg::Typed(pat_type) = param {
                let param_name = if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    &pat_ident.ident
                } else {
                    continue;
                };

                let param_type = &pat_type.ty;

                // Filter #[arg(...)] attributes
                let attrs: Vec<_> = pat_type.attrs.iter()
                    .filter(|attr| attr.path().is_ident("arg"))
                    .collect();

                if !attrs.is_empty() {
                    fields.push(quote! {
                        #(#attrs)*
                        #param_name: #param_type,
                    });
                } else {
                    fields.push(quote! {
                        #[arg(long)]
                        #param_name: #param_type,
                    });
                }
            }
        }

        quote! {
            #(#fields)*
        }
    }

    /// Check if a type is a reference to a parameters struct
    pub fn is_parameters_type(ty: &syn::Type, params_ident: &Ident) -> bool {
        if let syn::Type::Reference(type_ref) = ty {
            if let syn::Type::Path(type_path) = &*type_ref.elem {
                if let Some(segment) = type_path.path.segments.last() {
                    return segment.ident == *params_ident;
                }
            }
        }
        false
    }
    
    /// Build a command variant from a submodule
    fn build_command_variant_from_submodule(&self, submodule: &TusksModule) -> TokenStream {
        let submod_name = &submodule.name;
        let variant_ident = convert_submodule_to_enum_variant(submod_name);

        // Extract fields from submodule's parameters
        let fields = if let Some(ref params) = submodule.parameters {
            self.build_enum_fields_from_parameters(params)
        } else {
            quote! {}
        };

        // Add subcommand field if submodule has commands
        let subcommand_field = if !submodule.tusks.is_empty() || !submodule.external_modules.is_empty() {
            let subcommand_attr = submodule.generate_command_attribute_for_subcommands();
            quote! {
                #subcommand_attr
                sub: Option<#submod_name::Commands>,
            }
        } else {
            quote! {}
        };

        let command_attr = submodule.generate_command_attribute();
        
        quote! {
            #command_attr
            #[allow(non_camel_case_types)]
            #variant_ident {
                #fields
                #subcommand_field
            },
        }
    }

    /// Build fields for enum variants from Parameters struct (without pub)
    fn build_enum_fields_from_parameters(&self, params: &TusksParameters) -> TokenStream {
        let mut fields = Vec::new();

        for field in &params.pstruct.fields {
            let field_name = &field.ident;

            // Skip the super_ field
            if field_name.as_ref().map(|id| id == "super_").unwrap_or(false) {
                continue;
            }

            if field_name.as_ref().map(|id| id == "_phantom_lifetime_marker").unwrap_or(false) {
                continue;
            }

            let field_type = Self::dereference_type(&field.ty);

            // Filter and keep #[arg(...)] attributes with original spans
            let attrs: Vec<_> = field.attrs.iter()
                .filter(|attr| attr.path().is_ident("arg"))
                .collect();

            fields.push(quote! {
                #(#attrs)*
                #field_name: #field_type,
            });
        }

        quote! {
            #(#fields)*
        }
    }
    
    
}
