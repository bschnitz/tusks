# AGENTS.md - Tusks Macro Repository

## Build Commands
- `cargo build` - Build the project
- `cargo test` - Run all tests
- `cargo test --lib` - Run library tests only
- `cargo test --test <test_name>` - Run specific test
- `cargo check` - Quick syntax check
- `cargo clippy` - Run linter (requires clippy)
- `cargo fmt` - Format code (requires rustfmt)

## Code Style Guidelines

### Imports
- Group imports by crate: std, external crates, local modules
- Use absolute paths for local imports (e.g., `crate::module::item`)
- Sort imports alphabetically within groups

### Formatting
- Follow Rustfmt defaults
- Use 4 spaces for indentation
- Max line length: 100 characters
- Use trailing commas in function parameters and match arms

### Types & Naming
- Use snake_case for functions, variables, and module names
- Use PascalCase for types, traits, and structs
- Use SCREAMING_SNAKE_CASE for constants
- Use descriptive names (avoid single-letter variables)

### Error Handling
- Use `Result<T, E>` for fallible operations
- Return `Err(e.to_compile_error().into())` for proc-macro errors
- Use `?` operator for propagating errors
- Provide meaningful error messages

### Procedural Macros
- Implement `Parse` trait for attribute structs
- Use `parse_macro_input!` for parsing inputs
- Return `TokenStream` from macro functions
- Include debug output when `debug` feature is enabled

### Dependencies
- Use relative paths for local dependencies (e.g., `tusks-lib = { path = "../tusks-lib" }`)
- Keep dependencies minimal and version-locked

### Testing
- Write tests for macro behavior
- Use `#[cfg(test)]` for test-only code
- Test both success and error cases
- Include documentation examples in tests