use syn::{Ident, LitBool, LitInt, LitStr, Token, parenthesized, parse::{Parse, ParseStream}};

use crate::parsing::attribute::models::{TasksConfig, TusksAttr};

impl Parse for TusksAttr {
    /// Parses the `#[tusks(...)]` attribute and extracts all configuration options.
    /// 
    /// Supports the following syntax:
    /// - Boolean flags: `debug`, `root`, `derive_debug_for_parameters`
    ///   - Can be specified as just the flag name (implies `true`)
    ///   - Or with explicit value: `debug = true` or `debug = false`
    /// - Nested configuration: `tasks(max_groupsize=5, max_depth=20, separator=".")`
    /// 
    /// # Example
    /// ```ignore
    /// #[tusks(root, debug, tasks(max_groupsize=10, separator="/"))]
    /// ```
    /// 
    /// # Errors
    /// Returns an error if:
    /// - An unknown attribute name is encountered
    /// - The syntax is malformed (missing commas, invalid values, etc.)
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attr = TusksAttr::default();
        
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            
            match ident.to_string().as_str() {
                "debug" => attr.debug = parse_bool_flag(input)?,
                "root" => attr.root = parse_bool_flag(input)?,
                "derive_debug_for_parameters" => {
                    attr.derive_debug_for_parameters = parse_bool_flag(input)?
                },
                "tasks" => {
                    attr.tasks = Some(parse_optional_nested_config::<TasksConfig>(input)?);
                },
                other => return Err(unknown_attribute_error(&ident, other)),
            }
            
            parse_trailing_comma(input)?;
        }
        
        Ok(attr)
    }
}

/// Parse an optional nested configuration:
/// - `ident(...)` → parses `T`
/// - `ident`      → returns `T::default()`
fn parse_optional_nested_config<T>(input: ParseStream) -> syn::Result<T>
where
    T: Parse + Default,
{
    if input.peek(syn::token::Paren) {
        parse_nested_config(input)
    } else {
        Ok(T::default())
    }
}

impl Parse for TasksConfig {
    /// Parses the task configuration parameters inside `tasks(...)`.
    /// 
    /// All parameters are optional and will use default values if not specified:
    /// - `max_groupsize`: defaults to 5
    /// - `max_depth`: defaults to 20
    /// - `separator`: defaults to "."
    /// 
    /// # Example
    /// ```ignore
    /// tasks(max_groupsize=10, separator="/")
    /// // Results in: max_groupsize=10, max_depth=20 (default), separator="/"
    /// ```
    /// 
    /// # Errors
    /// Returns an error if:
    /// - An unknown parameter name is encountered
    /// - A parameter value has the wrong type (e.g., string instead of integer)
    /// - The syntax is malformed (missing `=`, invalid literals, etc.)
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut config = TasksConfig::default();
        
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            
            match ident.to_string().as_str() {
                "max_groupsize" => config.max_groupsize = parse_required_value(input, parse_usize)?,
                "max_depth" => config.max_depth = parse_required_value(input, parse_usize)?,
                "separator" => config.separator = parse_required_value(input, parse_string)?,
                "use_colors" => config.use_colors = parse_bool_flag(input)?,
                other => return Err(unknown_parameter_error(&ident, other)),
            }
            
            parse_trailing_comma(input)?;
        }
        
        Ok(config)
    }
}

// Helper functions

/// Parse an optional boolean flag that can be either `flag` or `flag = true/false`
fn parse_bool_flag(input: ParseStream) -> syn::Result<bool> {
    if input.peek(Token![=]) {
        input.parse::<Token![=]>()?;
        let value: LitBool = input.parse()?;
        Ok(value.value)
    } else {
        Ok(true)
    }
}

/// Parse a required parameter value: `= <value>`
fn parse_required_value<T, F>(input: ParseStream, parser: F) -> syn::Result<T>
where
    F: FnOnce(ParseStream) -> syn::Result<T>,
{
    input.parse::<Token![=]>()?;
    parser(input)
}

/// Parse a nested configuration like `tasks(...)`
fn parse_nested_config<T: Parse>(input: ParseStream) -> syn::Result<T> {
    let content;
    parenthesized!(content in input);
    content.parse::<T>()
}

/// Parse a trailing comma if present
fn parse_trailing_comma(input: ParseStream) -> syn::Result<()> {
    if !input.is_empty() {
        input.parse::<Token![,]>()?;
    }
    Ok(())
}

/// Parse a usize literal
fn parse_usize(input: ParseStream) -> syn::Result<usize> {
    let value: LitInt = input.parse()?;
    value.base10_parse()
}

/// Parse a string literal
fn parse_string(input: ParseStream) -> syn::Result<String> {
    let value: LitStr = input.parse()?;
    Ok(value.value())
}

/// Create error for unknown attribute
fn unknown_attribute_error(ident: &Ident, name: &str) -> syn::Error {
    syn::Error::new(
        ident.span(),
        format!("unknown tusks attribute: {}", name)
    )
}

/// Create error for unknown parameter
fn unknown_parameter_error(ident: &Ident, name: &str) -> syn::Error {
    syn::Error::new(
        ident.span(),
        format!("unknown tasks parameter: {}", name)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_tusks_attr(input: &str) -> syn::Result<TusksAttr> {
        syn::parse_str::<TusksAttr>(input)
    }

    fn parse_tasks_config(input: &str) -> syn::Result<TasksConfig> {
        syn::parse_str::<TasksConfig>(input)
    }

    // --- TusksAttr parsing ---

    #[test]
    fn empty_attr() {
        let attr = parse_tusks_attr("").unwrap();
        assert!(!attr.debug);
        assert!(!attr.root);
        assert!(!attr.derive_debug_for_parameters);
        assert!(attr.tasks.is_none());
    }

    #[test]
    fn root_flag() {
        let attr = parse_tusks_attr("root").unwrap();
        assert!(attr.root);
        assert!(!attr.debug);
    }

    #[test]
    fn debug_flag() {
        let attr = parse_tusks_attr("debug").unwrap();
        assert!(attr.debug);
    }

    #[test]
    fn derive_debug_flag() {
        let attr = parse_tusks_attr("derive_debug_for_parameters").unwrap();
        assert!(attr.derive_debug_for_parameters);
    }

    #[test]
    fn explicit_bool_true() {
        let attr = parse_tusks_attr("root = true").unwrap();
        assert!(attr.root);
    }

    #[test]
    fn explicit_bool_false() {
        let attr = parse_tusks_attr("root = false").unwrap();
        assert!(!attr.root);
    }

    #[test]
    fn multiple_flags() {
        let attr = parse_tusks_attr("root, debug").unwrap();
        assert!(attr.root);
        assert!(attr.debug);
    }

    #[test]
    fn all_flags() {
        let attr = parse_tusks_attr("root, debug, derive_debug_for_parameters").unwrap();
        assert!(attr.root);
        assert!(attr.debug);
        assert!(attr.derive_debug_for_parameters);
    }

    #[test]
    fn unknown_attribute_errors() {
        let err = parse_tusks_attr("unknown").unwrap_err();
        assert!(err.to_string().contains("unknown tusks attribute"));
    }

    #[test]
    fn tasks_without_parens_uses_defaults() {
        let attr = parse_tusks_attr("root, tasks").unwrap();
        assert!(attr.root);
        let tasks = attr.tasks.unwrap();
        assert_eq!(tasks.max_groupsize, 5);
        assert_eq!(tasks.max_depth, 20);
        assert_eq!(tasks.separator, ".");
        assert!(tasks.use_colors);
    }

    #[test]
    fn tasks_with_custom_config() {
        let attr = parse_tusks_attr(
            "root, tasks(max_groupsize = 10, separator = \"/\", max_depth = 3)"
        ).unwrap();
        let tasks = attr.tasks.unwrap();
        assert_eq!(tasks.max_groupsize, 10);
        assert_eq!(tasks.max_depth, 3);
        assert_eq!(tasks.separator, "/");
    }

    #[test]
    fn tasks_use_colors_false() {
        let attr = parse_tusks_attr("tasks(use_colors = false)").unwrap();
        let tasks = attr.tasks.unwrap();
        assert!(!tasks.use_colors);
    }

    #[test]
    fn tasks_use_colors_flag_only() {
        let attr = parse_tusks_attr("tasks(use_colors)").unwrap();
        let tasks = attr.tasks.unwrap();
        assert!(tasks.use_colors);
    }

    // --- TasksConfig parsing ---

    #[test]
    fn tasks_config_defaults() {
        let config = parse_tasks_config("").unwrap();
        assert_eq!(config.max_groupsize, 5);
        assert_eq!(config.max_depth, 20);
        assert_eq!(config.separator, ".");
        assert!(config.use_colors);
    }

    #[test]
    fn tasks_config_partial_override() {
        let config = parse_tasks_config("max_groupsize = 3").unwrap();
        assert_eq!(config.max_groupsize, 3);
        assert_eq!(config.max_depth, 20); // default
    }

    #[test]
    fn tasks_config_unknown_parameter_errors() {
        let err = parse_tasks_config("unknown = 5").unwrap_err();
        assert!(err.to_string().contains("unknown tasks parameter"));
    }

    #[test]
    fn tasks_config_custom_separator() {
        let config = parse_tasks_config("separator = \"::\"").unwrap();
        assert_eq!(config.separator, "::");
    }
}
