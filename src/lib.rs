pub mod checks;

use std::process::exit;

use crate::checks::{
    body::{second_section, Body},
    entire::{empty, not_ascii},
    footer::{third_section, Footer},
    sign_off::SignOff,
    summary::Summary,
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
                "[Error] The commit message does not conform to the Amaranth commit message format."
            );
            exit(1);
        }
        false => {
            println!("[Success] The commit message conforms to the Amaranth commit message format.")
        }
    }
}

fn check_results(content: &str) -> Vec<Result<&str, &str>> {
    let blank_lines = number_of_blank_lines(&content);
    let summary = summary(&content);
    let body = body(&content);
    let footer = footer(&content, blank_lines);

    let summary_and_sign_off: bool = blank_lines == 2;
    let summary_body_and_sign_off: bool = blank_lines == 3
        && !second_section(&content).iter().all(|x| {
            x.starts_with("Resolves:")
                || x.starts_with("See also:")
                || x.starts_with("Signed-off-by:")
        });
    if summary_and_sign_off {
        vec![
            // entire commit message
            empty(&content),
            not_ascii(&content),
            // summary checks
            summary.category_abbreviation(),
            summary.first_word_lowercase(),
            summary.first_word_imperative_mood(),
            summary.period(),
            summary.over_max_length(),
            summary.first_word_lowercase(),
        ]
    } else if summary_body_and_sign_off {
        vec![
            // entire commit message
            empty(&content),
            not_ascii(&content),
            // summary checks
            summary.category_abbreviation(),
            summary.first_word_lowercase(),
            summary.first_word_imperative_mood(),
            summary.period(),
            summary.over_max_length(),
            summary.first_word_lowercase(),
            // body checks
            body.max_length(),
            body.footer_lines(),
        ]
    } else if blank_lines == 2
        && second_section(&content)
            .iter()
            .all(|x| x.starts_with("Resolves:") || x.starts_with("See also:"))
    {
        vec![
            // entire commit message
            empty(&content),
            not_ascii(&content),
            // summary checks
            summary.category_abbreviation(),
            summary.first_word_lowercase(),
            summary.first_word_imperative_mood(),
            summary.period(),
            summary.over_max_length(),
            summary.first_word_lowercase(),
            // footer checks
            footer.max_length(),
            footer.all_footer_lines(),
        ]
    } else if blank_lines == 3
        && !second_section(&content)
            .iter()
            .all(|x| x.starts_with("Resolves:") || x.starts_with("See also:"))
        && third_section(&content)
            .iter()
            .all(|x| x.starts_with("Resolves:") || x.starts_with("See also:"))
    {
        vec![
            // entire commit message
            empty(&content),
            not_ascii(&content),
            // summary checks
            summary.category_abbreviation(),
            summary.first_word_lowercase(),
            summary.first_word_imperative_mood(),
            summary.period(),
            summary.over_max_length(),
            summary.first_word_lowercase(),
            // body checks
            body.max_length(),
            body.footer_lines(),
            // footer checks
            footer.max_length(),
            footer.all_footer_lines(),
        ]
    } else {
        vec![Err("Invalid commit message structure.")]
    }
}
