use proc_macro2::TokenStream;
use syn::{Attribute, parse_quote, spanned::Spanned};

use crate::models::{
    Tusk,
    ExternalModule,
    TusksModule
};

use quote::quote;

impl TusksModule {
    /// Example 1 - Root module of tusks
    /// ```rust
    /// #[command(name = "tasks")] /// <===== here =====
    ///     pub struct Cli {
    ///         #[arg(long)]
    ///         pub root_param: String,
    ///         #[arg(short, long)]
    ///         pub verbose: bool,
    ///         #[command(subcommand)] /// see generate_command_attribute_for_subcommands
    ///         pub sub: Option<Commands>,
    ///     }
    /// ```
    ///
    /// Example 2 - This is a submodule-subcommand:
    /// ```rust
    /// pub enum Commands {
    ///     /// ... other subcommands and submodule-subcommands
    ///     #[command(name = "level1")] /// <===== here =====
    ///     #[allow(non_camel_case_types)]
    ///     level1 {
    ///         #[arg(long)]
    ///         level1_field: Option<String>,
    ///         #[arg(long, default_value = "42")]
    ///         level1_number: i32,
    ///         #[command(subcommand)] /// see generate_command_attribute_for_subcommands
    ///         sub: Option<level1::Commands>,
    ///     },
    /// }
    /// ```
    pub fn generate_command_attribute(&self) -> TokenStream {
        let existing_attrs = self.extract_attributes(&["command"]);

        if !existing_attrs.is_empty() {
            // Use existing command attribute
            quote! { #(#existing_attrs)* }
        } else {
            // Generate default command attribute
            quote! { #[command()] }
        }
    }

    /// Example 1 - Root module of tusks
    /// ```rust
    /// #[command(name = "tasks")] /// see generate_command_attribute
    ///     pub struct Cli {
    ///         #[arg(long)]
    ///         pub root_param: String,
    ///         #[arg(short, long)]
    ///         pub verbose: bool,
    ///         #[command(subcommand)] /// <===== here =====
    ///         pub sub: Option<Commands>,
    ///     }
    /// ```
    ///
    /// Example 2 - This is a submodule-subcommand:
    /// ```rust
    /// pub enum Commands {
    ///     /// ... other subcommands and submodule-subcommands
    ///     #[command(name = "level1")] /// see generate_command_attribute
    ///     #[allow(non_camel_case_types)]
    ///     level1 {
    ///         #[arg(long)]
    ///         level1_field: Option<String>,
    ///         #[arg(long, default_value = "42")]
    ///         level1_number: i32,
    ///         #[command(subcommand)] /// <===== here =====
    ///         sub: Option<level1::Commands>,
    ///     },
    /// }
    /// ```
    pub fn generate_command_attribute_for_subcommands(&self) -> TokenStream {
        let existing_attrs = self.extract_attributes(&["subcommands"]);

        if !existing_attrs.is_empty() {
            transform_attributes_to_command(existing_attrs, "subcommand")
        } else {
            quote! { #[command(subcommand)] }
        }
    }

    /// Example:
    /// ```rust
    /// pub enum Commands {
    ///     // ... other non-external subcommands ...
    ///     #[command(flatten)]
    ///     TuskExternalCommands(ExternalCommands),
    /// }
    /// ```
    pub fn generate_command_attribute_for_external_subcommands(&self) -> TokenStream {
        let existing_attrs = self.extract_attributes(&["external_subcommands"]);
        
        if !existing_attrs.is_empty() {
            transform_attributes_to_command(existing_attrs, "flatten")
        } else {
            quote! { #[command(flatten)] }
        }
    }
}

impl Tusk {
    pub fn generate_command_attribute(&self) -> TokenStream {
        let existing_attrs = self.extract_attributes(&["command"]);
        use_attributes_or_default(&existing_attrs, quote! { #[command()] })
    }
}

impl ExternalModule {
    /// Example:
    /// ```rust
    /// pub enum ExternalCommands {
    ///    #[command(name = "ext2")]
    ///    #[allow(non_camel_case_types)]
    ///    ext2(super::super::ext2::__internal_tusks_module::cli::Cli),
    ///}
    ///```
    pub fn generate_command_attribute(&self) -> TokenStream {
        let existing_attrs = self.extract_attributes(&["command"]);
        use_attributes_or_default(&existing_attrs, quote! { #[command()] })
    }
}

fn use_attributes_or_default(attrs: &[&Attribute], default: TokenStream) -> TokenStream {
    if !attrs.is_empty() {
        quote! { #(#attrs)* }
    } else {
        default
    }
}

/// Helper function to transform attributes from one form to another while preserving spans.
/// 
/// Transforms attributes like `#[source_name(params)]` to `#[command(target_keyword, params)]`
/// 
/// # Arguments
/// * `attrs` - The attributes to transform
/// * `target_keyword` - The keyword to use in the command attribute (e.g., "subcommand", "flatten")
/// 
/// # Examples
/// * `#[subcommands(arg1, arg2)]` with target "subcommand" → `#[command(subcommand, arg1, arg2)]`
/// * `#[external_subcommands]` with target "flatten" → `#[command(flatten)]`
fn transform_attributes_to_command(attrs: Vec<&syn::Attribute>, target_keyword: &str) -> TokenStream {
    let mut result = TokenStream::new();
    let mut target_ident = syn::Ident::new(target_keyword, proc_macro2::Span::call_site());
    
    for attr in attrs {
        let pound_span = attr.pound_token.span;
        let bracket_span = attr.bracket_token.span;

        target_ident.set_span(attr.span());
        
        // Parse the tokens inside the attribute
        if let syn::Meta::List(meta_list) = &attr.meta {
            let inner_tokens = &meta_list.tokens;
            
            // Create new attribute: #[command(target_keyword, inner_tokens)]
            let new_attr: syn::Attribute = parse_quote! {
                #[command(#target_ident, #inner_tokens)]
            };
            
            // Transfer the original spans
            let mut new_attr_with_span = new_attr;
            new_attr_with_span.pound_token.span = pound_span;
            new_attr_with_span.bracket_token.span = bracket_span;
            
            result.extend(quote! { #new_attr_with_span });
        } else {
            // If it's just #[attr_name] without parameters
            let new_attr: syn::Attribute = parse_quote! {
                #[command(#target_ident)]
            };
            
            let mut new_attr_with_span = new_attr;
            new_attr_with_span.pound_token.span = pound_span;
            new_attr_with_span.bracket_token.span = bracket_span;
            
            result.extend(quote! { #new_attr_with_span });
        }
    }
    
    result
}
