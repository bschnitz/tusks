use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("async-basic").unwrap()
}

// --- Async commands ---

#[test]
fn async_fetch() {
    cli()
        .args(&["fetch", "https://example.com"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Fetching: https://example.com"));
}

#[test]
fn async_check_returns_u8() {
    cli()
        .args(&["check"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Running async check..."));
}

#[test]
fn async_validate_returns_option() {
    cli()
        .args(&["validate", "config.toml"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Validating: config.toml"));
}

// --- Sync commands still work ---

#[test]
fn sync_greet() {
    cli()
        .args(&["greet", "World"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

// --- Async in nested modules ---

#[test]
fn async_nested_migrate() {
    cli()
        .args(&["db", "migrate", "v2.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Migrating to: v2.0"));
}

#[test]
fn sync_nested_status() {
    cli()
        .args(&["db", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DB status: ok"));
}

// --- Help and version ---

#[test]
fn help_output() {
    cli()
        .args(&["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Async CLI test"))
        .stdout(predicate::str::contains("fetch"))
        .stdout(predicate::str::contains("greet"))
        .stdout(predicate::str::contains("check"))
        .stdout(predicate::str::contains("db"));
}

#[test]
fn version_output() {
    cli()
        .args(&["--version"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1.0.0"));
}
