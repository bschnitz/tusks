use proc_macro2::{Span, TokenStream};
use quote::quote;

/// Tracks the current position in the module hierarchy during code generation.
///
/// Used throughout codegen to build correct `cli::path::to::Commands` and
/// `super::path::to::function` token paths. Replaces the raw `&[&str]`
/// that was previously threaded through every function.
#[derive(Clone, Default)]
pub struct ModulePath {
    segments: Vec<String>,
}

impl ModulePath {
    pub fn new() -> Self {
        Self { segments: Vec::new() }
    }

    /// Return a new path with `name` appended.
    pub fn join(&self, name: &str) -> Self {
        let mut new = self.clone();
        new.segments.push(name.to_string());
        new
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// The last segment, if any (used for error messages like "Subcommand required for X!").
    pub fn last(&self) -> Option<&str> {
        self.segments.last().map(|s| s.as_str())
    }

    fn to_idents(&self) -> Vec<syn::Ident> {
        self.segments.iter()
            .map(|s| syn::Ident::new(s, Span::call_site()))
            .collect()
    }

    /// Build a `cli::path::to` token path for matching on Commands enums.
    /// Empty → `cli`, non-empty → `cli::seg1::seg2`.
    pub fn cli_path(&self) -> TokenStream {
        if self.is_empty() {
            quote! { cli }
        } else {
            let idents = self.to_idents();
            quote! { cli::#(#idents)::* }
        }
    }

    /// Build a `super::path::to` token path for calling functions.
    /// Empty → `super::`, non-empty → `super::seg1::seg2::`.
    pub fn super_path(&self) -> TokenStream {
        if self.is_empty() {
            quote! { super:: }
        } else {
            let idents = self.to_idents();
            quote! { super::#(#idents)::* :: }
        }
    }

    /// Build a `super::path::to::name` token path for a specific item.
    pub fn super_path_to(&self, name: &syn::Ident) -> TokenStream {
        if self.is_empty() {
            quote! { super::#name }
        } else {
            let idents = self.to_idents();
            quote! { super::#(#idents)::*::#name }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_path() {
        let p = ModulePath::new();
        assert!(p.is_empty());
        assert!(p.last().is_none());
    }

    #[test]
    fn join_creates_new_path() {
        let p = ModulePath::new().join("git").join("advanced");
        assert!(!p.is_empty());
        assert_eq!(p.last(), Some("advanced"));
    }

    #[test]
    fn join_does_not_mutate_original() {
        let p = ModulePath::new();
        let _ = p.join("git");
        assert!(p.is_empty());
    }

    #[test]
    fn cli_path_empty() {
        let p = ModulePath::new();
        assert_eq!(p.cli_path().to_string(), "cli");
    }

    #[test]
    fn cli_path_nested() {
        let p = ModulePath::new().join("git").join("advanced");
        assert_eq!(p.cli_path().to_string(), "cli :: git :: advanced");
    }

    #[test]
    fn super_path_to_empty() {
        let name = syn::Ident::new("my_func", Span::call_site());
        let p = ModulePath::new();
        assert_eq!(p.super_path_to(&name).to_string(), "super :: my_func");
    }

    #[test]
    fn super_path_to_nested() {
        let name = syn::Ident::new("my_func", Span::call_site());
        let p = ModulePath::new().join("git");
        assert_eq!(p.super_path_to(&name).to_string(), "super :: git :: my_func");
    }
}
