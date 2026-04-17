use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("arg").unwrap()
}

#[test]
fn test_types() {
    cli()
        .args(&["types", "--flag", "hello", "42", "--", "-10", "3.14"])
        .assert()
        .success()
        .stdout(predicate::str::contains("String: hello"))
        .stdout(predicate::str::contains("Number (u32): 42"))
        .stdout(predicate::str::contains("Signed (i32): -10"))
        .stdout(predicate::str::contains("Float (f64): 3.14"))
        .stdout(predicate::str::contains("Flag (bool): true"));
}

#[test]
fn test_flags_short() {
    cli()
        .args(&["flags", "-v", "-q", "-d"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: true"))
        .stdout(predicate::str::contains("Quiet: true"))
        .stdout(predicate::str::contains("Debug: true"));
}

#[test]
fn test_flags_long() {
    cli()
        .args(&["flags", "--verbose", "--quiet", "--debug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: true"))
        .stdout(predicate::str::contains("Quiet: true"))
        .stdout(predicate::str::contains("Debug: true"));
}

#[test]
fn test_optional_with_values() {
    cli()
        .args(&["optional", "required", "--opt", "optional_val", "--number", "42"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Required: required"))
        .stdout(predicate::str::contains("Optional: Some(\"optional_val\")"))
        .stdout(predicate::str::contains("Number: Some(42)"));
}

#[test]
fn test_optional_without_values() {
    cli()
        .args(&["optional", "required"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Required: required"))
        .stdout(predicate::str::contains("Optional: None"))
        .stdout(predicate::str::contains("Number: None"));
}

#[test]
fn test_defaults_no_args() {
    cli()
        .args(&["defaults"])
        .assert()
        .success()
        .stdout(predicate::str::contains("File: default.txt"))
        .stdout(predicate::str::contains("Count: 10"))
        .stdout(predicate::str::contains("Level: info"));
}

#[test]
fn test_defaults_with_overrides() {
    cli()
        .args(&["defaults", "--file", "custom.txt", "--count", "5", "--level", "debug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("File: custom.txt"))
        .stdout(predicate::str::contains("Count: 5"))
        .stdout(predicate::str::contains("Level: debug"));
}

#[test]
fn test_multiple_values() {
    cli()
        .args(&["multiple", "--files", "a.txt", "b.txt", "--tags", "one,two,three", "--items", "x,y,z"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Files: [\"a.txt\", \"b.txt\"]"))
        .stdout(predicate::str::contains("Tags: [\"one\", \"two\", \"three\"]"))
        .stdout(predicate::str::contains("Items: [\"x,y,z\"]"));
}

#[test]
fn test_naming() {
    cli()
        .args(&["naming", "input.txt", "--output", "out.txt", "--port", "8080"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Input: "))
        .stdout(predicate::str::contains("input.txt"))
        .stdout(predicate::str::contains("Output: "))
        .stdout(predicate::str::contains("out.txt"))
        .stdout(predicate::str::contains("Port: 8080"));
}

#[test]
fn test_parser_valid() {
    cli()
        .args(&["parser", "--percentage", "75", "--port", "3000"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Percentage: 75%"))
        .stdout(predicate::str::contains("Port: 3000"));
}

#[test]
fn test_parser_invalid_percentage() {
    cli()
        .args(&["parser", "--percentage", "150", "--port", "3000"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Percentage must be between 0 and 100"));
}

#[test]
fn test_parser_invalid_port() {
    cli()
        .args(&["parser", "--percentage", "50", "--port", "80"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Port must be 1024 or higher"));
}

#[test]
fn test_conflicts_verbose() {
    cli()
        .args(&["conflicts", "--verbose"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: true"))
        .stdout(predicate::str::contains("Quiet: false"));
}

#[test]
fn test_conflicts_quiet() {
    cli()
        .args(&["conflicts", "--quiet"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: false"))
        .stdout(predicate::str::contains("Quiet: true"));
}

#[test]
fn test_conflicts_both_fails() {
    cli()
        .args(&["conflicts", "--verbose", "--quiet"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_conflicts_requires() {
    cli()
        .args(&["conflicts", "--feature", "--config", "test.conf"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Feature: true"))
        .stdout(predicate::str::contains("Config: Some(\"test.conf\")"));
}

#[test]
fn test_conflicts_requires_missing() {
    cli()
        .args(&["conflicts", "--feature"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"));
}

#[test]
fn test_hidden_visible() {
    cli()
        .args(&["hidden", "test", "--normal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Visible: test"))
        .stdout(predicate::str::contains("Normal: true"));
}

#[test]
fn test_hidden_secret_flag() {
    cli()
        .args(&["hidden", "test", "--debug-secret", "--normal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Debug secret: true"));
}

#[test]
fn test_hidden_help_output() {
    cli()
        .args(&["hidden", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Visible argument"))
        .stdout(predicate::str::contains("Normal flag"))
        .stdout(predicate::str::contains("Hidden debug flag").not());
}

#[test]
fn test_actions_no_verbose() {
    cli()
        .args(&["actions", "test"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbosity: normal (level 0)"))
        .stdout(predicate::str::contains("Message: test"));
}

#[test]
fn test_actions_single_verbose() {
    cli()
        .args(&["actions", "-v", "test"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbosity: verbose (level 1)"));
}

#[test]
fn test_actions_triple_verbose() {
    cli()
        .args(&["actions", "-vvv", "test"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbosity: debug (level 3)"));
}

#[test]
fn test_positional_required() {
    cli()
        .args(&["positional", "first", "second"])
        .assert()
        .success()
        .stdout(predicate::str::contains("First: first"))
        .stdout(predicate::str::contains("Second: second"))
        .stdout(predicate::str::contains("Named: None"))
        .stdout(predicate::str::contains("Third: None"));
}

#[test]
fn test_positional_with_named() {
    cli()
        .args(&["positional", "first", "second", "--named", "test", "third"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Named: Some(\"test\")"))
        .stdout(predicate::str::contains("Third: Some(\"third\")"));
}

#[test]
fn test_ranges_valid() {
    cli()
        .args(&["ranges", "--small=50", "--signed=-25"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Small: 50"))
        .stdout(predicate::str::contains("Signed: -25"));
}

#[test]
fn test_ranges_small_too_large() {
    cli()
        .args(&["ranges", "--small=101", "--signed=0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("101 is not in 1..=100"));
}

#[test]
fn test_ranges_signed_out_of_range() {
    cli()
        .args(&["ranges", "--small=50", "--signed=60"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("60 is not in -50..=50"));
}

#[test]
fn test_help_output() {
    cli()
        .args(&["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test all #[arg] attribute features"))
        .stdout(predicate::str::contains("types"))
        .stdout(predicate::str::contains("flags"))
        .stdout(predicate::str::contains("optional"));
}

#[test]
fn test_version_output() {
    cli()
        .args(&["--version"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1.0.0"));
}

// --- Edge case tests ---

#[test]
fn test_types_empty_string() {
    cli()
        .args(&["types", "", "1", "0", "0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("String: \n"));
}

#[test]
fn test_types_unicode_string() {
    cli()
        .args(&["types", "日本語テスト", "42", "0", "1.5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("String: 日本語テスト"));
}

#[test]
fn test_types_u32_zero() {
    cli()
        .args(&["types", "test", "0", "0", "0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Number (u32): 0"));
}

#[test]
fn test_types_u32_max() {
    cli()
        .args(&["types", "test", "4294967295", "0", "0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Number (u32): 4294967295"));
}

#[test]
fn test_types_u32_overflow() {
    cli()
        .args(&["types", "test", "4294967296", "0", "0.0"])
        .assert()
        .failure();
}

#[test]
fn test_types_negative_for_unsigned() {
    cli()
        .args(&["types", "test", "-1", "0", "0.0"])
        .assert()
        .failure();
}

#[test]
fn test_types_i32_negative() {
    cli()
        .args(&["types", "--flag", "test", "1", "--", "-2147483648", "0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Signed (i32): -2147483648"));
}

#[test]
fn test_types_float_negative() {
    cli()
        .args(&["types", "--flag", "test", "1", "--", "-1", "-3.14"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Float (f64): -3.14"));
}

#[test]
fn test_optional_empty_string_value() {
    cli()
        .args(&["optional", "req", "--opt", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("Optional: Some(\"\")"));
}

#[test]
fn test_defaults_override_with_empty() {
    cli()
        .args(&["defaults", "--file", "", "--level", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("File: \n"))
        .stdout(predicate::str::contains("Level: \n"));
}

#[test]
fn test_multiple_empty_files_list() {
    cli()
        .args(&["multiple", "--files", "single"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Files: [\"single\"]"));
}

#[test]
fn test_ranges_boundary_min() {
    cli()
        .args(&["ranges", "--small=1", "--signed=-50"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Small: 1"))
        .stdout(predicate::str::contains("Signed: -50"));
}

#[test]
fn test_ranges_boundary_max() {
    cli()
        .args(&["ranges", "--small=100", "--signed=50"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Small: 100"))
        .stdout(predicate::str::contains("Signed: 50"));
}

#[test]
fn test_ranges_below_min() {
    cli()
        .args(&["ranges", "--small=0", "--signed=0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("0 is not in 1..=100"));
}

#[test]
fn test_unknown_subcommand() {
    cli()
        .args(&["nonexistent"])
        .assert()
        .failure();
}

#[test]
fn test_parser_percentage_zero() {
    cli()
        .args(&["parser", "--percentage", "0", "--port", "1024"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Percentage: 0%"));
}

#[test]
fn test_parser_percentage_max() {
    cli()
        .args(&["parser", "--percentage", "100", "--port", "1024"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Percentage: 100%"));
}

#[test]
fn test_parser_port_boundary() {
    cli()
        .args(&["parser", "--percentage", "50", "--port", "1024"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Port: 1024"));
}

#[test]
fn test_parser_port_below_boundary() {
    cli()
        .args(&["parser", "--percentage", "50", "--port", "1023"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Port must be 1024 or higher"));
}

#[test]
fn test_positional_missing_required() {
    cli()
        .args(&["positional", "first"])
        .assert()
        .failure();
}

#[test]
fn test_flags_no_flags_all_false() {
    cli()
        .args(&["flags"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose: false"))
        .stdout(predicate::str::contains("Quiet: false"))
        .stdout(predicate::str::contains("Debug: false"));
}

#[test]
fn test_types_string_with_spaces() {
    cli()
        .args(&["types", "hello world", "1", "0", "0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("String: hello world"));
}
