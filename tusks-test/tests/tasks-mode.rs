use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("tasks-mode").unwrap()
}

// Test task list output (no arguments)
#[test]
fn test_task_list() {
    cli()
        .assert()
        .success()
        .stdout(predicate::str::contains("Task management system"))
        .stdout(predicate::str::contains("git"))
        .stdout(predicate::str::contains("docker"))
        .stdout(predicate::str::contains("database"))
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("deploy"));
}

// Root level task
#[test]
fn test_hello_task() {
    cli()
        .args(&["hello", "World"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

// Flat syntax with dot separator
#[test]
fn test_git_clone_flat() {
    cli()
        .args(&["git.clone", "https://github.com/user/repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cloning: https://github.com/user/repo"));
}

#[test]
fn test_git_commit_flat() {
    cli()
        .args(&["git.commit", "Initial commit"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Committing: Initial commit"));
}

#[test]
fn test_git_push_flat() {
    cli()
        .args(&["git.push"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Pushing changes"));
}

// Traditional subcommand syntax still works
#[test]
fn test_git_clone_traditional() {
    cli()
        .args(&["git", "clone", "https://github.com/user/repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cloning: https://github.com/user/repo"));
}

#[test]
fn test_git_commit_traditional() {
    cli()
        .args(&["git", "commit", "Fix bug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Committing: Fix bug"));
}

// Nested tasks with flat syntax
#[test]
fn test_git_advanced_rebase_flat() {
    cli()
        .args(&["git.advanced.rebase", "main"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rebasing onto: main"));
}

#[test]
fn test_git_advanced_cherry_pick_flat() {
    cli()
        .args(&["git.advanced.cherry-pick", "abc123"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cherry-picking: abc123"));
}

// Nested tasks with traditional syntax
#[test]
fn test_git_advanced_rebase_traditional() {
    cli()
        .args(&["git", "advanced", "rebase", "develop"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rebasing onto: develop"));
}

// Docker tasks
#[test]
fn test_docker_build_flat() {
    cli()
        .args(&["docker.build", "."])
        .assert()
        .success()
        .stdout(predicate::str::contains("Building from: ."));
}

#[test]
fn test_docker_run_flat() {
    cli()
        .args(&["docker.run", "nginx"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running: nginx"));
}

#[test]
fn test_docker_container_list_flat() {
    cli()
        .args(&["docker.container.list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Listing containers"));
}

#[test]
fn test_docker_container_inspect_flat() {
    cli()
        .args(&["docker.container.inspect", "abc123"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Inspecting: abc123"));
}

#[test]
fn test_docker_image_list_flat() {
    cli()
        .args(&["docker.image.list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Listing images"));
}

#[test]
fn test_docker_image_remove_flat() {
    cli()
        .args(&["docker.image.remove", "image123"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removing: image123"));
}

#[test]
fn test_docker_image_pull_flat() {
    cli()
        .args(&["docker.image.pull", "ubuntu:latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Pulling: ubuntu:latest"));
}

// Database tasks
#[test]
fn test_database_create_flat() {
    cli()
        .args(&["database.create", "mydb"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Creating database: mydb"));
}

#[test]
fn test_database_drop_flat() {
    cli()
        .args(&["database.drop", "olddb"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Dropping database: olddb"));
}

#[test]
fn test_database_migrate_up_flat() {
    cli()
        .args(&["database.migrate.up"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running migrations up"));
}

#[test]
fn test_database_migrate_down_flat() {
    cli()
        .args(&["database.migrate.down"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rolling back migrations"));
}

#[test]
fn test_database_migrate_status_flat() {
    cli()
        .args(&["database.migrate.status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Migration status"));
}

// Test tasks
#[test]
fn test_test_unit_flat() {
    cli()
        .args(&["test.unit"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running unit tests"));
}

#[test]
fn test_test_integration_flat() {
    cli()
        .args(&["test.integration"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running integration tests"));
}

#[test]
fn test_test_e2e_flat() {
    cli()
        .args(&["test.e2e"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running e2e tests"));
}

// Deploy tasks
#[test]
fn test_deploy_staging_flat() {
    cli()
        .args(&["deploy.staging", "v1.0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deploying v1.0.0 to staging"));
}

#[test]
fn test_deploy_production_flat() {
    cli()
        .args(&["deploy.production", "v2.0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deploying v2.0.0 to production"));
}

// Help with 'h' prefix
#[test]
fn test_help_with_h_prefix() {
    cli()
        .args(&["h", "git.clone"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Clone a repository"));
}

#[test]
fn test_help_with_flag() {
    cli()
        .args(&["git.clone", "-h"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Clone a repository"));
}

#[test]
fn test_help_traditional() {
    cli()
        .args(&["git", "clone", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Clone a repository"));
}

// Invalid task
#[test]
fn test_invalid_task() {
    cli()
        .args(&["invalid.task"])
        .assert()
        .failure();
}

// Version
#[test]
fn test_version() {
    cli()
        .args(&["--version"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1.0.0"));
}
