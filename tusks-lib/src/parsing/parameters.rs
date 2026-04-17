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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_struct(code: &str) -> ItemStruct {
        syn::parse_str::<ItemStruct>(code).unwrap()
    }

    #[test]
    fn valid_parameters_struct() {
        let s = parse_struct("pub struct Parameters<'a> { pub name: &'a str }");
        let result = TusksParameters::from_struct(s).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn ignores_non_parameters_struct() {
        let s = parse_struct("pub struct Other { pub name: String }");
        let result = TusksParameters::from_struct(s).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn ignores_skipped_parameters() {
        let s = parse_struct("#[skip] pub struct Parameters<'a> { pub x: &'a str }");
        let result = TusksParameters::from_struct(s).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn rejects_private_parameters() {
        let s = parse_struct("struct Parameters<'a> { x: &'a str }");
        let err = TusksParameters::from_struct(s).unwrap_err();
        assert!(err.to_string().contains("must be public"));
    }

    #[test]
    fn rejects_non_reference_field() {
        let s = parse_struct("pub struct Parameters { pub name: String }");
        let err = TusksParameters::from_struct(s).unwrap_err();
        assert!(err.to_string().contains("must be & references"));
    }

    #[test]
    fn rejects_super_field() {
        let s = parse_struct("pub struct Parameters<'a> { pub super_: &'a str }");
        let err = TusksParameters::from_struct(s).unwrap_err();
        assert!(err.to_string().contains("super_ field is not allowed"));
    }

    #[test]
    fn accepts_empty_parameters() {
        let s = parse_struct("pub struct Parameters {}");
        let result = TusksParameters::from_struct(s).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn accepts_multiple_reference_fields() {
        let s = parse_struct(
            "pub struct Parameters<'a> { pub a: &'a str, pub b: &'a u32 }"
        );
        let result = TusksParameters::from_struct(s).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn rejects_mixed_ref_and_non_ref() {
        let s = parse_struct(
            "pub struct Parameters<'a> { pub a: &'a str, pub b: String }"
        );
        let err = TusksParameters::from_struct(s).unwrap_err();
        assert!(err.to_string().contains("must be & references"));
    }
}
