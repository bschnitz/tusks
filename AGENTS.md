# AGENTS.md - Guidelines for tusks Repository

## Build & Test Commands
- **Build all**: `cargo build --workspace`
- **Test all**: `cargo test --workspace`
- **Run single test**: `cargo test --package <crate> --test <test_name>`
- **Lint**: `cargo clippy --workspace`
- **Format**: `cargo fmt --all`
- **Check**: `cargo check --workspace`

## Scanning Restrictions
- Only scan these directories: `tusks`, `tusks-lib`, `tusks-macro`, `tusks-test`
- Only scan markdown files in root directory: `*.md`
- Do not scan other files or directories in root

## Code Style Guidelines

### General
- Rust 2024 edition across all crates
- Follow Rust standard library conventions
- Use `cargo fmt` and `cargo clippy` before commits
- Keep lines under 100 characters, 4 spaces indentation

### Imports & Structure
- Group imports by crate (std, external, local)
- Use absolute paths for intra-crate imports
- Organize modules logically (root, external, tests)
- Use `use` statements at module level

### Types & Naming
- `snake_case` for functions and variables
- `PascalCase` for types, traits, and structs
- `SCREAMING_SNAKE_CASE` for constants
- Descriptive names over abbreviations

### Error Handling
- Use `Result<T, E>` for fallible operations
- Use `?` operator for error propagation
- Define custom error types where needed
- Implement `Display` for clear error messages

### Documentation
- Use Rust doc comments (`///`) for public APIs
- Include examples and usage information
- Document complex algorithms and edge cases
- Keep documentation up-to-date with code changes

### Testing
- Add integration tests in `tests/` directory
- Test both success and error cases
- Use `assert_cmd` and `predicates` for CLI testing
- Ensure test coverage for all command-line arguments

### Dependencies
- Keep dependencies minimal and up-to-date
- Follow semver for versioning
- Use workspace dependencies where appropriate
- Update Cargo.lock regularly