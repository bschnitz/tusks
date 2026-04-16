# AGENTS.md - Guidelines for tusks-tasks Repository

## Build & Test Commands
- `cargo build` - Build the project
- `cargo check` - Quick syntax check
- `cargo test` - Run all tests
- `cargo test --lib` - Run library tests only
- `cargo test --bin` - Run binary tests only
- `cargo clippy` - Run linter
- `cargo fmt` - Format code according to Rust style

## Code Style Guidelines

### Imports
- Group imports by crate: standard library, external crates, local modules
- Use absolute paths for local modules (e.g., `crate::list::models`)
- Keep imports alphabetically sorted within groups

### Formatting
- Use Rustfmt defaults (cargo fmt)
- 4 spaces for indentation
- 80 characters line limit
- Use trailing commas in structs/enum definitions

### Types & Naming
- Use `snake_case` for functions and variables
- Use `PascalCase` for structs, enums, traits
- Use `SCREAMING_SNAKE_CASE` for constants
- Prefer `Result<T, E>` over custom error types
- Use `Option<T>` for optional values

### Error Handling
- Use `?` operator for propagating errors
- Provide clear error messages
- Consider using `anyhow` for complex error handling
- Document expected error cases

### Module Structure
- Follow Rust module system conventions
- Use `pub mod` for public modules
- Keep module files focused (single responsibility)
- Use submodules for related functionality

### Documentation
- Use Rust doc comments (`///`) for public APIs
- Document function parameters and return values
- Include examples when helpful
- Keep documentation concise and clear

### Testing
- Write unit tests for public functions
- Use `#[cfg(test)]` for test-only code
- Mock external dependencies when needed
- Test edge cases and error conditions

### Dependencies
- Keep dependencies minimal and up-to-date
- Use version ranges in Cargo.toml
- Document why each dependency is needed
- Consider using `features` for optional dependencies

## Essential Rules (Mandatory)
- **Always follow instructions exactly** - only do what is explicitly requested
- **No additional work** - don't perform tasks not mentioned in the request
- **Prefer minimalism** - less is better than more
- **No code building** - don't compile or run code
- **No syntax fixes** - don't correct syntax errors unless explicitly asked
- **No proactive improvements** - only implement what's requested, nothing more