use syn::{Attribute, Ident, ItemFn, ItemStruct, PathSegment, spanned::Spanned};

#[derive(Default)]
pub struct Attributes(pub Vec<Attribute>);

/// Represents a tusks module with all its elements
#[derive(Debug)]
pub struct TusksModule {
    /// Name of the module (e.g. "tasks", "sub1")
    pub name: Ident,

    pub attrs: Attributes,

    /// The parent annotated as pub use ... as parent_
    pub external_parent: Option<ExternalModule>,
    
    /// The Parameters struct (if not existing during parse it will be generated)
    pub parameters: Option<TusksParameters>,
    
    /// List of all public functions
    pub tusks: Vec<Tusk>,
    
    /// List of all pub sub-modules (recursive)
    pub submodules: Vec<TusksModule>,
    
    /// List of all external modules (pub use ... as ...)
    pub external_modules: Vec<ExternalModule>,

    /// if #[command(allow_external_subcommands=true)] is set
    pub allow_external_subcommands: bool,
}

/// Represents a parameters struct
pub struct TusksParameters {
    /// The underlying struct
    pub pstruct: ItemStruct,
}

/// Represents a command function (tusk)
pub struct Tusk {
    /// The underlying function
    pub func: ItemFn,

    pub is_default: bool
}

/// Represents an externally imported module
pub struct ExternalModule {
    /// The alias name (e.g. "sub2")
    pub alias: Ident,
    
    /// The original pub use statement that created this external module
    pub item_use: syn::ItemUse,
}

impl std::fmt::Debug for TusksParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TusksParameters")
            .field("name", &self.pstruct.ident.to_string())
            .finish()
    }
}

impl std::fmt::Debug for Tusk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tusk")
            .field("name", &self.func.sig.ident.to_string())
            .finish()
    }
}

impl std::fmt::Debug for ExternalModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalModule")
            .field("alias", &self.alias.to_string())
            .field("item_use", &format!("ItemUse at span: {:?}", self.item_use.span()))
            .finish()
    }
}

impl std::fmt::Debug for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Attributes")
            .field("count", &self.0.len())
            .field(
                "attributes",
                &self.0.iter().map(|attr| {
                    let path_str = attr
                        .path()
                        .segments
                        .iter()
                        .map(|seg: &PathSegment| seg.ident.to_string())
                        .collect::<Vec<_>>()
                        .join("::");

                    format!("{} at span: {:?}", path_str, attr.span())
                }).collect::<Vec<String>>(),
            )
            .finish()
    }
}
