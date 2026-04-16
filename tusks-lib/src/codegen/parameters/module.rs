use syn::{Field, Fields, GenericParam, ItemMod, ItemStruct, Lifetime, Type};
use quote::quote;
use proc_macro2::Span;

use crate::{TusksModule, models::TusksParameters};

impl TusksModule {
    /// Supplement Parameters structs where missing and add super_ fields
    pub fn supplement_parameters(
        &mut self, 
        module: &mut ItemMod, 
        is_tusks_root: bool,
        derive_debug: bool,
    ) -> syn::Result<()> {
        // 1. Get or create Parameters struct with lifetime
        let lifetime = if let Some(ref params) = self.parameters {
            // Extract lifetime from existing struct
            Self::extract_lifetime(&params.pstruct)?
        } else {
            self.add_parameters_struct(module, derive_debug)?
        };

        let mut parameters_struct = Self::find_parameters_struct_mut(module)?;
        
        // 2. Add super_ field if needed
        if !is_tusks_root {
            self.add_super_field_to_parameters_struct(
                &mut parameters_struct,
                &lifetime)?;
        }

        Self::add_phantom_field_to_struct(&mut parameters_struct, &lifetime)?;

        // Update our internal structure
        if let Some(ref mut params) = self.parameters {
            params.pstruct = parameters_struct.clone();
        }
        
        // 3. Recursively process submodules
        if let Some((_, ref mut items)) = module.content {
            for submodule_data in &mut self.submodules {
                // Find corresponding ItemMod in module items
                if let Some(item_mod) = items.iter_mut().find_map(|item| {
                    if let syn::Item::Mod(m) = item {
                        if m.ident == submodule_data.name {
                            return Some(m);
                        }
                    }
                    None
                }) {
                    // Recursively supplement (submodules are never tusks root)
                    submodule_data.supplement_parameters(
                        item_mod,
                        false,
                        derive_debug
                    )?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract the first lifetime parameter from a struct
    pub fn extract_lifetime(item_struct: &ItemStruct) -> syn::Result<Lifetime> {
        for param in &item_struct.generics.params {
            if let GenericParam::Lifetime(lifetime_param) = param {
                return Ok(lifetime_param.lifetime.clone());
            }
        }
        
        // If no lifetime found, return error
        Err(syn::Error::new_spanned(
            &item_struct.ident,
            "Parameters struct must have a lifetime parameter"
        ))
    }
    
    /// Create a new empty Parameters struct with the given lifetime
    fn add_parameters_struct(
        &mut self,
        module: &mut ItemMod, 
        derive_debug: bool
    ) -> syn::Result<Lifetime> {
        let lifetime = Lifetime::new("'a", Span::call_site());

        let lifetime_param = { quote! {<#lifetime>} };

        let derive_attr =
            if derive_debug { quote! { #[derive(Debug)] } }
            else { quote! {} };

        let tokens = quote! {
            #derive_attr
            pub struct Parameters #lifetime_param {
            }
        };

        let params_struct: ItemStruct = syn::parse2(tokens).map_err(|e| {
            syn::Error::new(Span::call_site(), format!("Failed to create Parameters struct: {}", e))
        })?;

        Self::add_struct_to_module(module, params_struct.clone())?;

        self.parameters = Some(TusksParameters{ pstruct: params_struct });

        Ok(lifetime)
    }
    
    /// Add a struct to the module's items
    fn add_struct_to_module(module: &mut ItemMod, item_struct: ItemStruct) -> syn::Result<()> {
        if let Some((_, ref mut items)) = module.content {
            items.insert(0, syn::Item::Struct(item_struct));
            Ok(())
        } else {
            Err(syn::Error::new_spanned(
                &module.ident,
                "Module has no content"
            ))
        }
    }
    
    fn find_parameters_struct_mut(module: &mut ItemMod) -> syn::Result<&mut ItemStruct> {
        if let Some((_, ref mut items)) = module.content {
            for item in items.iter_mut() {
                if let syn::Item::Struct(s) = item {
                    if s.ident == "Parameters" {
                        return Ok(s);
                    }
                }
            }
        }

        Err(syn::Error::new_spanned(
            &module.ident,
            "Parameters struct not found in module"
        ))
    }

    /// Add super_ field to the Parameters struct in the module
    fn add_super_field_to_parameters_struct(
        &mut self,
        parameters_struct: &mut ItemStruct,
        lifetime: &Lifetime
    ) -> syn::Result<()> {
        // Determine the type of super_ based on whether we have an external parent
        let super_type = if self.external_parent.is_some() {
            // Local root with external parent: &'lifetime parent_::Parameters<'lifetime>
            quote! { &#lifetime parent_::Parameters<#lifetime> }
        } else {
            // Submodule: &'lifetime super::Parameters<'lifetime>
            quote! { &#lifetime super::Parameters<#lifetime> }
        };

        // Parse the type
        let super_field_type: Type = syn::parse2(super_type).map_err(|e| {
            syn::Error::new(Span::call_site(), format!("Failed to parse super_ type: {}", e))
        })?;

        // Create the super_ field
        let super_field = Field {
            attrs: vec![],
            vis: syn::Visibility::Public(syn::token::Pub::default()),
            mutability: syn::FieldMutability::None,
            ident: Some(syn::Ident::new("super_", Span::call_site())),
            colon_token: Some(syn::token::Colon::default()),
            ty: super_field_type,
        };

        // Add super_ field
        if let Fields::Named(ref mut fields) = parameters_struct.fields {
            fields.named.push(super_field);
        } else {
            return Err(syn::Error::new_spanned(
                &parameters_struct.ident,
                "Parameters struct must have named fields"
            ));
        }

        Ok(())
    }

    fn add_phantom_field_to_struct(
        item_struct: &mut ItemStruct,
        lifetime: &Lifetime
    ) -> syn::Result<()> {
        use syn::{Field, Type, Ident};

        // Parse nur den Typ
        let phantom_type: Type = syn::parse2(quote! {
            ::std::marker::PhantomData<&#lifetime ()>
        })?;

        // Konstruiere das Field direkt
        let phantom_field = Field {
            attrs: vec![],
            vis: syn::Visibility::Public(syn::token::Pub::default()),
            mutability: syn::FieldMutability::None,
            ident: Some(Ident::new("_phantom_lifetime_marker", Span::call_site())),
            colon_token: Some(Default::default()),
            ty: phantom_type,
        };

        // Füge das Feld zu den struct fields hinzu
        match &mut item_struct.fields {
            syn::Fields::Named(fields) => {
                fields.named.push(phantom_field);
            }
            syn::Fields::Unnamed(_) => {
                return Err(syn::Error::new_spanned(
                    item_struct,
                    "Cannot add named field to tuple struct"
                ));
            }
            syn::Fields::Unit => {
                item_struct.fields = syn::Fields::Named(syn::FieldsNamed {
                    brace_token: Default::default(),
                    named: {
                        let mut fields = syn::punctuated::Punctuated::new();
                        fields.push(phantom_field);
                        fields
                    }
                });
            }
        }

        Ok(())
    }
}
