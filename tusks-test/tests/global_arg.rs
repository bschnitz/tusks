use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("global-arg").unwrap()
}

#[test]
fn global_arg_before_subcommand() {
    cli()
        .args(&["--verbose", "sub", "action"])
        .assert()
        .success()
        .stdout(predicate::str::contains("sub verbose=true"));
}

#[test]
fn global_arg_after_subcommand() {
    cli()
        .args(&["sub", "action", "--verbose"])
        .assert()
        .success()
        .stdout(predicate::str::contains("sub verbose=true"));
}

#[test]
fn global_arg_at_root() {
    cli()
        .args(&["--verbose", "root-cmd"])
        .assert()
        .success()
        .stdout(predicate::str::contains("root verbose=true"));
}

#[test]
fn global_arg_default_false() {
    cli()
        .args(&["sub", "action"])
        .assert()
        .success()
        .stdout(predicate::str::contains("sub verbose=false"));
}
