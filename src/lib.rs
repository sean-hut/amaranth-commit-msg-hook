pub mod body_checks;
pub mod entire_commit_checks;
pub mod summary_checks;

use std::process::exit;

use crate::entire_commit_checks::{empty, not_ascii};

use crate::summary_checks::{
    first_summary_word_not_imperative_mood, first_summary_word_not_lowercase,
    invalid_category_abbreviation, summary_ends_with_period, summary_over_50_characters,
};

use crate::body_checks::lines_over_max_length;

pub fn check_commit_message(content: &String) {
    output_check_results(&check_results(&content));

    program_exit(&check_results(&content));
}

fn check_results(content: &String) -> Vec<Result<String, String>> {
    vec![
        // entire commit message
        empty(&content),
        not_ascii(&content),
        // summary checks
        invalid_category_abbreviation(&content),
        summary_ends_with_period(&content),
        summary_over_50_characters(&content),
        first_summary_word_not_lowercase(&content),
        first_summary_word_not_imperative_mood(&content),
        // body checks
        lines_over_max_length(&content),
    ]
}

fn output_check_results(checks: &Vec<Result<String, String>>) {
    for x in checks {
        match x {
            Ok(pass_message) => println!("[PASS] {}", pass_message),
            Err(fail_message) => eprintln!("[FAIL] {}", fail_message),
        }
    }
}

fn any_failed_checks(checks: &Vec<Result<String, String>>) -> bool {
    checks.into_iter().map(|x| x.is_err()).any(|x| x == true)
}

fn program_exit(checks: &Vec<Result<String, String>>) {
    match any_failed_checks(&checks) {
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
