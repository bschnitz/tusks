use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("result-return").unwrap()
}

// --- Result<(), E> ---

#[test]
fn result_ok_unit_succeeds() {
    cli()
        .args(&["succeed"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Success!"));
}

#[test]
fn result_err_prints_error_and_exits_1() {
    cli()
        .args(&["fail"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("something went wrong"));
}

// --- Result<u8, E> ---

#[test]
fn result_ok_u8_returns_exit_code() {
    cli()
        .args(&["check"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Check passed"));
}

#[test]
fn result_ok_u8_custom_exit_code() {
    cli()
        .args(&["check-fail"])
        .assert()
        .code(42);
}

// --- Result<Option<u8>, E> ---

#[test]
fn result_option_ok() {
    cli()
        .args(&["maybe"])
        .assert()
        .success()
        .stdout(predicate::str::contains("All good"));
}

#[test]
fn result_option_err() {
    cli()
        .args(&["maybe", "--fail"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("not ok"));
}

// --- Doc comments as help text ---

#[test]
fn doc_comment_appears_in_help() {
    cli()
        .args(&["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Doc comment used as help text"));
}

#[test]
fn doc_comments_on_all_commands_in_help() {
    cli()
        .args(&["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Succeeds with Ok(())"))
        .stdout(predicate::str::contains("Fails with an error message"))
        .stdout(predicate::str::contains("Returns Ok with exit code"));
}
