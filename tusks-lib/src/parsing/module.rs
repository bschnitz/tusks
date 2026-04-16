use crate::{models::{Attributes, ExternalModule, Tusk, TusksModule, TusksParameters}, parsing::util::get_attribute_value::AttributeValue};
use syn::spanned::Spanned;
use crate::parsing::util::attr::AttributeCheck;

use syn::{ItemMod, ItemStruct};

impl TusksModule {
    /// Parses a syn::ItemMod into a TusksModule
    pub fn from_module(module: ItemMod, is_tusks_root: bool, is_root: bool) -> syn::Result<Option<Self>> {
        let allow_external_subcommands = module.get_attribute_bool(
            "command",
            "allow_external_subcommands"
        );

        let name = module.ident.clone();
        let span = module.span();

        // Validate that the module is public
        if !matches!(module.vis, syn::Visibility::Public(_)) {
            if module.has_attr("tusks") {
                return Err(syn::Error::new_spanned(&name, "tusks module must be public"));
            }
            return Ok(None);
        }

        if module.has_attr("skip") {
            return Ok(None);
        }
        
        // Check if module has content (inline module)
        let items = match module.content {
            Some(content) => content.1, // content is (brace_token, Vec<Item>)
            None => {
                return Err(syn::Error::new(
                    span,
                    "tusks module must be inline (not a file reference)"
                ));
            }
        };

        let mut tusks_module = TusksModule {
            name,
            attrs: Attributes(module.attrs),
            external_parent: None,
            parameters: None,
            tusks: Vec::new(),
            submodules: Vec::new(),
            external_modules: Vec::new(),
            allow_external_subcommands,
        };
        
        tusks_module.extract_module_items(items, is_root)?;

        tusks_module.validate_is_root_or_has_parent(is_tusks_root, is_root)?;
        
        Ok(Some(tusks_module))
    }

    fn validate_is_root_or_has_parent(&self, is_tusks_root: bool, is_root: bool) -> syn::Result<()> {
        if !is_root {
            return Ok(());
        }

        if !is_tusks_root {
            if !self.external_parent.is_some() {
                return Err(syn::Error::new_spanned(
                    &self.name,
                    "A tusks module must either be root \
                        or must declare a parent via `pub use path::to::parent::module as parent_`."
                ));
            }
        }
        else {
            if let Some(parent) = &self.external_parent {
                let mut err = syn::Error::new_spanned(
                    &self.name,
                    "A tusks module must either be root or declare a parent but not both."
                );
                err.combine(syn::Error::new_spanned(&parent.alias, "Parent is declared here."));
                return Err(err);
            }
        }

        return Ok(());
    }
    
    /// Extract all relevant items from a module
    fn extract_module_items(&mut self, items: Vec<syn::Item>, is_root: bool) -> syn::Result<()> {
        let mut has_default_tusk = false;
        for item in items {
            match item {
                syn::Item::Struct(item_struct) => {
                    self.parse_struct(item_struct.clone())?;
                }

                syn::Item::Fn(item_fn) => {
                    if let Some(tusk) = Tusk::from_fn(
                        item_fn.clone(),
                        has_default_tusk,
                        self.allow_external_subcommands
                    )? {
                        has_default_tusk = has_default_tusk || tusk.is_default;
                        self.tusks.push(tusk);
                    }
                }
                
                syn::Item::Mod(item_mod) => {
                    if let Some(module) = Self::from_module(item_mod.clone(), false, false)? {
                        self.submodules.push(module);
                    }
                }
                
                syn::Item::Use(item_use) => {
                    // Only consider pub use
                    if matches!(item_use.vis, syn::Visibility::Public(_)) {
                        // Extract external modules
                        self.extract_external_modules(&item_use.tree, &item_use, is_root);
                    }
                }
                
                _ => {
                    // Ignore other items
                }
            }
        }
        Ok(())
    }
    
    /// Parse a struct and check if it's a parameters struct
    fn parse_struct(&mut self, item_struct: ItemStruct) -> syn::Result<()> {
        if let Some(params) = TusksParameters::from_struct(item_struct)? {
            self.parameters = Some(params);
        }
        Ok(())
    }
    
/// Extract external module names from a use tree
fn extract_external_modules(
    &mut self,
    tree: &syn::UseTree,
    item_use: &syn::ItemUse,
    is_root: bool
) {
    match tree {
        syn::UseTree::Path(use_path) => {
            // use foo::<rest>
            self.extract_external_modules(&use_path.tree, item_use, is_root);
        }
        syn::UseTree::Name(use_name) => {
            // Check if it's parent_
            if use_name.ident == "parent_" && is_root {
                self.external_parent = Some(ExternalModule {
                    alias: use_name.ident.clone(),
                    item_use: item_use.clone(),
                });
            } else {
                self.external_modules.push(ExternalModule {
                    alias: use_name.ident.clone(),
                    item_use: item_use.clone(),
                });
            }
        }
        syn::UseTree::Rename(use_rename) => {
            // Check if this is the reference to the parent module
            if use_rename.rename == "parent_" {
                self.external_parent = Some(ExternalModule {
                    alias: use_rename.rename.clone(),
                    item_use: item_use.clone(),
                });
            } else {
                self.external_modules.push(ExternalModule {
                    alias: use_rename.rename.clone(),
                    item_use: item_use.clone(),
                });
            }
        }
        syn::UseTree::Glob(_) => {
            // use foo::* => ignore
        }
        syn::UseTree::Group(use_group) => {
            // e.g. use foo::{bar, baz};
            for item in &use_group.items {
                self.extract_external_modules(item, item_use, is_root);
            }
        }
    }
}
}
