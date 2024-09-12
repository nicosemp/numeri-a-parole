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
