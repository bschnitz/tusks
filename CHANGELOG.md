# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [3.2.1] - 2026-04-19

### Fixed
- **Async + external modules**: Fixed `E0726` implicit elided lifetime error
  that occurred when using the `async` feature together with external modules.
  The `handle_matches` function signature for external modules now uses
  `Parameters<'_>` instead of eliding the lifetime.

## [3.2.0] - 2026-04-17

### Added
- **ValueEnum support**: Enums with `#[derive(Clone, clap::ValueEnum)]` inside
  tusks modules now work as argument types with automatic validation and
  possible-values in help output.
- **Global arg tests**: Verified and tested that `#[arg(global = true)]` passes
  through correctly to clap.
- **CHANGELOG.md**: Project now tracks changes in a changelog.

### Changed
- Resolve all clippy warnings; rename `to_list()` to `into_list()` following
  Rust naming conventions for consuming methods.
- Expand "Relationship with Clap" README section with comprehensive attribute
  passthrough documentation, ValueEnum example, global arg example, and
  explicit list of unsupported features.

## [3.1.0] - 2026-04-17

### Added
- **Doc comments as help text**: `///` comments on functions and modules now
  appear as `about` text in `--help` output. No need for explicit
  `#[command(about = "...")]` for simple descriptions.
- **Result return types**: Command functions can now return `Result<T, E>`
  where `T` is `()`, `u8`, or `Option<u8>`. On `Err`, the error is printed
  to stderr and exit code 1 is returned.
- **Shell completions**: New `completions` feature flag adds a hidden
  `--completions <SHELL>` argument that generates completion scripts for
  bash, zsh, fish, elvish, and powershell (via `clap_complete`).
- **Async command functions**: New `async` feature flag allows `pub async fn`
  commands. Generates tokio runtime in `exec_cli()` and `.await` in dispatch.
  Mixed sync/async in the same module works. All return types supported.

### Fixed
- `has_default_match_arm` was never set to `true`, causing unreachable error
  arms when a `#[default]` function exists.
- Debug `eprintln!` in `List::print()` that printed internal state to stderr
  on every task list display.

### Changed
- Resolve all clippy warnings (collapsed if-let chains, unnecessary borrows,
  `to_list()` renamed to `into_list()` to follow Rust naming conventions).

## [3.0.0] - 2026-04-17

### Changed
- **BREAKING**: Function parameters without an explicit `#[arg(...)]` attribute
  are now **positional** arguments (matching clap's default behavior) instead of
  `--long` named arguments. To keep the old behavior, add `#[arg(long)]`
  explicitly.

### Added
- 142 new tests: unit tests for `tusks-tasks` and `tusks-lib` parsing,
  `trybuild` compile-fail tests, integration edge-case tests.

### Refactored
- Introduce `ModulePath` type to replace raw `&[&str]` path tracking in codegen.
- Split `TusksModule` methods into trait-based codegen phases (`CliCodegen`,
  `HandleMatchesCodegen`, `ParametersCodegen`).
- Centralize special field filtering via `field_util::is_generated_field()`.
- Consolidate three identity `enum_util` functions into single `to_variant_ident()`.
- Deduplicate error arm generation via shared `build_no_command_error()`.
- Remove empty placeholder files and translate German comments to English.

### Fixed
- Typo: `add_use_staements` → `add_use_statements`.
- 12 broken doctests (changed from ` ```rust ` to ` ```ignore ` for illustrative
  code blocks).

## [2.1.7] - 2025-03-01

### Fixed
- Relax `unicode-width` requirement to `0.2.0`.
