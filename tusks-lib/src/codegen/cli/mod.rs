mod module;

use proc_macro2::TokenStream;
use syn::Ident;

/// Code generation phase: builds the `pub mod cli` contents
/// (Cli struct, Commands enum, ExternalCommands enum).
pub trait CliCodegen {
    fn build_cli(&self, path: Vec<&Ident>, debug: bool) -> TokenStream;
}
