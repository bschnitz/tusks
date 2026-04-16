# AGENTS.md - Codebase Guidelines for tusks-test

## Build/Lint/Test Commands

- **Build**: `cargo build` - Build the project
- **Run**: `cargo run` - Run the binary
- **Test all**: `cargo test` - Run all tests
- **Test single**: `cargo test --test integration_tests` - Run integration tests
- **Format**: `cargo fmt` - Format code according to Rust style
- **Lint**: `cargo clippy` - Run Clippy linter
- **Check**: `cargo check` - Type check without building

## Code Style Guidelines

### General
- Rust 2024 edition with modern idiomatic patterns
- Follow Rust standard library conventions
- Use `cargo fmt` for consistent formatting
- Run `cargo clippy` before commits

### Imports & Structure
- Group imports by module (standard, external, local)
- Use `use` statements at module level, not inside functions
- Organize modules in logical groups (root, external modules)
- Use `#[tusks]` attribute for task modules as shown in main.rs

### Types & Naming
- Use Rust primitive types where appropriate
- Follow snake_case for function and variable names
- Use PascalCase for struct and enum names
- Use descriptive names (e.g., `root_param` instead of `rp`)

### Error Handling
- Use `Result<T, E>` for functions that can fail
- Provide meaningful error messages
- Handle errors at appropriate levels
- Use `?` operator for propagating errors

### Documentation
- Add `///` doc comments for public functions
- Include examples and usage information
- Document complex logic and algorithms
- Follow Rustdoc conventions

### Testing
- Add integration tests in `tests/integration_tests.rs`
- Test both success and error cases
- Use `assert_cmd` and `predicates` for testing
- Ensure tests cover all command-line arguments

### Dependencies
- Use `tusks` crate for task management
- Use `clap` for argument parsing
- Keep dependencies minimal and up-to-date
- Follow semver for versioning

### Feature Flags
- Use `debug` feature for development
- Enable with `cargo build --features debug`