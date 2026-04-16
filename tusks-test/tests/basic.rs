use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("basic").unwrap()
}

#[test]
fn test_hello() {
    cli()
        .args(&["hello", "Alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Alice!"));
}

#[test]
fn test_hello_different_name() {
    cli()
        .args(&["hello", "Bob"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Bob!"));
}

#[test]
fn test_goodbye_informal() {
    cli()
        .args(&["goodbye", "Alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Bye, Alice!"));
}

#[test]
fn test_goodbye_formal() {
    cli()
        .args(&["goodbye", "Alice", "--formal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Farewell, Alice!"));
}

#[test]
fn test_echo_once() {
    cli()
        .args(&["echo", "Hello World"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World\n").count(1));
}

#[test]
fn test_echo_count_short() {
    cli()
        .args(&["echo", "Test", "-c", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test\n").count(3));
}

#[test]
fn test_echo_count_long() {
    cli()
        .args(&["echo", "Message", "--count", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Message\n").count(2));
}

#[test]
fn test_help() {
    cli()
        .args(&["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Basic CLI test"))
        .stdout(predicate::str::contains("hello"))
        .stdout(predicate::str::contains("goodbye"))
        .stdout(predicate::str::contains("echo"));
}

#[test]
fn test_version() {
    cli()
        .args(&["--version"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1.0.0"));
}

#[test]
fn test_hello_help() {
    cli()
        .args(&["hello", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("basic hello"))
        .stdout(predicate::str::contains("Name to greet"));
}

#[test]
fn test_goodbye_help() {
    cli()
        .args(&["goodbye", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("say goodbye"))
        .stdout(predicate::str::contains("--formal"));
}

#[test]
fn test_missing_argument() {
    cli()
        .args(&["hello"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"));
}

#[test]
fn test_unknown_command() {
    cli()
        .args(&["unknown"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}
