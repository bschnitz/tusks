mod common;
use assert_cmd::prelude::*;
use predicates::prelude::*;

// =============================================================================
// ROOT LEVEL COMMAND ATTRIBUTE TESTS
// =============================================================================

#[test]
fn test_root_help_shows_long_about() {
    common::cli()
        .args(&["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("A comprehensive command-line interface"));
}

#[test]
fn test_root_help_shows_version() {
    common::cli()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("1.0.0"));
}

// =============================================================================
// TASK COMMAND ATTRIBUTE TESTS
// =============================================================================

#[test]
fn test_task1_shown_in_root_help() {
    // about is shown in the command list
    common::cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("task1"))
        .stdout(predicate::str::contains("First task with parameters"));
}

#[test]
fn test_task1_help_shows_long_about() {
    // long_about is shown in the detailed help
    common::cli()
        .args(&["--root-param", "test", "task1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Demonstrates accessing root parameters"));
}

#[test]
fn test_task2_shown_in_root_help() {
    common::cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("task2"))
        .stdout(predicate::str::contains("Doubles a numeric value"));
}

#[test]
fn test_task3_shown_in_root_help() {
    common::cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("task3"))
        .stdout(predicate::str::contains("Task with no arguments"));
}

#[test]
fn test_task4_shown_in_root_help() {
    common::cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("task4"))
        .stdout(predicate::str::contains("Process a list of items"));
}

#[test]
fn test_task4_help_shows_long_about() {
    common::cli()
        .args(&["--root-param", "test", "task4", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Takes multiple items as input"));
}

// =============================================================================
// MODULE COMMAND ATTRIBUTE TESTS
// =============================================================================

#[test]
fn test_level1_shown_in_root_help() {
    // about shown in root's command list
    common::cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("level1"))
        .stdout(predicate::str::contains("Level 1 submodule"));
}

#[test]
fn test_level1_help_shows_long_about() {
    // long_about shown in level1's detailed help
    common::cli()
        .args(&["--root-param", "test", "level1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("First level of nested commands"));
}

#[test]
fn test_level1b_shown_in_root_help() {
    common::cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("level1b"))
        .stdout(predicate::str::contains("Alternative level 1 module without Parameters"));
}

#[test]
fn test_level2_shown_in_level1_help() {
    // about shown in level1's command list
    common::cli()
        .args(&["--root-param", "test", "level1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("level2"))
        .stdout(predicate::str::contains("Level 2 - deeply nested module"));
}

#[test]
fn test_level3_shown_in_level2_help() {
    common::cli()
        .args(&["--root-param", "test", "level1", "level2", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("level3"))
        .stdout(predicate::str::contains("Maximum depth level"));
}

// =============================================================================
// SUBTASK COMMAND ATTRIBUTE TESTS
// =============================================================================

#[test]
fn test_subtask1_shown_in_level1_help() {
    // about shown in level1's command list
    common::cli()
        .args(&["--root-param", "test", "level1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("subtask1"))
        .stdout(predicate::str::contains("First subtask with parameter chain"));
}

#[test]
fn test_subtask2_shown_in_level1_help() {
    common::cli()
        .args(&["--root-param", "test", "level1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("subtask2"))
        .stdout(predicate::str::contains("Second subtask"));
}

#[test]
fn test_subtask2_help_shows_long_about() {
    // long_about shown in subtask2's detailed help
    common::cli()
        .args(&["--root-param", "test", "level1", "subtask2", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Demonstrates accessing parent parameters through super_"));
}

#[test]
fn test_deep_task_shown_in_level2_help() {
    common::cli()
        .args(&["--root-param", "test", "level1", "level2", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("deep-task"))
        .stdout(predicate::str::contains("Deep nested task"));
}

#[test]
fn test_deep_task_help_shows_long_about() {
    common::cli()
        .args(&["--root-param", "test", "level1", "level2", "--level2-id", "1", "deep-task", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Demonstrates accessing parameters from multiple levels"));
}

#[test]
fn test_very_deep_shown_in_level3_help() {
    common::cli()
        .args(&["--root-param", "test", "level1", "level2", "level3", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("very-deep"))
        .stdout(predicate::str::contains("Task at maximum nesting depth"));
}

// =============================================================================
// EXTERNAL MODULE COMMAND ATTRIBUTE TESTS
// =============================================================================

#[test]
fn test_extroot_shown_in_root_help() {
    // about shown in root's command list
    common::cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("extroot"))
        .stdout(predicate::str::contains("External root module"));
}

#[test]
fn test_extroot_help_shows_long_about() {
    // long_about shown in extroot's detailed help
    common::cli()
        .args(&["--root-param", "test", "extroot", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("External module attached at root level"));
}

#[test]
fn test_ext1_shown_in_level1_help() {
    common::cli()
        .args(&["--root-param", "test", "level1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ext1"))
        .stdout(predicate::str::contains("External module 1"));
}

#[test]
fn test_ext2_shown_in_ext1_help() {
    common::cli()
        .args(&["--root-param", "test", "level1", "ext1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ext2"))
        .stdout(predicate::str::contains("External module 1 - nested under level1"));
}

#[test]
fn test_ext2_help_shows_long_about() {
    common::cli()
        .args(&["--root-param", "test", "level1", "ext1", "ext2", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Demonstrates four-level parameter chain"));
}

// =============================================================================
// EXTERNAL TASK COMMAND ATTRIBUTE TESTS
// =============================================================================

#[test]
fn test_ext_task_shown_in_extroot_help() {
    // about shown in extroot's command list
    common::cli()
        .args(&["--root-param", "test", "extroot", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ext-task"))
        .stdout(predicate::str::contains("Main external root task"));
}

#[test]
fn test_ext_sub_task_shown_in_sub_help() {
    common::cli()
        .args(&["--root-param", "test", "extroot", "--extroot-name", "ext", "sub", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ext-sub-task"))
        .stdout(predicate::str::contains("Task within external submodule"));
}

#[test]
fn test_ext1_task_shown_in_ext1_help() {
    common::cli()
        .args(&["--root-param", "test", "level1", "ext1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ext1-task"))
        .stdout(predicate::str::contains("External1 main task"));
}

#[test]
fn test_ext1_task_help_shows_long_about() {
    common::cli()
        .args(&["--root-param", "test", "level1", "ext1", "ext1-task", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("three-level parameter chain"));
}

#[test]
fn test_ext1_vec_task_shown_in_ext1_help() {
    common::cli()
        .args(&["--root-param", "test", "level1", "ext1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ext1-vec-task"))
        .stdout(predicate::str::contains("Process vector of integers"));
}

#[test]
fn test_ext2_task_shown_in_ext2_help() {
    common::cli()
        .args(&["--root-param", "test", "level1", "ext1", "ext2", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ext2-task"))
        .stdout(predicate::str::contains("Deep external task"));
}

#[test]
fn test_ext2_task_help_shows_long_about() {
    common::cli()
        .args(&["--root-param", "test", "level1", "ext1", "ext2", "--ext2-param", "e2", "ext2-task", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Access parameters from 4 levels"));
}

#[test]
fn test_ext2_complex_shown_in_ext2_help() {
    common::cli()
        .args(&["--root-param", "test", "level1", "ext1", "ext2", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ext2-complex"))
        .stdout(predicate::str::contains("Complex task with multiple argument types"));
}

#[test]
fn test_ext2_complex_help_shows_long_about() {
    common::cli()
        .args(&["--root-param", "test", "level1", "ext1", "ext2", "--ext2-param", "e2", "ext2-complex", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Tests bool flags, Vec parameters"));
}

// =============================================================================
// LEVEL1B TASK COMMAND ATTRIBUTE TESTS
// =============================================================================

#[test]
fn test_task_no_params_shown_in_level1b_help() {
    // about shown in level1b's command list
    common::cli()
        .args(&["--root-param", "test", "level1b", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("task-no-params"))
        .stdout(predicate::str::contains("Task without module Parameters struct"));
}

#[test]
fn test_task_multi_args_shown_in_level1b_help() {
    common::cli()
        .args(&["--root-param", "test", "level1b", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("task-multi-args"))
        .stdout(predicate::str::contains("Task with multiple argument types"));
}

#[test]
fn test_task_multi_args_help_shows_long_about() {
    common::cli()
        .args(&["--root-param", "test", "level1b", "task-multi-args", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Demonstrates various argument types"));
}

// =============================================================================
// COMBINATION TESTS
// =============================================================================

#[test]
fn test_all_subcommands_listed_in_help() {
    common::cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("task0"))
        .stdout(predicate::str::contains("task1"))
        .stdout(predicate::str::contains("task2"))
        .stdout(predicate::str::contains("task3"))
        .stdout(predicate::str::contains("task4"))
        .stdout(predicate::str::contains("level1"))
        .stdout(predicate::str::contains("level1b"))
        .stdout(predicate::str::contains("extroot"));
}

#[test]
fn test_level1_subcommands_listed() {
    common::cli()
        .args(&["--root-param", "test", "level1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("subtask1"))
        .stdout(predicate::str::contains("subtask2"))
        .stdout(predicate::str::contains("level2"))
        .stdout(predicate::str::contains("ext1"));
}

#[test]
fn test_command_attributes_dont_affect_functionality() {
    // Verify that adding #[command] attributes doesn't break existing functionality
    common::cli()
        .args(&["--root-param", "test", "--verbose", "task1", "--arg1", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== root::task1 ==="))
        .stdout(predicate::str::contains("arg1: hello"));
}
