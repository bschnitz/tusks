use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("tasks").unwrap()
}

// --- Task list output formatting ---

#[test]
fn task_list_shows_description() {
    cli()
        .assert()
        .success()
        .stdout(predicate::str::contains("A binary for managing tasks with git and docker submodules"));
}

#[test]
fn task_list_shows_all_tasks() {
    cli()
        .assert()
        .success()
        .stdout(predicate::str::contains("docker.build"))
        .stdout(predicate::str::contains("docker.run"))
        .stdout(predicate::str::contains("git.clone"))
        .stdout(predicate::str::contains("git.commit"));
}

#[test]
fn task_list_uses_dot_separator() {
    cli()
        .assert()
        .success()
        .stdout(predicate::str::contains("docker.build"))
        .stdout(predicate::str::contains("git.clone"));
}

#[test]
fn task_list_shows_descriptions_for_tasks() {
    cli()
        .assert()
        .success()
        .stdout(predicate::str::contains("Build Docker image"))
        .stdout(predicate::str::contains("Run Docker container"))
        .stdout(predicate::str::contains("Clone a git repository"))
        .stdout(predicate::str::contains("Commit changes"));
}

#[test]
fn task_list_uses_dot_fill_between_name_and_description() {
    // The span_token is '.' by default, used as fill between task name and description
    cli()
        .assert()
        .success()
        .stdout(predicate::str::contains("docker.build ...."))
        .stdout(predicate::str::contains("git.clone ......."));
}

#[test]
fn task_list_tasks_are_indented() {
    // Tasks are indented with 4 spaces (task_indent default)
    cli()
        .assert()
        .success()
        .stdout(predicate::str::contains("    docker.build"));
}

#[test]
fn task_list_is_sorted_alphabetically() {
    let output = cli()
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let docker_pos = stdout.find("docker.build").unwrap();
    let git_pos = stdout.find("git.clone").unwrap();
    assert!(docker_pos < git_pos, "docker tasks should appear before git tasks");
}

#[test]
fn task_list_no_color_codes_when_colors_disabled() {
    // use_colors=false is set in the tasks binary
    let output = cli()
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    // ANSI escape codes start with \x1b[
    assert!(!stdout.contains("\x1b["), "output should not contain ANSI color codes");
}

// --- Task execution ---

#[test]
fn run_git_clone_flat() {
    cli()
        .args(&["git.clone", "https://example.com/repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cloning https://example.com/repo"));
}

#[test]
fn run_git_commit_flat() {
    cli()
        .args(&["git.commit", "fix: bug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Committing with message: fix: bug"));
}

#[test]
fn run_docker_build_flat() {
    cli()
        .args(&["docker.build", "."])
        .assert()
        .success()
        .stdout(predicate::str::contains("Building Docker image from ."));
}

#[test]
fn run_docker_run_flat() {
    cli()
        .args(&["docker.run", "nginx"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running container nginx"));
}

// --- Traditional subcommand syntax ---

#[test]
fn run_git_clone_traditional() {
    cli()
        .args(&["git", "clone", "https://example.com/repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cloning https://example.com/repo"));
}

#[test]
fn run_git_commit_traditional() {
    cli()
        .args(&["git", "commit", "initial"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Committing with message: initial"));
}

// --- Help ---

#[test]
fn help_for_task_with_h_prefix() {
    cli()
        .args(&["h", "git.clone"])
        .assert()
        .success();
}

#[test]
fn help_flag() {
    cli()
        .args(&["--help"])
        .assert()
        .success();
}

// --- Errors ---

#[test]
fn invalid_task_fails() {
    cli()
        .args(&["nonexistent.task"])
        .assert()
        .failure();
}

#[test]
fn version_output() {
    cli()
        .args(&["--version"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1.0.0"));
}
