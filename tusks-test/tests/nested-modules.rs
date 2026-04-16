use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("nested-modules").unwrap()
}

#[test]
fn test_root_command() {
    cli()
        .args(&["root-command", "test"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Root: test"));
}

#[test]
fn test_level1_command() {
    cli()
        .args(&["level1", "command", "value1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Level1 command: value1"));
}

#[test]
fn test_level1_another() {
    cli()
        .args(&["level1", "another", "--feature"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Level1 another: feature=true"));
}

#[test]
fn test_level2_command() {
    cli()
        .args(&["level1", "level2", "command", "input1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Level2 command: input1"));
}

#[test]
fn test_level3_command() {
    cli()
        .args(&["level1", "level2", "level3", "command", "data1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Level3 command: data1"));
}

#[test]
fn test_level3_deep() {
    cli()
        .args(&["level1", "level2", "level3", "deep", "--count", "5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Level3 deep: count=Some(5)"));
}

#[test]
fn test_level3_deep_no_count() {
    cli()
        .args(&["level1", "level2", "level3", "deep"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Level3 deep: count=None"));
}

#[test]
fn test_database_connect() {
    cli()
        .args(&["database", "connect", "--connection", "postgres://localhost"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Connecting to: postgres://localhost"));
}

#[test]
fn test_database_query_execute() {
    cli()
        .args(&["database", "query", "execute", "SELECT * FROM users"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Executing query: SELECT * FROM users"));
}

#[test]
fn test_database_query_list() {
    cli()
        .args(&["database", "query", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Listing all queries"));
}

#[test]
fn test_database_migrate_up() {
    cli()
        .args(&["database", "migrate", "up", "--version", "v2.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running migrations up to: Some(\"v2.0\")"));
}

#[test]
fn test_database_migrate_up_no_version() {
    cli()
        .args(&["database", "migrate", "up"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running migrations up to: None"));
}

#[test]
fn test_database_migrate_down() {
    cli()
        .args(&["database", "migrate", "down", "--steps", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rolling back 3 steps"));
}

#[test]
fn test_database_migrate_down_default() {
    cli()
        .args(&["database", "migrate", "down"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rolling back 1 steps"));
}

#[test]
fn test_server_start() {
    cli()
        .args(&["server", "start", "--port", "3000"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Starting server on port: 3000"));
}

#[test]
fn test_server_start_default_port() {
    cli()
        .args(&["server", "start"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Starting server on port: 8080"));
}

#[test]
fn test_server_stop() {
    cli()
        .args(&["server", "stop"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Stopping server"));
}

#[test]
fn test_server_config_show() {
    cli()
        .args(&["server", "config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Showing configuration"));
}

#[test]
fn test_server_config_update() {
    cli()
        .args(&["server", "config", "update", "timeout", "30"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Updating config: timeout=30"));
}

#[test]
fn test_help_shows_modules() {
    cli()
        .args(&["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("level1"))
        .stdout(predicate::str::contains("database"))
        .stdout(predicate::str::contains("server"));
}

#[test]
fn test_level1_help() {
    cli()
        .args(&["level1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("First level operations"))
        .stdout(predicate::str::contains("command"))
        .stdout(predicate::str::contains("another"))
        .stdout(predicate::str::contains("level2"));
}

#[test]
fn test_database_help() {
    cli()
        .args(&["database", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Database management"))
        .stdout(predicate::str::contains("connect"))
        .stdout(predicate::str::contains("query"))
        .stdout(predicate::str::contains("migrate"));
}

#[test]
fn test_invalid_nesting() {
    cli()
        .args(&["level1", "level3", "command", "data"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}
