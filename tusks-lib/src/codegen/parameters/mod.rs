mod module;

use syn::ItemMod;

/// Code generation phase: supplements Parameters structs with
/// `super_` fields, lifetime markers, and missing struct definitions.
pub trait ParametersCodegen {
    fn supplement_parameters(
        &mut self,
        module: &mut ItemMod,
        is_tusks_root: bool,
        derive_debug: bool,
    ) -> syn::Result<()>;
}
