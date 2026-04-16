use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("parameters").unwrap()
}

#[test]
fn test_root_status_simple() {
    cli()
        .args(&["status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status check"));
}

#[test]
fn test_root_status_verbose() {
    cli()
        .args(&["--verbose", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status check"))
        .stdout(predicate::str::contains("Verbose: enabled"));
}

#[test]
fn test_root_status_with_config() {
    cli()
        .args(&["--config", "prod.toml", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Config: prod.toml"));
}

#[test]
fn test_root_status_all_params() {
    cli()
        .args(&["--verbose", "--config", "test.toml", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: enabled"))
        .stdout(predicate::str::contains("Config: test.toml"));
}

#[test]
fn test_database_connect() {
    cli()
        .args(&["database", "--connection", "postgres://localhost", "connect"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Connecting to: postgres://localhost"));
}

#[test]
fn test_database_connect_with_timeout() {
    cli()
        .args(&["database", "--connection", "postgres://localhost", "--timeout", "30", "connect"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Timeout: 30s"));
}

#[test]
fn test_database_connect_with_root_verbose() {
    cli()
        .args(&["--verbose", "database", "--connection", "postgres://localhost", "connect"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose mode enabled"));
}

#[test]
fn test_database_connect_all_params() {
    cli()
        .args(&[
            "--verbose",
            "--config", "db.toml",
            "database",
            "--connection", "postgres://localhost",
            "--timeout", "60",
            "connect"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Connecting to: postgres://localhost"))
        .stdout(predicate::str::contains("Timeout: 60s"))
        .stdout(predicate::str::contains("Verbose mode enabled"))
        .stdout(predicate::str::contains("Using config: db.toml"));
}

#[test]
fn test_database_query() {
    cli()
        .args(&[
            "database",
            "--connection", "postgres://localhost",
            "query",
            "SELECT * FROM users"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Executing: SELECT * FROM users"))
        .stdout(predicate::str::contains("Connection: postgres://localhost"));
}

#[test]
fn test_database_query_verbose() {
    cli()
        .args(&[
            "--verbose",
            "database",
            "--connection", "postgres://localhost",
            "query",
            "SELECT * FROM users"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Query details: length="));
}

#[test]
fn test_backup_create() {
    cli()
        .args(&[
            "database",
            "--connection", "postgres://localhost",
            "backup",
            "--path", "/backups/db",
            "create"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Creating backup to: /backups/db"))
        .stdout(predicate::str::contains("Database: postgres://localhost"));
}

#[test]
fn test_backup_create_with_compression() {
    cli()
        .args(&[
            "database",
            "--connection", "postgres://localhost",
            "backup",
            "--path", "/backups/db",
            "--compression", "9",
            "create"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Compression: 9"));
}

#[test]
fn test_backup_create_all_levels() {
    cli()
        .args(&[
            "--verbose",
            "--config", "backup.toml",
            "database",
            "--connection", "postgres://localhost",
            "--timeout", "120",
            "backup",
            "--path", "/backups/db",
            "--compression", "5",
            "create"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Creating backup to: /backups/db"))
        .stdout(predicate::str::contains("Compression: 5"))
        .stdout(predicate::str::contains("Database: postgres://localhost"))
        .stdout(predicate::str::contains("Verbose: showing detailed backup info"))
        .stdout(predicate::str::contains("Root config: backup.toml"));
}

#[test]
fn test_backup_restore() {
    cli()
        .args(&[
            "database",
            "--connection", "postgres://localhost",
            "backup",
            "--path", "/backups/db",
            "restore",
            "backup_2024.sql"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Restoring from: backup_2024.sql"))
        .stdout(predicate::str::contains("Target path: /backups/db"))
        .stdout(predicate::str::contains("Database: postgres://localhost"));
}

#[test]
fn test_backup_restore_verbose() {
    cli()
        .args(&[
            "--verbose",
            "database",
            "--connection", "postgres://localhost",
            "backup",
            "--path", "/backups/db",
            "restore",
            "backup_2024.sql"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose restore mode"));
}

#[test]
fn test_server_start() {
    cli()
        .args(&["server", "start"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Starting server on port: 8080"));
}

#[test]
fn test_server_start_custom_port() {
    cli()
        .args(&["server", "start", "--port", "3000"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Starting server on port: 3000"));
}

#[test]
fn test_server_start_with_root_params() {
    cli()
        .args(&["--verbose", "--config", "server.toml", "server", "start"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: detailed startup"))
        .stdout(predicate::str::contains("Config: server.toml"));
}

#[test]
fn test_server_deploy_run() {
    cli()
        .args(&[
            "server",
            "deploy",
            "--environment", "production",
            "run",
            "v1.2.3"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deploying version: v1.2.3"))
        .stdout(predicate::str::contains("Environment: production"));
}

#[test]
fn test_server_deploy_run_verbose() {
    cli()
        .args(&[
            "--verbose",
            "server",
            "deploy",
            "--environment", "staging",
            "run",
            "v2.0.0"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose deployment"));
}

#[test]
fn test_test_run_with_suite() {
    cli()
        .args(&["test", "--suite", "integration", "run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running test suite: integration"));
}

#[test]
fn test_test_run_without_suite() {
    cli()
        .args(&["test", "run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running default tests"));
}

#[test]
fn test_test_run_verbose() {
    cli()
        .args(&["--verbose", "test", "run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose test output"));
}

#[test]
fn test_test_list() {
    cli()
        .args(&["test", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Listing all available tests"));
}

#[test]
fn test_missing_required_param() {
    cli()
        .args(&["database", "connect"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"))
        .stderr(predicate::str::contains("--connection"));
}

#[test]
fn test_params_order_matters() {
    // Root params must come before subcommand
    cli()
        .args(&["database", "--verbose", "--connection", "postgres://localhost", "connect"])
        .assert()
        .failure();
    
    // Correct order
    cli()
        .args(&["--verbose", "database", "--connection", "postgres://localhost", "connect"])
        .assert()
        .success();
}
