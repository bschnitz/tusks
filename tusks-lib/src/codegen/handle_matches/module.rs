use proc_macro2::TokenStream;
use quote::quote;

use crate::AttributeCheck;
use crate::codegen::module_path::ModulePath;
use crate::codegen::util::enum_util::to_variant_ident;

use crate::{TusksModule, models::Tusk};

impl TusksModule {
    /// Generate the handle_matches function
    pub fn build_handle_matches(&self, is_tusks_root: bool) -> TokenStream {
        let signature = if is_tusks_root {
            quote! {
                pub fn handle_matches(cli: &cli::Cli) -> Option<u8>
            }
        } else {
            quote! {
                pub fn handle_matches(
                    cli: &cli::Cli,
                    super_parameters: &super::parent_::Parameters
                ) -> Option<u8>
            }
        };

        let params_init = self.build_parameters_initialization();
        let path = ModulePath::new();
        let match_arms = self.build_match_arms_recursive(&path);

        quote! {
            #signature {
                #params_init

                let commands = &cli.sub;
                match commands {
                    #(#match_arms)*
                }
            }
        }
    }

    fn build_parameters_initialization(&self) -> TokenStream {
        if let Some(ref params) = self.parameters {
            let mut field_inits = Vec::new();

            for field in &params.pstruct.fields {
                if let Some(field_name) = &field.ident {
                    let field_init = match field_name.to_string().as_str() {
                        "super_" => quote! { super_: super_parameters, },
                        "_phantom_lifetime_marker" => quote! {
                            _phantom_lifetime_marker: ::std::marker::PhantomData,
                        },
                        _ => quote! { #field_name: &cli.#field_name, },
                    };
                    field_inits.push(field_init);
                }
            }

            quote! {
                let parameters = super::Parameters {
                    #(#field_inits)*
                };
            }
        } else {
            quote! {}
        }
    }

    /// Build match arms recursively with path tracking
    pub fn build_match_arms_recursive(&self, path: &ModulePath) -> Vec<TokenStream> {
        let mut arms = Vec::new();
        let cli_path = path.cli_path();

        // Arms for tusks
        for tusk in &self.tusks {
            arms.push(self.build_function_match_arm(tusk, &cli_path, path));
        }

        // Default fallback match arm for the module at the current path
        let mut has_default_match_arm = false;
        for tusk in &self.tusks {
            if tusk.func.has_attr("default") {
                arms.push(self.build_default_function_match_arm(
                    tusk,
                    path,
                    self.allow_external_subcommands
                ));
                if self.allow_external_subcommands {
                    arms.push(self.build_external_subcommand_match_arm(tusk, path));
                }
                has_default_match_arm = true;
                break;
            }
        }

        if !has_default_match_arm {
            arms.push(Self::build_no_command_error_arm(path));
        }

        // Arms for submodules
        for submodule in &self.submodules {
            arms.push(submodule.build_submodule_match_arm(&cli_path, path));
        }

        // Arm for external commands (at ANY level, not just root!)
        if !self.external_modules.is_empty() {
            arms.push(self.build_external_arm(&cli_path, path));
        }

        arms
    }

    pub fn build_no_command_error(path: &ModulePath) -> TokenStream {
        if let Some(last) = path.last() {
            quote! {
                eprintln!("Subcommand required! Please provide a subcommand for {}!", #last);
                Some(1)
            }
        } else {
            quote! {
                eprintln!("Command required! Please provide a command!");
                Some(1)
            }
        }
    }

    fn build_no_command_error_arm(path: &ModulePath) -> TokenStream {
        let error = Self::build_no_command_error(path);
        quote! {
            None => {
                #error
            }
        }
    }

    fn build_external_arm(&self, cli_path: &TokenStream, path: &ModulePath) -> TokenStream {
        let mut external_arms = Vec::new();

        for ext_mod in &self.external_modules {
            let alias = &ext_mod.alias;
            let variant_ident = to_variant_ident(alias);
            let external_path = path.super_path_to(alias);

            external_arms.push(quote! {
                #cli_path::ExternalCommands::#variant_ident(cli) => {
                    #external_path::__internal_tusks_module::handle_matches(cli, &parameters)
                }
            });
        }

        quote! {
            Some(#cli_path::Commands::TuskExternalCommands(commands)) => {
                match commands {
                    #(#external_arms)*
                }
            }
        }
    }

    pub fn tusk_has_parameters_arg(&self, tusk: &Tusk) -> bool {
        if let Some(syn::FnArg::Typed(first_param)) = tusk.func.sig.inputs.first() {
            if let Some(ref params) = self.parameters {
                return Self::is_parameters_type(&first_param.ty, &params.pstruct.ident);
            }
        }
        false
    }
}
