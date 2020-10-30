pub mod checks;

use std::process::exit;

use crate::checks::{
    body::{body, second_section},
    entire::{empty, not_ascii},
    footer::{footer, third_section},
    summary::summary,
};

pub fn check_commit_message(content: &str) {
    let results = check_results(&content);

    output_check_results(&results);

    program_exit(&results);
}

fn output_check_results(checks: &[Result<&str, &str>]) {
    for x in checks {
        match x {
            Ok(pass_message) => println!("[PASS] {}", pass_message),
            Err(fail_message) => eprintln!("[FAIL] {}", fail_message),
        }
    }
}

fn program_exit(checks: &[Result<&str, &str>]) {
    match checks.iter().map(|x| x.is_err()).any(|x| x) {
        true => {
            eprintln!(
                "Error: The commit message failed to pass all the Amaranth commit message checks."
            );
            exit(1);
        }
        false => {
            println!("[Success] The commit message passed all the Amaranth commit message checks.")
        }
    }
}

fn number_of_blank_lines(content: &str) -> usize {
    content.lines().filter(|x| x.is_empty()).count()
}

