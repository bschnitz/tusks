use assert_cmd::prelude::*;
use predicates::prelude::*;

fn cli() -> std::process::Command {
    std::process::Command::cargo_bin("async-external").unwrap()
}

#[test]
fn async_external_module() {
    cli()
        .args(&["deploy", "start", "v1.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deploying v1.0"));
}
