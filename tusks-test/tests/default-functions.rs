use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("default-functions").unwrap()
}

#[test]
fn test_root_command() {
    cli()
        .args(&["root"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Root command executed"));
}

// Git module with default
#[test]
fn test_git_default_no_args() {
    cli()
        .args(&["git"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Git status"));
}

#[test]
fn test_git_default_with_repo() {
    cli()
        .args(&["git", "--repo", "/home/user/project"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Git status"))
        .stdout(predicate::str::contains("Repository: /home/user/project"));
}

#[test]
fn test_git_default_explicit() {
    cli()
        .args(&["git", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Git status"));
}

#[test]
fn test_git_commit() {
    cli()
        .args(&["git", "commit", "Initial commit"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Committing: Initial commit"));
}

#[test]
fn test_git_commit_with_repo() {
    cli()
        .args(&["git", "--repo", "/project", "commit", "Fix bug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Repository: /project"));
}

#[test]
fn test_git_push() {
    cli()
        .args(&["git", "push"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Pushing changes"));
}

// Build module with default, no parameters
#[test]
fn test_build_default() {
    cli()
        .args(&["build"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Building project with default settings"));
}

#[test]
fn test_build_default_explicit() {
    cli()
        .args(&["build", "default"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Building project with default settings"));
}

#[test]
fn test_build_clean() {
    cli()
        .args(&["build", "clean"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleaning build artifacts"));
}

#[test]
fn test_build_test() {
    cli()
        .args(&["build", "test"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running tests"));
}

// Run module with external subcommands
#[test]
fn test_run_default_no_command() {
    cli()
        .args(&["run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No command specified"));
}

#[test]
fn test_run_external_command() {
    cli()
        .args(&["run", "make"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running external command: make"));
}

#[test]
fn test_run_external_command_with_args() {
    cli()
        .args(&["run", "make", "clean", "all"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running external command: make"))
        .stdout(predicate::str::contains("Arguments: [\"clean\", \"all\"]"));
}

#[test]
fn test_run_external_with_workdir() {
    cli()
        .args(&["run", "--workdir", "/build", "cmake"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Working directory: /build"));
}

#[test]
fn test_run_builtin() {
    cli()
        .args(&["run", "builtin", "mytask"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running built-in task: mytask"));
}

// Server module with nested defaults
#[test]
fn test_server_default() {
    cli()
        .args(&["server"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Server status on port: 8080"));
}

#[test]
fn test_server_default_custom_port() {
    cli()
        .args(&["server", "--port", "3000"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Server status on port: 3000"));
}

#[test]
fn test_server_status_explicit() {
    cli()
        .args(&["server", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Server status on port: 8080"));
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
        .args(&["server", "--port", "9000", "start"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Starting server on port: 9000"));
}

#[test]
fn test_server_config_default() {
    cli()
        .args(&["server", "config"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Showing server configuration"));
}

#[test]
fn test_server_config_show_explicit() {
    cli()
        .args(&["server", "config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Showing server configuration"));
}

#[test]
fn test_server_config_update() {
    cli()
        .args(&["server", "config", "update", "timeout", "30"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Updating config: timeout=30"));
}

// Database module without default
#[test]
fn test_database_no_default() {
    cli()
        .args(&["database", "--connection", "postgres://localhost"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required").or(predicate::str::contains("subcommand")));
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
fn test_database_query() {
    cli()
        .args(&["database", "--connection", "postgres://localhost", "query", "SELECT * FROM users"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Executing: SELECT * FROM users"))
        .stdout(predicate::str::contains("Connection: postgres://localhost"));
}

// Health module with default returning exit code
#[test]
fn test_health_default_success() {
    cli()
        .args(&["health"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Health check passed"));
}

#[test]
fn test_health_check_explicit() {
    cli()
        .args(&["health", "check"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Health check passed"));
}

#[test]
fn test_health_component() {
    cli()
        .args(&["health", "component", "database"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Checking component: database"));
}

// Help tests
#[test]
fn test_git_help() {
    cli()
        .args(&["git", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Git operations"))
        .stdout(predicate::str::contains("status"))
        .stdout(predicate::str::contains("commit"))
        .stdout(predicate::str::contains("push"));
}

#[test]
fn test_run_help() {
    cli()
        .args(&["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Task runner"))
        .stdout(predicate::str::contains("execute"))
        .stdout(predicate::str::contains("builtin"));
}
