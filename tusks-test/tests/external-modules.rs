use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("external-modules").unwrap()
}

#[test]
fn test_root_status() {
    cli()
        .args(&["status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status check"));
}

#[test]
fn test_root_status_with_params() {
    cli()
        .args(&["--verbose", "--workdir", "/tmp", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: enabled"))
        .stdout(predicate::str::contains("Working directory: /tmp"));
}

// Git module tests
#[test]
fn test_git_clone() {
    cli()
        .args(&["git", "clone", "https://github.com/user/repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cloning: https://github.com/user/repo"));
}

#[test]
fn test_git_clone_with_target() {
    cli()
        .args(&["git", "clone", "https://github.com/user/repo", "myrepo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Target: myrepo"));
}

#[test]
fn test_git_clone_with_repo_param() {
    cli()
        .args(&["git", "--repo", "/home/user/repos", "clone", "https://github.com/user/repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Repo path: /home/user/repos"));
}

#[test]
fn test_git_clone_with_root_verbose() {
    cli()
        .args(&["--verbose", "git", "clone", "https://github.com/user/repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: detailed clone output"));
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
fn test_git_commit_verbose() {
    cli()
        .args(&["--verbose", "git", "commit", "Fix bug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: showing commit details"));
}

#[test]
fn test_git_push_defaults() {
    cli()
        .args(&["git", "push"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Pushing to origin/main"));
}

#[test]
fn test_git_push_custom() {
    cli()
        .args(&["git", "push", "--remote", "upstream", "--branch", "develop"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Pushing to upstream/develop"));
}

#[test]
fn test_git_advanced_rebase() {
    cli()
        .args(&["git", "advanced", "rebase", "main"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rebasing onto: main"));
}

#[test]
fn test_git_advanced_cherry_pick() {
    cli()
        .args(&["git", "advanced", "cherry-pick", "abc123"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cherry-picking: abc123"));
}

// Docker module tests
#[test]
fn test_docker_build() {
    cli()
        .args(&["docker", "build", "."])
        .assert()
        .success()
        .stdout(predicate::str::contains("Building image from: ."));
}

#[test]
fn test_docker_build_with_tag() {
    cli()
        .args(&["docker", "build", ".", "-t", "myapp:latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Tag: myapp:latest"));
}

#[test]
fn test_docker_build_with_host() {
    cli()
        .args(&["docker", "--host", "tcp://localhost:2375", "build", "."])
        .assert()
        .success()
        .stdout(predicate::str::contains("Docker host: tcp://localhost:2375"));
}

#[test]
fn test_docker_build_verbose() {
    cli()
        .args(&["--verbose", "docker", "build", "."])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: detailed build output"));
}

#[test]
fn test_docker_run() {
    cli()
        .args(&["docker", "run", "nginx"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running image: nginx"));
}

#[test]
fn test_docker_run_with_name() {
    cli()
        .args(&["docker", "run", "nginx", "-n", "my-nginx"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Container name: my-nginx"));
}

#[test]
fn test_docker_container_list() {
    cli()
        .args(&["docker", "container", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Listing running containers"));
}

#[test]
fn test_docker_container_list_all() {
    cli()
        .args(&["docker", "container", "list", "-a"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Listing all containers"));
}

#[test]
fn test_docker_container_stop() {
    cli()
        .args(&["docker", "container", "stop", "my-container"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Stopping container: my-container"));
}

#[test]
fn test_docker_image_list() {
    cli()
        .args(&["docker", "image", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Listing images"));
}

#[test]
fn test_docker_image_remove() {
    cli()
        .args(&["docker", "image", "remove", "myimage:latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removing image: myimage:latest"));
}

#[test]
fn test_docker_image_remove_force() {
    cli()
        .args(&["docker", "image", "remove", "myimage:latest", "-f"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Force removing image: myimage:latest"));
}

// Deploy module tests (with custom name "deploy")
#[test]
fn test_deploy_start() {
    cli()
        .args(&["deploy", "--environment", "production", "start", "v1.0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deploying version: v1.0.0"))
        .stdout(predicate::str::contains("Environment: production"));
}

#[test]
fn test_deploy_start_verbose() {
    cli()
        .args(&["--verbose", "deploy", "--environment", "staging", "start", "v2.0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: detailed deployment"));
}

#[test]
fn test_deploy_start_with_workdir() {
    cli()
        .args(&["--workdir", "/app", "deploy", "--environment", "production", "start", "v1.0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Working directory: /app"));
}

#[test]
fn test_deploy_rollback() {
    cli()
        .args(&["deploy", "--environment", "production", "rollback"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rolling back 1 version(s)"))
        .stdout(predicate::str::contains("Environment: production"));
}

#[test]
fn test_deploy_rollback_steps() {
    cli()
        .args(&["deploy", "--environment", "staging", "rollback", "--steps", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rolling back 3 version(s)"));
}

#[test]
fn test_deploy_status() {
    cli()
        .args(&["deploy", "--environment", "production", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deployment status for: production"));
}

// Local build module test
#[test]
fn test_build_compile() {
    cli()
        .args(&["build", "compile"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Compiling for target: \"default\""));
}

#[test]
fn test_build_compile_target() {
    cli()
        .args(&["build", "compile", "--target", "x86_64-linux"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Compiling for target: \"x86_64-linux\""));
}

#[test]
fn test_build_clean() {
    cli()
        .args(&["build", "clean"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleaning build artifacts"));
}

// Help tests
#[test]
fn test_help_shows_all_modules() {
    cli()
        .args(&["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("git"))
        .stdout(predicate::str::contains("docker"))
        .stdout(predicate::str::contains("deploy"))
        .stdout(predicate::str::contains("build"));
}

#[test]
fn test_git_help() {
    cli()
        .args(&["git", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Git version control"))
        .stdout(predicate::str::contains("clone"))
        .stdout(predicate::str::contains("commit"))
        .stdout(predicate::str::contains("push"));
}

#[test]
fn test_deploy_missing_required_param() {
    cli()
        .args(&["deploy", "start", "v1.0.0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"))
        .stderr(predicate::str::contains("--environment"));
}
