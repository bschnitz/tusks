mod module;
mod arms;

use proc_macro2::TokenStream;

/// Code generation phase: builds the `pub fn handle_matches` function
/// with recursive match arms for command dispatch.
pub trait HandleMatchesCodegen {
    fn build_handle_matches(&self, is_tusks_root: bool) -> TokenStream;
}
