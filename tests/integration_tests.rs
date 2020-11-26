use std::eprintln;
use std::process::{exit, Command};
use std::str::from_utf8;

fn command_output(file: &str) -> String {
    let binary = "target/debug/amaranth_commit_msg_hook";

    let output = Command::new(binary)
        .arg(file)
        .output()
        .expect("failed to execute process");

    match from_utf8(output.stderr.as_slice()) {
        Ok(s) => s.to_string(),
        Err(e) => {
            eprintln!("Invalid UTF-8 sequence: {}", e);
            exit(1);
        }
    }
}

/////////////////
// ascii check //
/////////////////

#[test]
fn ascii() {
    let file = "tests/expected-output/ascii.txt";
    let expected_output: &str = "[FAIL] Commit message is not ASCII.\n\
         [Error] The commit message does not conform to the Amaranth commit message format.\n";

    assert_eq!(command_output(file), expected_output);
}

////////////////////
// summary checks //
////////////////////

#[test]
fn category_abbreviation_only() {
    let file = "tests/expected-output/category-abbreviation-only.txt";
    let expected_output: &str =
        "[FAIL] Commit message summary line must to be more than just a category abbreviation.\n\
         [Error] The commit message does not conform to the Amaranth commit message format.\n";

    assert_eq!(command_output(file), expected_output);
}

#[test]
fn summary_length() {
    let file = "tests/expected-output/summary-length.txt";
    let expected_output: &str = "[FAIL] Summary line is more than 50 characters.\n\
         [Error] The commit message does not conform to the Amaranth commit message format.\n";

    assert_eq!(command_output(file), expected_output);
}

#[test]
fn ends_with_period() {
    let file = "tests/expected-output/end-with-period.txt";
    let expected_output: &str = "[FAIL] Summary line end in a period.\n\
         [Error] The commit message does not conform to the Amaranth commit message format.\n";

    assert_eq!(command_output(file), expected_output);
}
