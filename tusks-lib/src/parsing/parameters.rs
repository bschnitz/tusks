use syn::ItemStruct;
use crate::parsing::util::attr::AttributeCheck;

use crate::models::TusksParameters;

impl TusksParameters {
    pub fn from_struct(item_struct: ItemStruct) -> syn::Result<Option<Self>> {
        // Check if struct name is "Parameters"
        if item_struct.ident != "Parameters" {
            return Ok(None);
        }
        
        if item_struct.has_attr("skip") {
            return Ok(None);
        }

        // Validate that the struct is public
        if !matches!(item_struct.vis, syn::Visibility::Public(_)) {
            return Err(syn::Error::new_spanned(
                &item_struct.ident,
                "Parameters struct must be public"
            ));
        }

        // Validate that all fields are references
        for field in &item_struct.fields {
            // Check for super_ field, which is not allowed
            if let Some(field_name) = &field.ident {
                if field_name == "super_" {
                    return Err(syn::Error::new_spanned(
                        field_name,
                        "super_ field is not allowed in Parameters struct. \
                            It will be added programmatically."
                    ));
                }
            }
            
            if !Self::is_reference_type(&field.ty) {
                return Err(syn::Error::new_spanned(
                    &field.ty,
                    "all fields in Parameters struct must be & references"
                ));
            }
        }

        Ok(Some(TusksParameters {
            pstruct: item_struct,
        }))
    }

    /// Check if a type is a reference
    fn is_reference_type(ty: &syn::Type) -> bool {
        matches!(ty, syn::Type::Reference(_))
    }
}
