use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("return-values").unwrap()
}

#[test]
fn test_success_no_return() {
    cli()
        .args(&["success"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Operation completed successfully"));
}

#[test]
fn test_check_health_success() {
    cli()
        .args(&["check-health"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Health check passed"));
}

#[test]
fn test_check_health_failure() {
    cli()
        .args(&["check-health", "--fail"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("✗ Health check failed"));
}

#[test]
fn test_validate_valid_file() {
    cli()
        .args(&["validate", "file.txt"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ File is valid"));
}

#[test]
fn test_validate_invalid_file() {
    cli()
        .args(&["validate", "file.invalid"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("✗ Invalid file format"));
}

#[test]
fn test_validate_warning_non_strict() {
    cli()
        .args(&["validate", "file.warning"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("⚠ File has warnings (non-strict)"));
}

#[test]
fn test_validate_warning_strict() {
    cli()
        .args(&["validate", "file.warning", "--strict"])
        .assert()
        .code(2)
        .stdout(predicate::str::contains("⚠ File has warnings (strict mode)"));
}

#[test]
fn test_test_pass() {
    cli()
        .args(&["test", "pass"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Test passed"));
}

#[test]
fn test_test_run_success() {
    cli()
        .args(&["test", "run"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Test passed"));
}

#[test]
fn test_test_run_failure() {
    cli()
        .args(&["test", "run", "--fail"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("✗ Test failed"));
}

#[test]
fn test_test_optional_success() {
    cli()
        .args(&["test", "optional", "success"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Test passed"));
}

#[test]
fn test_test_optional_fail() {
    cli()
        .args(&["test", "optional", "fail"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("✗ Test failed"));
}

#[test]
fn test_test_optional_skip() {
    cli()
        .args(&["test", "optional", "skip"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("○ Test skipped"));
}

#[test]
fn test_test_optional_warn() {
    cli()
        .args(&["test", "optional", "warn"])
        .assert()
        .code(2)
        .stdout(predicate::str::contains("⚠ Test passed with warnings"));
}

#[test]
fn test_process_start_success() {
    cli()
        .args(&["process", "start", "myapp"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Process started successfully"));
}

#[test]
fn test_process_start_failure() {
    cli()
        .args(&["process", "start", "myapp", "--fail"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("✗ Failed to start process"));
}

#[test]
fn test_process_stop_success() {
    cli()
        .args(&["process", "stop", "myapp"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Process stopped"));
}

#[test]
fn test_process_stop_critical_no_force() {
    cli()
        .args(&["process", "stop", "critical"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("✗ Cannot stop critical process without --force"));
}

#[test]
fn test_process_stop_critical_force() {
    cli()
        .args(&["process", "stop", "critical", "--force"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Process stopped"));
}

#[test]
fn test_process_stop_unknown() {
    cli()
        .args(&["process", "stop", "unknown"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("○ Process status unknown, assuming stopped"));
}

#[test]
fn test_process_stop_nonexistent() {
    cli()
        .args(&["process", "stop", "nonexistent"])
        .assert()
        .code(2)
        .stdout(predicate::str::contains("⚠ Process not found"));
}

#[test]
fn test_process_status() {
    cli()
        .args(&["process", "status", "myapp"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Status of myapp: running"));
}

#[test]
fn test_deploy_start_success() {
    cli()
        .args(&["deploy", "start", "1.0.0"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Deployment successful"));
}

#[test]
fn test_deploy_start_beta_blocked() {
    cli()
        .args(&["deploy", "start", "0.1.0"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("✗ Cannot deploy beta version"));
}

#[test]
fn test_deploy_start_beta_skip_validation() {
    cli()
        .args(&["deploy", "start", "0.1.0", "--skip-validation"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Deployment successful"));
}

#[test]
fn test_deploy_start_unknown() {
    cli()
        .args(&["deploy", "start", "unknown"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("○ Version validation skipped"));
}

#[test]
fn test_deploy_start_broken() {
    cli()
        .args(&["deploy", "start", "broken"])
        .assert()
        .code(2)
        .stdout(predicate::str::contains("⚠ Deploying known broken version"));
}

#[test]
fn test_deploy_rollback_success() {
    cli()
        .args(&["deploy", "rollback"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Rolled back 1 step(s)"));
}

#[test]
fn test_deploy_rollback_multiple() {
    cli()
        .args(&["deploy", "rollback", "--steps", "5"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("✓ Rolled back 5 step(s)"));
}

#[test]
fn test_deploy_rollback_too_many() {
    cli()
        .args(&["deploy", "rollback", "--steps", "15"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("✗ Cannot rollback more than 10 steps"));
}

#[test]
fn test_custom_exit_code_0() {
    cli()
        .args(&["custom", "exit", "0"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Exiting with code: 0"));
}

#[test]
fn test_custom_exit_code_1() {
    cli()
        .args(&["custom", "exit", "1"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("Exiting with code: 1"));
}

#[test]
fn test_custom_exit_code_42() {
    cli()
        .args(&["custom", "exit", "42"])
        .assert()
        .code(42)
        .stdout(predicate::str::contains("Exiting with code: 42"));
}

#[test]
fn test_custom_multi_success() {
    cli()
        .args(&["custom", "multi", "success"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Success scenario"));
}

#[test]
fn test_custom_multi_warning() {
    cli()
        .args(&["custom", "multi", "warning"])
        .assert()
        .code(2)
        .stdout(predicate::str::contains("Warning scenario"));
}

#[test]
fn test_custom_multi_error() {
    cli()
        .args(&["custom", "multi", "error"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("Error scenario"));
}

#[test]
fn test_custom_multi_critical() {
    cli()
        .args(&["custom", "multi", "critical"])
        .assert()
        .code(3)
        .stdout(predicate::str::contains("Critical error"));
}

#[test]
fn test_custom_multi_none() {
    cli()
        .args(&["custom", "multi", "none"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("No exit code"));
}

#[test]
fn test_custom_multi_unknown() {
    cli()
        .args(&["custom", "multi", "unknown"])
        .assert()
        .code(255)
        .stdout(predicate::str::contains("Unknown scenario"));
}
