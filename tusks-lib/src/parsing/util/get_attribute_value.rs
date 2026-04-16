use syn::{Expr, Lit, Meta};

use crate::parsing::util::attr::HasAttributes;

/// Public trait for attribute value extraction
pub trait AttributeValue {
    fn get_attribute_value(&self, attr_name: &str, key: &str) -> Option<String>;

    fn get_attribute_bool(&self, attr_name: &str, key: &str) -> bool {
        self.get_attribute_value(attr_name, key)
            .map(|v| v == "true")
            .unwrap_or(false)
    }
}

/* -------------------------------------------------------
 * Generic AttributeValue implementation
 * -------------------------------------------------------*/
impl<T: HasAttributes> AttributeValue for T {
    fn get_attribute_value(&self, attr_name: &str, key: &str) -> Option<String> {
        // Find the matching attribute
        let attr = self.attrs().iter().find(|attr| {
            attr.path().segments.last()
                .map(|seg| seg.ident == attr_name)
                .unwrap_or(false)
        })?;

        // Parse the attribute meta
        let meta = attr.meta.clone();
        
        match meta {
            // #[attr(key1, key2 = value, ...)]
            Meta::List(list) => {
                // Parse tokens manually
                let tokens = list.tokens.clone();
                let parser = |input: syn::parse::ParseStream| {
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated(input)
                };
                let parsed = syn::parse::Parser::parse2(parser, tokens);
                
                if let Ok(nested) = parsed {
                    for meta in nested {
                        match meta {
                            // Case 1: #[attr(key)] -> key exists as flag
                            Meta::Path(path) => {
                                if path.segments.last()?.ident == key {
                                    return Some("true".to_string());
                                }
                            }
                            // Case 2: #[attr(key = value)]
                            Meta::NameValue(nv) => {
                                if nv.path.segments.last()?.ident == key {
                                    return Some(extract_value(&nv.value));
                                }
                            }
                            // Case 3: #[attr(key(...))] - nested
                            Meta::List(inner_list) => {
                                if inner_list.path.segments.last()?.ident == key {
                                    // Return the inner tokens as string
                                    return Some(inner_list.tokens.to_string());
                                }
                            }
                        }
                    }
                }
            }
            // #[attr = value] - direct name-value
            Meta::NameValue(nv) => {
                if key == attr_name {
                    return Some(extract_value(&nv.value));
                }
            }
            // #[attr] - just the path
            Meta::Path(_) => {
                if key == attr_name {
                    return Some("true".to_string());
                }
            }
        }
        
        None
    }
}

/// Extract string representation from Expr
fn extract_value(expr: &Expr) -> String {
    match expr {
        Expr::Lit(lit_expr) => {
            match &lit_expr.lit {
                Lit::Str(s) => s.value(),
                Lit::Bool(b) => b.value.to_string(),
                Lit::Int(i) => i.base10_digits().to_string(),
                Lit::Float(f) => f.base10_digits().to_string(),
                Lit::Char(c) => c.value().to_string(),
                _ => quote::quote!(#expr).to_string(),
            }
        }
        _ => quote::quote!(#expr).to_string(),
    }
}

/* -------------------------------------------------------
 * Tests
 * -------------------------------------------------------*/
#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, ItemStruct};

    #[test]
    fn test_attribute_values() {
        let item: ItemStruct = parse_quote! {
            #[demo(flag1, flag2 = false, flag3 = "hello", flag4 = 42)]
            struct Test;
        };

        assert_eq!(item.get_attribute_value("demo", "flag1"), Some("true".to_string()));
        assert_eq!(item.get_attribute_value("demo", "flag2"), Some("false".to_string()));
        assert_eq!(item.get_attribute_value("demo", "flag3"), Some("hello".to_string()));
        assert_eq!(item.get_attribute_value("demo", "flag4"), Some("42".to_string()));
        assert_eq!(item.get_attribute_value("demo", "nonexistent"), None);
    }

    #[test]
    fn test_numeric_values() {
        let item: ItemStruct = parse_quote! {
            #[config(num = 42, float = 3.14, ch = 'x')]
            struct Test;
        };

        assert_eq!(item.get_attribute_value("config", "num"), Some("42".to_string()));
        assert_eq!(item.get_attribute_value("config", "float"), Some("3.14".to_string()));
        assert_eq!(item.get_attribute_value("config", "ch"), Some("x".to_string()));
    }

    #[test]
    fn test_nested_attributes() {
        let item: ItemStruct = parse_quote! {
            #[serde(rename_all = "camelCase", deny_unknown_fields)]
            struct Test;
        };

        assert_eq!(item.get_attribute_value("serde", "rename_all"), Some("camelCase".to_string()));
        assert_eq!(item.get_attribute_value("serde", "deny_unknown_fields"), Some("true".to_string()));
    }
}
