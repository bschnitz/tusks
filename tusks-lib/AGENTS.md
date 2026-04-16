# AGENTS.md - Guidelines for tusks-lib Repository

## Build & Test Commands
- Build: `cargo build`
- Test all: `cargo test`
- Run single test: `cargo test --test <test_name>` or `cargo test -- <test_name>`
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Code Style Guidelines

### General
- Rust 2024 edition with strict formatting
- Follow Rust standard library conventions
- Use `clap` for CLI arguments
- Use `syn` and `quote` for code generation

### Imports & Formatting
- Group imports by crate (std, external, local)
- Use absolute paths for intra-crate imports
- Keep lines under 100 characters
- Use 4 spaces for indentation

### Types & Naming
- Use `snake_case` for functions and variables
- Use `PascalCase` for types and traits
- Use `SCREAMING_SNAKE_CASE` for constants
- Prefer `Result<T, E>` over `Option<T>` for fallible operations

### Error Handling
- Use `?` operator for propagating errors
- Define custom error types for library operations
- Provide clear error messages
- Implement `std::fmt::Display` for errors

### Code Generation
- Use `proc-macro2` and `quote` for safe code generation
- Validate input thoroughly before generating code
- Follow Rust's safety guarantees in generated code
- Document code generation behavior clearly

### Documentation
- Use Rust doc comments (`///`) for public APIs
- Include examples in documentation
- Document complex algorithms and edge cases
- Keep documentation up-to-date with code changes