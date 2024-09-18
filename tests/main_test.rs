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
fn test_one() {
    let output = run_with_args(&["1"]);
    assert!(output.contains("uno"));
}

#[test]
fn test_mille() {
    let output = run_with_args(&["1000"]);
    assert!(output.contains("mille\n"));
}

#[test]
fn test_one_million() {
    let output = run_with_args(&["1000000"]);
    assert!(output.contains("un milione"));
}

#[test]
fn test_complex_big_number() {
    let output = run_with_args(&["123456789"]);
    assert!(
        output.contains("centoventitre milioni quattrocentocinquantaseimilasettecentottantanove")
    );
}
