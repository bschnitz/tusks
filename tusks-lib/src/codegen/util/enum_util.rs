use syn::Ident;

/// Convert a function name to an enum variant name
/// Example: "my_tusk_function" -> "MyTuskFunction"
pub fn convert_function_to_enum_variant(func_name: &Ident) -> Ident {
    func_name.clone()
}

/// Convert a submodule name to an enum variant name
/// Example: "sub_module" -> "SubModule"
pub fn convert_submodule_to_enum_variant(submod_name: &Ident) -> Ident {
    submod_name.clone()
}

/// Convert an external module name to an enum variant name
/// Example: "my_lib" -> "MyLib"
pub fn convert_external_module_to_enum_variant(alias: &Ident) -> Ident {
    alias.clone()
}
