use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("value-enum").unwrap()
}

#[test]
fn enum_explicit_value() {
    cli()
        .args(&["paint", "--color", "always", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("color=always message=hello"));
}

#[test]
fn enum_default_value() {
    cli()
        .args(&["paint", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("color=auto message=hello"));
}

#[test]
fn enum_invalid_value_fails() {
    cli()
        .args(&["paint", "--color", "invalid", "hello"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"))
        .stderr(predicate::str::contains("possible values"));
}

#[test]
fn second_enum_type() {
    cli()
        .args(&["output", "--format", "json", "test"])
        .assert()
        .success()
        .stdout(predicate::str::contains("format=json data=test"));
}

#[test]
fn enum_values_shown_in_help() {
    cli()
        .args(&["paint", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("auto"))
        .stdout(predicate::str::contains("always"))
        .stdout(predicate::str::contains("never"));
}
