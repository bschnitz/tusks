use syn::Attribute;

use crate::{TusksModule, models::{ExternalModule, Tusk, TusksParameters}};

impl TusksParameters {
    pub fn extract_attributes<'a>(&'a self, names: &[&str]) -> Vec<&'a Attribute> {
        self
            .pstruct
            .attrs
            .iter()
            .filter(|attr| {
                attr.path().get_ident()
                    .map(|ident| names.contains(&ident.to_string().as_str()))
                    .unwrap_or(false)
            })
            .collect()
    }

}

impl Tusk {
    pub fn extract_attributes<'a>(&'a self, names: &[&str]) -> Vec<&'a Attribute> {
        self.func.attrs.iter().filter(|attr| {
            attr.path().get_ident()
                .map(|ident| names.contains(&ident.to_string().as_str()))
                .unwrap_or(false)
        }).collect()
    }
}

impl ExternalModule {
    pub fn extract_attributes<'a>(&'a self, names: &[&str]) -> Vec<&'a Attribute> {
        self.item_use.attrs.iter().filter(|attr| {
            attr.path().get_ident()
                .map(|ident| names.contains(&ident.to_string().as_str()))
                .unwrap_or(false)
        }).collect()
    }
}

impl TusksModule {
    pub fn extract_attributes<'a>(&'a self, names: &[&str]) -> Vec<&'a Attribute> {
        self.attrs.0.iter().filter(|attr| {
            attr.path().get_ident()
                .map(|ident| names.contains(&ident.to_string().as_str()))
                .unwrap_or(false)
        }).collect()
    }
}
