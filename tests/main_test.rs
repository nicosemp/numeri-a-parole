use std::process::Command;

fn run_with_args(args: &[&str]) -> String {
    let output = Command::new("cargo")
        .arg("run")
        .args(args)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn test_zero() {
    let output = run_with_args(&["0"]);
    assert!(output.contains("zero"));
}

#[test]
fn test_one_and_two() {
    let output = run_with_args(&["1", "2"]);
    assert!(output.contains("uno"));
    assert!(output.contains("due"));
}

#[test]
fn test_mille() {
    let output = run_with_args(&["1000"]);
    assert!(output.contains("mille\n"));
}

#[test]
fn test_invalid_min() {
    let output = run_with_args(&["abc"]);
    assert!(output.contains("Error"));
}

#[test]
fn test_min_greater_than_max() {
    let output = run_with_args(&["10", "5"]);
    assert!(output.contains("Error"));
}

#[test]
fn test_milione() {
    let output = run_with_args(&["1000000"]);
    assert!(output.contains("milione"));
}

#[test]
fn test_miliardo() {
    let output = run_with_args(&["1000000000"]);
    assert!(output.contains("miliardo"));
}

#[test]
fn test_longest_output() {
    let output = run_with_args(&["1", "10"]);
    assert!(output.contains("Longest numbers"));
}

#[test]
fn test_longest_tie() {
    // uno (3), due (3), tre (3) all share max length 3 in the range 1..=3
    let output = run_with_args(&["1", "3"]);
    assert!(output.contains("3 found"));
}
